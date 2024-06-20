//! This crate provides an integration with [starship](https://starship.cosmology.tech/) to be able to interact with it in rust directly.
#![warn(missing_docs)]

pub mod client;

use crate::client::StarshipClient;
use cw_orch_core::environment::{ChainInfoOwned, ChainState, NetworkInfoOwned};
use cw_orch_core::CwEnvError;
use cw_orch_daemon::{Daemon, DaemonBuilder};
use ibc_chain_registry::chain::ChainData;
use std::collections::HashMap;
use tokio::runtime::Handle;

#[derive(Clone)]
/// Starship integration
pub struct Starship {
    /// Daemon objects representing all the chains available inside the starship environment
    pub daemons: HashMap<String, Daemon>,
    starship_client: StarshipClient,
    /// Runtime handle for awaiting async functions
    pub rt_handle: Handle,
}

impl Starship {
    /// Creates a new instance and connects to a starship deployment
    pub fn new(rt_handle: &Handle, url: Option<&str>) -> Result<Self, CwEnvError> {
        let starship_client = StarshipClient::new(rt_handle.clone(), url)?;

        let mut daemons: HashMap<String, Daemon> = HashMap::new();
        for chain in starship_client.chains.iter() {
            let mnemonic = rt_handle.block_on(async {
                let registry = starship_client.registry().await;
                registry
                    .test_mnemonic(chain.chain_id.as_str())
                    .await
                    .unwrap()
            });

            let mut daemon_builder = DaemonBuilder::default();
            let mut daemon_builder = daemon_builder
                .chain(chain_data_conversion(chain.clone()))
                .mnemonic(mnemonic)
                .handle(rt_handle);

            if let Some(existing_daemon) = daemons.values().next() {
                daemon_builder = daemon_builder.state(existing_daemon.state())
            }

            daemons.insert(chain.chain_id.to_string(), daemon_builder.build()?);
        }

        Ok(Self {
            daemons,
            starship_client,
            rt_handle: rt_handle.clone(),
        })
    }
    /// Get a chain daemon from the starship infrastructure
    pub fn daemon(&self, chain_id: &str) -> Result<&Daemon, CwEnvError> {
        self.daemons
            .get(chain_id)
            .ok_or(CwEnvError::StdErr(format!("Chain not found: {}", chain_id)))
    }
    /// Get the starship client
    pub fn client(&self) -> &StarshipClient {
        &self.starship_client
    }
    /// Get all daemons
    pub fn daemons(&self) -> Vec<Daemon> {
        self.daemons.values().cloned().collect()
    }
}

fn chain_data_conversion(chain: ChainData) -> ChainInfoOwned {
    ChainInfoOwned {
        chain_id: chain.chain_id.to_string(),
        gas_denom: chain.fees.fee_tokens[0].denom.clone(),
        gas_price: chain.fees.fee_tokens[0].average_gas_price,
        grpc_urls: chain.apis.grpc.into_iter().map(|g| g.address).collect(),
        lcd_url: Some(chain.apis.rest.into_iter().map(|l| l.address).collect()),
        fcd_url: None,
        network_info: NetworkInfoOwned {
            chain_name: chain.chain_name,
            pub_address_prefix: chain.bech32_prefix,
            coin_type: chain.slip44,
        },
        kind: chain.network_type.into(),
    }
}
