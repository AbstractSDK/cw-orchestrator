//! Interactions with docker using bollard

use ibc_chain_registry::chain::{ChainData, Grpc};
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use tokio::time::sleep;
use tokio::time::Duration;

use log4rs::Config;
use tonic::transport::Channel;

use std::collections::HashMap;
use std::default::Default;
use std::path::PathBuf;
use tokio::runtime::Handle;

use super::error::InterchainError;

use crate::CosmTxResponse;
use crate::{ContractInstance, Daemon, DaemonError};

use super::docker::DockerHelper;
use super::hermes::Hermes;
use super::IcResult;
use crate::daemon::queriers::DaemonQuerier;
use crate::follow_ibc_execution::follow_trail;
use crate::queriers::Node;
use crate::state::ChainState;

pub type ContainerId = String;
pub type Port = String;
pub type NetworkId = String;
pub type Mnemonic = String;

/// Represents a set of locally running blockchain nodes and a Hermes relayer.
pub struct InterchainInfrastructure {
    /// Daemons indexable by network id, i.e. "juno-1", "osmosis-2", ...
    daemons: HashMap<NetworkId, Daemon>,
    pub hermes: Hermes,
}

impl InterchainInfrastructure {
    /// Builds a new `InterchainInfrastructure` instance.
    pub fn new<T>(runtime: &Handle, chains: Vec<(T, &str)>) -> IcResult<Self>
    where
        T: Into<ChainData>,
    {
        let (mut chains, mnemonics): (Vec<ChainData>, _) = chains
            .into_iter()
            .map(|(chain, mnemonic)| (chain.into(), mnemonic.to_string()))
            .unzip::<_, _, Vec<_>, Vec<_>>();
        // Start update gRPC ports with local daemons
        runtime.block_on(Self::configure_networks(&mut chains))?;

        let daemons = Self::build_daemons(
            runtime,
            // combine the chain with its mnemonic
            &chains.into_iter().zip(mnemonics).collect::<Vec<_>>(),
        )?;
        let hermes = runtime.block_on(Self::get_hermes())?;

        // set up logging for the chains

        let encoder = Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n}",
        ));
        let main_log_path = generate_log_file_path("main");
        let main_appender = FileAppender::builder()
            .encoder(encoder.clone())
            .build(&main_log_path)
            .unwrap();
        // ensure dir exists
        std::fs::create_dir_all(main_log_path.parent().unwrap()).unwrap();
        // add main appender to config
        let mut config =
            Config::builder().appender(Appender::builder().build("main", Box::new(main_appender)));

        // add appender for each daemon
        for daemon in daemons.values() {
            let chain_id = daemon.state().chain_id.clone();
            let log_path = generate_log_file_path(&chain_id);
            let daemon_appender = FileAppender::builder()
                .encoder(encoder.clone())
                .build(&log_path)
                .unwrap();

            config = config
                .appender(Appender::builder().build(&chain_id, Box::new(daemon_appender)))
                .logger(
                    Logger::builder()
                        .appender(&chain_id)
                        .build(&chain_id, LevelFilter::Info),
                );
        }

        let config = config
            .build(Root::builder().appender("main").build(LevelFilter::Info))
            .unwrap();

        log4rs::init_config(config).unwrap();

        for daemon in daemons.values() {
            let log_target = &daemon.state().chain_id;
            // log startup to each daemon log
            log::info!(target: log_target, "Starting daemon {log_target}");
        }

        Ok(Self { daemons, hermes })
    }

    /// Get the daemon for a network-id in the interchain.
    pub fn daemon(&self, chain_id: impl ToString) -> Result<Daemon, InterchainError> {
        self.daemons
            .get(&chain_id.to_string())
            .ok_or(InterchainError::DaemonNotFound(chain_id.to_string()))
            .cloned()
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

    async fn get_hermes() -> IcResult<Hermes> {
        let docker_helper = DockerHelper::new().await?;
        docker_helper.get_hermes()
    }

    /// Build the daemons from the shared runtime and chain data
    fn build_daemons(
        runtime_handle: &Handle,
        chain_data: &[(ChainData, Mnemonic)],
    ) -> Result<HashMap<NetworkId, Daemon>, DaemonError> {
        let mut daemons = HashMap::new();
        for (chain, mnemonic) in chain_data {
            let daemon = Daemon::builder()
                .chain(chain.clone())
                .deployment_id("interchain")
                .handle(runtime_handle)
                .mnemonic(mnemonic)
                .build()
                .unwrap();

            daemons.insert(chain.chain_id.to_string(), daemon);
        }
        Ok(daemons)
    }

    // This function is a wrapper around `self.hermes.create_channel` that helps track the channel creation and all surrounding IBC messages
    pub async fn create_hermes_channel(
        &self,
        connection: &str,
        channel_version: &str,
        contract_a: &dyn ContractInstance<Daemon>,
        contract_b: &dyn ContractInstance<Daemon>,
        configure_local_network: Option<bool>,
    ) -> Result<(), DaemonError> {
        let channel_creation_events_a = vec![
            format!(
                "channel_open_ack.port_id='wasm.{}'",
                contract_a.address().unwrap()
            ), // client is on chain1
            format!(
                "channel_open_ack.counterparty_port_id='wasm.{}'",
                contract_b.address().unwrap()
            ), // host is on chain2
            format!("channel_open_ack.connection_id='{}'", connection),
        ];

        let channel_creation_events_b = vec![
            format!(
                "channel_open_confirm.port_id='wasm.{}'",
                contract_b.address().unwrap()
            ),
            format!(
                "channel_open_confirm.counterparty_port_id='wasm.{}'",
                contract_a.address().unwrap()
            ),
            format!("channel_open_confirm.connection_id='{}'", connection),
        ];
        let channel_a = contract_a.get_chain().channel();
        let channel_b = contract_b.get_chain().channel();

        // First we get the last transactions for channel creation on the port, to make sure the tx we will intercept later is a new one
        let current_channel_creation_hash_a = &Node::new(channel_a.clone())
            .find_tx_by_events(
                channel_creation_events_a.clone(),
                None,
                Some(cosmos_sdk_proto::cosmos::tx::v1beta1::OrderBy::Desc),
            )
            .await?
            .get(0)
            .map(|tx| tx.txhash.clone());

        let current_channel_creation_hash_b = &Node::new(channel_b.clone())
            .find_tx_by_events(
                channel_creation_events_b.clone(),
                None,
                Some(cosmos_sdk_proto::cosmos::tx::v1beta1::OrderBy::Desc),
            )
            .await?
            .get(0)
            .map(|tx| tx.txhash.clone());

        // Then we can safely create the channel
        self.hermes
            .create_channel(connection, channel_version, contract_a, contract_b)
            .await;

        log::info!("Channel creation message sent to hermes, awaiting for channel creation end");

        // Then we make sure the channel is indeed created between the two chains
        // We get the channel open on chain 1
        let channel_creation_tx_a = find_new_tx_with_events(
            &channel_a,
            &channel_creation_events_a,
            current_channel_creation_hash_a,
        )
        .await?;
        let channel_creation_tx_b = find_new_tx_with_events(
            &channel_b,
            &channel_creation_events_b,
            current_channel_creation_hash_b,
        )
        .await?;

        log::info!("Successfully created a channel between {} and {} on connection '{}' and channels {}:'{}'(txhash : {}) and {}:'{}' (txhash : {})", 
            contract_a.address().unwrap(),
            contract_b.address().unwrap(),
            connection,
            contract_a.get_chain().state().chain_id,
            channel_creation_tx_a.get_events("channel_open_ack")[0].get_first_attribute_value("channel_id").unwrap(),
            channel_creation_tx_a.txhash,
            contract_b.get_chain().state().chain_id,
            channel_creation_tx_b.get_events("channel_open_confirm")[0].get_first_attribute_value("channel_id").unwrap(),
            channel_creation_tx_b.txhash,
        );

        // Finally, we make sure additional packets are resolved before returning
        let grpc_channel_a = contract_a.get_chain().channel();
        let chain_id_a = contract_a.get_chain().state().chain_id.clone();
        let tx_hash_a = channel_creation_tx_a.txhash.clone();

        let grpc_channel_b = contract_b.get_chain().channel();
        let chain_id_b = contract_b.get_chain().state().chain_id.clone();
        let tx_hash_b = channel_creation_tx_b.txhash.clone();

        follow_trail(
            chain_id_a,
            grpc_channel_a,
            tx_hash_a,
            configure_local_network,
        )
        .await
        .unwrap();

        follow_trail(
            chain_id_b,
            grpc_channel_b,
            tx_hash_b,
            configure_local_network,
        )
        .await
        .unwrap();
        Ok(())
    }
}

