//! Starship integration

use crate::{
    daemon::DaemonBuilder,
    prelude::{CwOrchError, Daemon, InterchainEnv},
};
use cw_orch_starship::StarshipClient;
use std::collections::HashMap;
use tokio::runtime::Handle;

/// Starship integration
pub struct Starship {
    daemons: HashMap<String, Daemon>,
    starship_client: StarshipClient,
}

impl Starship {
    ///
    pub fn new(rt_handle: Handle, url: Option<&str>) -> Result<Self, CwOrchError> {
        let starship_client = StarshipClient::new(rt_handle.clone(), url)?;

        let mut daemons = HashMap::new();
        for chain in starship_client.chains.iter() {
            let mnemonic = rt_handle.block_on(async {
                let registry = starship_client.registry().await;
                let mnemonic = registry
                    .test_mnemonic(chain.chain_id.as_str())
                    .await
                    .unwrap();
                mnemonic
            });

            let daemon = DaemonBuilder::default()
                .chain(chain.clone())
                .mnemonic(mnemonic)
                .handle(&rt_handle)
                .build()?;
            daemons.insert(chain.chain_id.to_string(), daemon);
        }

        Ok(Self {
            daemons,
            starship_client,
        })
    }
    /// Get a chain daemon from the starship infrastructure
    pub fn daemon(&self, chain_id: &str) -> Result<&Daemon, CwOrchError> {
        self.daemons
            .get(chain_id)
            .ok_or(anyhow::anyhow!("Chain not found: {}", chain_id).into())
    }
    /// Get the starship client
    pub fn client(&self) -> &StarshipClient {
        &self.starship_client
    }
    /// Get all daemons
    pub fn daemons(&self) -> Vec<Daemon> {
        self.daemons.values().cloned().collect()
    }
    /// Creates an interchain environement object to be able to track all transactions and ibc related operations
    pub fn interchain_env(&self) -> InterchainEnv {
        InterchainEnv::from_daemons(self.daemons())
    }
}