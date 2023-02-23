//! Interactions with docker using bollard

use bollard::models::ContainerSummary;
use bollard::Docker;
use bollard::{
    container::{InspectContainerOptions, ListContainersOptions},
    service::{PortBinding, PortMap},
};
use futures_util::stream;
use futures_util::stream::StreamExt;
use ibc_chain_registry::chain::{ChainData, Grpc};
use secp256k1::All;
use std::default::Default;
use std::{collections::HashMap, rc::Rc, sync::Arc};
use tokio::runtime::Runtime;

use crate::{
    daemon::{sender::Sender, state::DaemonState},
    networks::{osmosis::OSMO_2, JUNO_1, LOCAL_JUNO, LOCAL_OSMO, OSMO_4},
    Daemon, DaemonError, DaemonOptions, DaemonOptionsBuilder,
};

use super::IcResult;

const CHAIN_A_MNEMONIC: &str = "dilemma imitate split detect useful creek cart sort grow essence fish husband seven hollow envelope wedding host dry permit game april present panic move";
const CHAIN_B_MNEMONIC: &str = "settle gas lobster judge silk stem act shoulder pluck waste pistol word comfort require early mouse provide marine butter crowd clock tube move wool";

pub type ContainerId = String;
pub type Port = String;
pub struct InterchainInfrastructure {
    pub chain_a: Daemon,
    pub chain_b: Daemon,
}

impl InterchainInfrastructure {
    pub fn new(runtime: &Arc<Runtime>) -> IcResult<Self> {
        // Start from local options
        let chains = runtime.block_on(Self::configure_networks([JUNO_1, OSMO_2]))?;
        let mut daemons = Self::build_daemons(runtime, &chains)?;
        Ok(Self {
            chain_a: daemons.swap_remove(0),
            chain_b: daemons.swap_remove(0),
        })
    }

    async fn configure_networks(networks: [impl Into<ChainData>; 2]) -> IcResult<Vec<ChainData>> {
        // convert to chain data
        let mut networks = networks
            .into_iter()
            .map(|network| network.into())
            .collect::<Vec<ChainData>>();

        // use chain data network name as to filter container ids
        let containers_grpc_port = Self::grpc_ports().await?;

        // update network with correct grpc port
        networks.iter_mut().for_each(|network| {
            for container in &containers_grpc_port {
                if container.0.contains(&network.chain_name) {
                    network.apis.grpc = vec![Grpc {
                        address: format!("http://0.0.0.0:{}", container.1),
                        ..Default::default()
                    }];
                }
            }
        });
        Ok(networks)
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

    async fn ports(arg: (Docker, &ContainerSummary)) -> IcResult<(ContainerId, PortMap)> {
        let (docker, container) = arg;

        let port_map = docker
            .inspect_container(
                container.id.as_ref().unwrap(),
                None::<InspectContainerOptions>,
            )
            .await?
            .network_settings
            .unwrap()
            .ports
            .unwrap();

        let name = container
            .names
            .as_ref()
            .unwrap()
            .first()
            .unwrap()
            .to_owned();

        Ok((name, port_map))
    }

    fn map_container_port(port_map: &PortMap, port: &str) -> String {
        for (container_port, bindings) in port_map {
            if container_port.contains(port) {
                return bindings
                    .as_ref()
                    .unwrap()
                    .first()
                    .unwrap()
                    .host_port
                    .as_ref()
                    .unwrap()
                    .to_owned();
            }
        }
        panic!("No port found for {} given portmap {:#?}", port, port_map);
    }

    /// Get the port mapping for all the running containers
    pub async fn container_ports() -> IcResult<Vec<(ContainerId, PortMap)>> {
        // connect to docker
        let docker = Docker::connect_with_socket_defaults().unwrap();
        // get all running containers
        let mut list_container_filters = HashMap::new();
        list_container_filters.insert("status", vec!["running"]);
        let containers = &docker
            .list_containers(Some(ListContainersOptions {
                all: true,
                filters: list_container_filters,
                ..Default::default()
            }))
            .await?;
        let docker_stream = stream::repeat(docker);
        let result_stream = docker_stream
            .zip(stream::iter(containers))
            .map(|(docker_item, container_item)| {
                async move {
                    // Call your function on docker_item and container_item here
                    let result = Self::ports((docker_item, container_item)).await.unwrap();
                    result
                }
            })
            .buffer_unordered(2) // Run up to 2 tasks concurrently
            .boxed();
        let results = result_stream.collect::<Vec<_>>().await;
        Ok(results)
    }

    /// Get the grpc ports for all the running containers
    pub async fn grpc_ports() -> IcResult<Vec<(ContainerId, Port)>> {
        Self::container_ports().await.map(|container_binded_ports| {
            container_binded_ports
                .into_iter()
                .map(|(id, mapping)| (id, Self::map_container_port(&mapping, "9090")))
                .collect()
        })
    }
}
