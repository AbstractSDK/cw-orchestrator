//! This crate provides an integration with [starship](https://starship.cosmology.tech/) to be able to interact with it in rust directly.
#![warn(missing_docs)]

/// Path to the starship config that's currently deployed. Default values will be used otherwise(port 8081)
pub const STARSHIP_CONFIG_ENV_NAME: &str = "CW_ORCH_STARSHIP_CONFIG_PATH";

pub mod client;

use crate::client::StarshipClient;
use cw_orch_core::environment::{ChainInfoOwned, ChainState, NetworkInfoOwned};
use cw_orch_core::CwEnvError;
use cw_orch_daemon::{Daemon, DaemonBuilder, RUNTIME};
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
    pub fn new(url: Option<&str>) -> Result<Self, CwEnvError> {
        let runtime = RUNTIME.handle();
        Self::new_with_runtime(runtime, url)
    }

    /// Creates a new instance and connects to a starship deployment
    pub fn new_with_runtime(rt_handle: &Handle, url: Option<&str>) -> Result<Self, CwEnvError> {
        let starship_config = match try_to_read_starship_config() {
            Ok(config) => Some(config),
            Err(e) => {
                log::log!(log::Level::Warn, "Not using yaml config: {e:?}");
                None
            }
        };
        let starship_client = StarshipClient::new(rt_handle.clone(), url, starship_config)?;

        let mut daemons: HashMap<String, Daemon> = HashMap::new();
        for chain in starship_client.chains.iter() {
            let mnemonic = rt_handle.block_on(async {
                let registry = starship_client.registry().await;
                registry
                    .test_mnemonic(chain.chain_id.as_str())
                    .await
                    .unwrap()
            });

            let mut daemon_builder = DaemonBuilder::new(chain_data_conversion(chain.clone()));
            let mut daemon_builder = daemon_builder
                .mnemonic(mnemonic)
                .load_network(false)
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

fn try_to_read_starship_config() -> Result<yaml_rust2::Yaml, CwEnvError> {
    let env = std::env::var(STARSHIP_CONFIG_ENV_NAME)?;
    let source = std::fs::read_to_string(env)?;

    let mut yaml_docs = yaml_rust2::YamlLoader::load_from_str(&source)
        .map_err(|e| CwEnvError::StdErr(e.info().to_owned()))?;

    if yaml_docs.len() != 1 {
        return Err(CwEnvError::StdErr("too many starship configs".to_owned()));
    }
    Ok(yaml_docs.pop().unwrap())
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
