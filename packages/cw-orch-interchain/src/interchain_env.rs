//! Interactions with docker using bollard

use crate::daemon::networks::parse_network;
use crate::daemon::Daemon;
use crate::daemon::DaemonError;
use crate::interface_traits::ContractInstance;
use crate::state::ChainState;

use crate::packet_inspector::PacketInspector;
use ibc_relayer_types::core::ics24_host::identifier::PortId;

use crate::IcResult;
use ibc_chain_registry::chain::{ChainData, Grpc};
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::runtime::ConfigBuilder;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

use log4rs::Config;

use std::collections::HashMap;
use std::default::Default;
use std::path::PathBuf;
use tokio::runtime::Handle;

use crate::InterchainError;


pub type NetworkId = String;
pub type Mnemonic = String;

/// Represents a set of locally running blockchain nodes and a Hermes relayer.
pub struct InterchainEnv {
    /// Daemons indexable by network id, i.e. "juno-1", "osmosis-2", ...
    chain_config: HashMap<NetworkId, ChainData>,
    daemons: HashMap<NetworkId, Daemon>,

    log: InterchainLog,
    runtime: Handle,
}

impl InterchainEnv {
    /// Builds a new `InterchainEnv` instance.
    pub fn new<T>(runtime: &Handle, chains: Vec<(T, Option<impl ToString>)>) -> IcResult<Self>
    where
        T: Into<ChainData>,
    {
        let (chains, mnemonics): (Vec<ChainData>, _) = chains
            .into_iter()
            .map(|(chain, mnemonic)| {
                let chain_data = chain.into();
                (
                    chain_data.clone(),
                    (chain_data.chain_id, mnemonic.map(|mn| mn.to_string())),
                )
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let mut obj = Self {
            chain_config: HashMap::new(),
            daemons: HashMap::new(),

            log: InterchainLog::new(),
            runtime: runtime.clone(),
        };
        // First we register the chain_data
        obj.add_chain_config(chains)?;
        // Then the mnemonics
        obj.add_mnemonics(
            mnemonics
                .into_iter()
                .filter(|(_, mn)| mn.is_some())
                .map(|(chain_id, mn)| (chain_id, mn.unwrap()))
                .collect(),
        )?;

        Ok(obj)
    }

    /// Registers a custom chain to the current interchain environment
    pub fn add_chain_config<T>(&mut self, chain_configs: Vec<T>) -> IcResult<()>
    where
        T: Into<ChainData>,
    {
        let chain_data_configs: Vec<ChainData> =
            chain_configs.into_iter().map(|c| c.into()).collect();
        // We create logs for the new chains that were just added
        self.log.add_chains(
            &chain_data_configs
                .iter()
                .map(|chain| chain.chain_id.to_string())
                .collect(),
        );

        // We can't update the chain config while running. It's supposed to be created at the beginning of execution
        for chain_data in chain_data_configs {
            let chain_id = chain_data.chain_id.to_string();
            if self.chain_config.contains_key(&chain_id) {
                return Err(InterchainError::AlreadyRegistered(chain_id));
            }
            self.chain_config.insert(chain_id, chain_data);
        }
        Ok(())
    }

    /// Adds a mnemonic to the current configuration for a specific chain.
    /// This requires that the chain Ids are already registered in the object using the add_chain_config function ?
    /// TODO allow registering mnemonics for chains accessible using the parse_network function
    pub fn add_mnemonics(
        &mut self,
        mnemonics: Vec<(impl ToString, impl ToString)>,
    ) -> IcResult<&mut Self> {
        // We can't update the chain config while running. It's supposed to be created at the beginning of execution
        for (chain_id, mn) in mnemonics {
            let chain_id = ChainId::from_string(chain_id.to_string().as_str());
            if self.daemons.contains_key(&chain_id.to_string()) {
                return Err(InterchainError::AlreadyRegistered(chain_id.to_string()));
            }
            let chain_data = self.chain_data(chain_id)?;
            self.build_daemon(chain_data, mn)?;
        }

        Ok(self)
    }

    /// Build a daemon from chain data and mnemonic and add it to the current configuration
    pub fn build_daemon(&mut self, chain_data: ChainData, mnemonic: impl ToString) -> IcResult<()> {
        let daemon = Daemon::builder()
            .chain(chain_data.clone())
            .deployment_id("interchain") // TODO, how do we choose that
            .handle(&self.runtime)
            .mnemonic(mnemonic)
            .build()
            .unwrap();

        self.daemons.insert(chain_data.chain_id.to_string(), daemon);

        Ok(())
    }

    /// Add already constructed daemons to the environment
    pub fn add_daemons(&mut self, daemons: &[Daemon]) -> IcResult<()> {
        // First we add the chain data to our configuration
        self.add_chain_config(
            daemons
                .iter()
                .map(|d| d.state().chain_data.clone())
                .collect(),
        )?;
        // Then we add the daemons

        Ok(())
    }

    /// Get the daemon for a network-id in the interchain.
    pub fn daemon(&self, chain_id: impl ToString) -> Result<Daemon, InterchainError> {
        self.daemons
            .get(&chain_id.to_string())
            .ok_or(InterchainError::DaemonNotFound(chain_id.to_string()))
            .cloned()
    }

    /// Get the chain data for a network-id in the interchain.
    /// If the chain data is not registered in the environment, it fetches the configuration from the ibc_interchain_registry
    pub fn chain_data(&self, chain_id: impl ToString) -> Result<ChainData, InterchainError> {
        self.chain_config
            .get(&chain_id.to_string())
            .cloned()
            .or_else(|| {
                parse_network(chain_id.to_string().as_str()).ok().map(|i| {
                    let chain_data: ChainData = i.into();
                    chain_data
                })
            })
            .ok_or(InterchainError::ChainConfigNotFound(chain_id.to_string()))
    }

    /// Get the gRPC ports for the local daemons and set them in the `ChainData` objects.
    pub async fn configure_networks(networks: &mut [ChainData]) -> IcResult<()> {
        let docker_helper = DockerHelper::new().await?;

        // use chain data network name as to filter container ids
        let containers_grpc_port = docker_helper.grpc_ports().await?;

        // update network with correct grpc port
        networks.iter_mut().for_each(|network| {
            for container in &containers_grpc_port {
                if container.0.contains(&network.chain_name) {
                    network.apis.grpc = vec![Grpc {
                        address: format!("http://0.0.0.0:{}", container.1),
                        ..Default::default()
                    }];
                    log::info!(
                        "Connected to chain {} on port {}",
                        network.chain_name,
                        container.1
                    );
                }
            }
        });
        Ok(())
    }

    /// Blocks until all the IBC packets sent during the transaction on chain `chain_id` with transaction hash `packet_send_tx_hash` have completed their cycle
    /// (Packet Sent, Packet Received, Packet Acknowledgment)
    /// This also follows additional packets sent out in the resulting transactions
    /// See the documentation for `PacketInspector::await_ibc_execution` for more details about the awaiting procedure
    pub async fn await_ibc_execution(
        &self,
        chain_id: NetworkId,
        packet_send_tx_hash: String,
    ) -> Result<(), DaemonError> {
        // We crate an interchain env object that is safe to send between threads
        let interchain_env =
            PacketInspector::new(&self.chain_config.values().cloned().collect()).await?;

        // We follow the trail
        interchain_env
            .await_ibc_execution(chain_id, packet_send_tx_hash)
            .await?;

        Ok(())
    }
}

pub struct InterchainLog {
    handle: log4rs::Handle,
    chain_ids: Vec<String>,
}

impl InterchainLog {
    fn get_encoder() -> Box<PatternEncoder> {
        Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n}",
        ))
    }

    fn builder() -> ConfigBuilder {
        let encoder = InterchainLog::get_encoder();
        let main_log_path = generate_log_file_path("main");
        std::fs::create_dir_all(main_log_path.parent().unwrap()).unwrap();

        let main_appender = FileAppender::builder()
            .encoder(encoder)
            .build(&main_log_path)
            .unwrap();
        Config::builder().appender(Appender::builder().build("main", Box::new(main_appender)))
    }

    fn build_logger(config: ConfigBuilder) -> log4rs::Config {
        config
            .build(Root::builder().appender("main").build(LevelFilter::Info))
            .unwrap()
    }

    fn add_logger(&self, config: ConfigBuilder, chain_id: String) -> ConfigBuilder {
        // We create the log file and register in the log config
        let log_path = generate_log_file_path(&chain_id);
        let daemon_appender = FileAppender::builder()
            .encoder(InterchainLog::get_encoder())
            .build(log_path)
            .unwrap();

        config
            .appender(Appender::builder().build(&chain_id, Box::new(daemon_appender)))
            .logger(
                Logger::builder()
                    .appender(&chain_id)
                    .build(&chain_id, LevelFilter::Info),
            )
    }

    /// Initiates an interchain log setup
    /// This will log the different chain interactions and updates on separate files for each chain.
    /// This is useful for tracking operations happenning on IBC chains

    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_chains(&mut self, chain_ids: &Vec<String>) {
        // We restart the config with the older builders
        let mut config_builder = InterchainLog::builder();
        for chain_id in &self.chain_ids {
            config_builder = self.add_logger(config_builder, chain_id.to_string());
        }

        // And then we add the new builders
        for chain_id in chain_ids {
            // We verify the log setup is not already created for the chain id
            // We silently continue if we already have a log setup for the daemon
            if self.chain_ids.contains(chain_id) {
                continue;
            }
            self.chain_ids.push(chain_id.clone());
            config_builder = self.add_logger(config_builder, chain_id.clone());
            // log startup to each daemon log
            log::info!("Starting specific log: {chain_id}");
        }
        self.handle
            .set_config(InterchainLog::build_logger(config_builder));
    }
}

impl Default for InterchainLog {
    fn default() -> Self {
        // ensure dir exists
        // add main appender to config
        let config_builder = InterchainLog::builder();
        let config = InterchainLog::build_logger(config_builder);

        let handle = log4rs::init_config(config).unwrap();

        Self {
            handle,
            chain_ids: vec![],
        }
    }
}

/// Get the file path for the log target
fn generate_log_file_path(file: &str) -> PathBuf {
    let file_name = format!("{}.log", file);

    let mut log_path = std::env::current_dir().unwrap();
    log_path.push("logs");
    log_path.push(file_name);

    log_path
}

/// format the port for a contract
pub fn contract_port(contract: &dyn ContractInstance<Daemon>) -> PortId {
    format!("wasm.{}", contract.addr_str().unwrap())
        .parse()
        .unwrap()
}
