use std::collections::HashMap;

use cw_orch_core::environment::ChainInfoOwned;
use cw_orch_core::environment::ChainState;
use cw_orch_daemon::Daemon;
use cw_orch_interchain_core::env::ChainId;
use cw_orch_interchain_daemon::{IcDaemonResult, Mnemonic};
use ibc_relayer::chain::handle::ChainHandle;
use tokio::runtime::Handle;

use ibc_relayer::config::{Config, RestConfig, TelemetryConfig};

use crate::config::chain_config;
use crate::config::KEY_NAME;
use crate::keys::restore_key;

#[derive(Clone)]
pub struct HermesRelayer {
    /// Daemon objects representing all the chains available inside the starship environment
    pub daemons: HashMap<String, (Daemon, bool, String)>,
    /// Runtime handle for awaiting async functions
    pub rt_handle: Handle,

    pub connection_ids: HashMap<(String, String), String>,
}

impl HermesRelayer {
    /// Builds a new `InterchainEnv` instance.
    /// For use with starship, we advise to use `Starship::interchain_env` instead
    pub fn new<T>(
        runtime: &Handle,
        chains: Vec<(T, Option<Mnemonic>, bool, String)>,
        connections: HashMap<(String, String), String>,
    ) -> IcDaemonResult<Self>
    where
        T: Into<ChainInfoOwned>,
    {
        let mut env = Self::raw(runtime);

        // We create daemons for each chains
        for (chain_data, mnemonic, is_consumer_chain, rpc) in chains {
            let daemon = env.build_daemon(runtime, chain_data.into(), mnemonic)?;
            env.daemons.insert(
                daemon.state().chain_data.chain_id.to_string(),
                (daemon, is_consumer_chain, rpc),
            );
        }
        env.connection_ids = connections;

        Ok(env)
    }

    /// This creates an interchain environment from existing daemon instances
    /// The `channel_creator` argument will be responsible for creation interchain channel
    /// If using starship, prefer using Starship::interchain_env for environment creation
    pub fn from_daemons(rt: &Handle, daemons: Vec<(Daemon, bool, String)>) -> Self {
        let mut env = Self::raw(rt);
        for (daemon, is_consumer_chain, rpc) in daemons {
            env.daemons.insert(
                daemon.state().chain_data.chain_id.to_string(),
                (daemon, is_consumer_chain, rpc),
            );
        }
        env
    }

    fn raw(rt: &Handle) -> Self {
        Self {
            daemons: HashMap::new(),
            rt_handle: rt.clone(),
            connection_ids: Default::default(),
        }
    }

    /// Build a daemon from chain data and mnemonic and add it to the current configuration
    fn build_daemon(
        &mut self,
        runtime: &Handle,
        chain_data: ChainInfoOwned,
        mnemonic: Option<impl ToString>,
    ) -> IcDaemonResult<Daemon> {
        let mut daemon_builder = Daemon::builder();
        let mut daemon_builder = daemon_builder.chain(chain_data.clone()).handle(runtime);

        daemon_builder = if let Some(mn) = mnemonic {
            daemon_builder.mnemonic(mn)
        } else {
            daemon_builder
        };

        // State is shared between daemons, so if a daemon already exists, we use its state
        daemon_builder = if let Some((daemon, _, _)) = self.daemons.values().next() {
            daemon_builder.state(daemon.state())
        } else {
            daemon_builder
        };

        let daemon = daemon_builder.build().unwrap();

        Ok(daemon)
    }

    pub fn duplex_config(&self, src_chain: ChainId, dst_chain: ChainId) -> Config {
        let (src_daemon, src_is_consumer_chain, src_rpc_url) = self.daemons.get(src_chain).unwrap();
        let src_chain_data = &src_daemon.state().chain_data;

        let (dst_daemon, dst_is_consumer_chain, dst_rpc_url) = self.daemons.get(dst_chain).unwrap();
        let dst_chain_data = &dst_daemon.state().chain_data;

        Config {
            global: ibc_relayer::config::GlobalConfig {
                log_level: ibc_relayer::config::LogLevel::Info,
            },
            mode: ibc_relayer::config::ModeConfig::default(),
            rest: RestConfig::default(),
            telemetry: TelemetryConfig::default(),
            chains: vec![
                chain_config(
                    src_chain,
                    src_rpc_url,
                    src_chain_data,
                    *src_is_consumer_chain,
                ),
                chain_config(
                    dst_chain,
                    dst_rpc_url,
                    dst_chain_data,
                    *dst_is_consumer_chain,
                ),
            ],
            tracing_server: Default::default(),
        }
    }

    pub fn add_key(&self, chain: &impl ChainHandle) {
        let chain_id = chain.config().unwrap().id().to_string();

        let (daemon, _, _) = self.daemons.get(&chain_id).unwrap();

        let chain_data = &daemon.state().chain_data;
        let hd_path = daemon.wallet().options().hd_index;
        let key = restore_key(self.mnemonic().clone(), hd_path.unwrap_or(0), chain_data).unwrap();
        chain.add_key(KEY_NAME.to_string(), key).unwrap();
    }

    fn mnemonic(&self) -> String {
        std::env::var("TEST_MNEMONIC").unwrap()
    }
}
