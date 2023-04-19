//! Interactions with docker using bollard

use ibc_chain_registry::chain::{ChainData, Grpc};

use std::collections::HashMap;
use std::default::Default;
use std::{rc::Rc, sync::Arc};
use tokio::runtime::Runtime;

use crate::{
    daemon::{sender::Sender, state::DaemonState},
    Daemon, DaemonError, DaemonOptionsBuilder,
};

use super::docker::DockerHelper;
use super::hermes::Hermes;
use super::IcResult;

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
    /// Build a new `InterchainInfrastructure` instance.
    /// 1. Check if interchain_test is installed
    /// 2. Clone test file to interchain test dir and run it
    /// 3. Wait for X amount of time
    /// 4. Get container information (daemons and Hermes)
    pub fn new<T>(runtime: &Arc<Runtime>, chains: Vec<(T, &str)>) -> IcResult<Self>
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
            &chains.into_iter().zip(mnemonics).collect::<Vec<_>>(),
        )?;
        let hermes = runtime.block_on(Self::get_hermes())?;
        Ok(Self { daemons, hermes })
    }

    /// Get the daemon for a network-id in the interchain.
    pub fn daemon(&self, chain_id: &str) -> Daemon {
        self.daemons
            .get(chain_id)
            .expect(format!("Daemon for {} not found in interchain", chain_id).as_str())
            .clone()
    }

    /// Get the gRPC ports for the local daemons and set them in the `ChainData` objects.
    async fn configure_networks(networks: &mut Vec<ChainData>) -> IcResult<()> {
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
        runtime: &Arc<Runtime>,
        chain_data: &[(ChainData, Mnemonic)],
    ) -> Result<HashMap<NetworkId, Daemon>, DaemonError> {
        let mut daemons = HashMap::new();
        for (chain, mnemonic) in chain_data {
            let options = DaemonOptionsBuilder::default()
                .network(chain.clone())
                .deployment_id("interchain")
                .build()
                .unwrap();
            let chain_a_state = Rc::new(runtime.block_on(DaemonState::new(options))?);
            let chain_a_sender = Rc::new(Sender::from_mnemonic(&chain_a_state, &mnemonic)?);
            daemons.insert(
                chain.chain_id.to_string(),
                Daemon::new(&chain_a_sender, &chain_a_state, runtime)?,
            );
        }
        Ok(daemons)
    }
}