const MAX_TX_QUERY_RETRIES: usize = 5;
async fn find_new_tx_with_events(
    channel: &Channel,
    events: &Vec<String>,
    last_hash: &Option<String>,
) -> Result<CosmTxResponse, DaemonError> {
    for _ in 0..MAX_TX_QUERY_RETRIES {
        match &Node::new(channel.clone())
            .find_tx_by_events(
                events.clone(),
                None,
                Some(cosmos_sdk_proto::cosmos::tx::v1beta1::OrderBy::Desc),
            )
            .await
        {
            Ok(txs) => {
                if let Some(tx) = txs.get(0) {
                    if tx.txhash != last_hash.clone().unwrap_or("".to_string()) {
                        return Ok(tx.clone());
                    }
                }
                log::debug!("No new TX by events found");
                log::debug!("Waiting 10s");
                sleep(Duration::from_secs(10)).await;
            }
            Err(_) => break,
        }
    }

    Err(DaemonError::AnyError(anyhow::Error::msg(format!(
        "No newer TX than {:?} found with events {:?}",
        last_hash, events
    ))))
}

/// Get the file path for the log target
fn generate_log_file_path(file: &str) -> PathBuf {
    let file_name = format!("{}.log", file);

    let mut log_path = std::env::current_dir().unwrap();
    log_path.push("logs");
    log_path.push(file_name);

    log_path
}
