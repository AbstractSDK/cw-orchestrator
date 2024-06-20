use std::collections::HashMap;

use cw_orch_core::environment::ChainInfoOwned;
use cw_orch_core::environment::ChainState;
use cw_orch_daemon::Daemon;
use cw_orch_interchain_daemon::{IcDaemonResult, Mnemonic};
use tokio::runtime::Handle;
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
}
