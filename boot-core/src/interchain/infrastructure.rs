//! Interactions with docker using bollard

use futures_util::stream::StreamExt;
use ibc_chain_registry::chain::{ChainData, Grpc};

use std::default::Default;
use std::{rc::Rc, sync::Arc};
use tokio::runtime::Runtime;

use crate::{
    daemon::{sender::Sender, state::DaemonState},
    networks::{osmosis::OSMO_2, JUNO_1},
    Daemon, DaemonError, DaemonOptionsBuilder,
};

use super::docker::DockerHelper;
use super::hermes::Hermes;
use super::IcResult;

const CHAIN_A_MNEMONIC: &str = "dilemma imitate split detect useful creek cart sort grow essence fish husband seven hollow envelope wedding host dry permit game april present panic move";
const CHAIN_B_MNEMONIC: &str = "settle gas lobster judge silk stem act shoulder pluck waste pistol word comfort require early mouse provide marine butter crowd clock tube move wool";

pub type ContainerId = String;
pub type Port = String;
pub struct InterchainInfrastructure {
    pub chain_a: Daemon,
    pub chain_b: Daemon,
    pub hermes: Hermes,
}

impl InterchainInfrastructure {
    /// Build a new `InterchainInfrastructure` instance.
    /// 1. Check if interchain_test is installed
    /// 2. Clone test file to interchain test dir and run it
    /// 3. Wait for X amount of time
    /// 4. Get container information (daemons and Hermes)
    pub fn new(runtime: &Arc<Runtime>) -> IcResult<Self> {
        // Start from local options
        let (chains, hermes) = runtime.block_on(Self::configure_networks([JUNO_1, OSMO_2]))?;
        let mut daemons = Self::build_daemons(runtime, &chains)?;

        Ok(Self {
            chain_a: daemons.swap_remove(0),
            chain_b: daemons.swap_remove(0),
            hermes,
        })
    }

    async fn configure_networks(
        networks: [impl Into<ChainData>; 2],
    ) -> IcResult<(Vec<ChainData>, Hermes)> {
        // convert to chain data
        let mut networks = networks
            .into_iter()
            .map(|network| network.into())
            .collect::<Vec<ChainData>>();

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

        let hermes = docker_helper.get_hermes()?;
        Ok((networks, hermes))
    }

    /// Build the daemons from the shared runtime and chain data
    /// uses [CHAIN_A_MNEMONIC] and [CHAIN_B_MNEMONIC] to create senders
    fn build_daemons(
        runtime: &Arc<Runtime>,
        chain_data: &[ChainData],
    ) -> Result<Vec<Daemon>, DaemonError> {
        let chain_a_options = DaemonOptionsBuilder::default()
            .network(chain_data[0].clone())
            .deployment_id("interchain")
            .build()
            .unwrap();
        let chain_b_options = DaemonOptionsBuilder::default()
            .network(chain_data[1].clone())
            .deployment_id("interchain")
            .build()
            .unwrap();

        let chain_a_state = Rc::new(runtime.block_on(DaemonState::new(chain_a_options))?);
        let chain_b_state = Rc::new(runtime.block_on(DaemonState::new(chain_b_options))?);

        let chain_a_sender = Rc::new(Sender::from_mnemonic(&chain_a_state, CHAIN_A_MNEMONIC)?);
        let chain_b_sender = Rc::new(Sender::from_mnemonic(&chain_b_state, CHAIN_B_MNEMONIC)?);

        let chain_a = Daemon::new(&chain_a_sender, &chain_a_state, runtime)?;
        let chain_b = Daemon::new(&chain_b_sender, &chain_b_state, runtime)?;
        Ok(vec![chain_a, chain_b])
    }
}
