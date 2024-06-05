//! Defines a structure to interact with an online chain registry

use ibc_chain_registry::{chain::ChainData, paths::IBCPath};
use serde::Deserialize;
use serde_json::Value;
use url::Url;

use super::{error::StarshipClientError, StarshipClientResult};

/// Alias for URL
pub type URL = String;

/// Structure that is able to query information about the chains inside the starship registry
#[derive(Debug, Clone)]
pub struct Registry(url::Url);

impl Registry {
    /// Construct the registry url from the config
    pub async fn new(url: Url) -> Self {
        let registry = Self(url);

        let registry_url = registry.chains_data_url();

        // Assert that the registry is reachable
        let client = reqwest::Client::new();
        client
            .get(registry_url)
            .send()
            .await
            .map_err(|e| StarshipClientError::RegistryError(e.to_string()))
            .unwrap();

        registry
    }

    fn get_url(&self, subpath: &str) -> String {
        self.0.join(subpath).unwrap().to_string()
    }

    fn chains_data_url(&self) -> String {
        self.get_url("chains")
    }

    fn ibc_data_url(&self) -> String {
        self.get_url("ibc")
    }

    /// Get an IBC path between two chains.
    pub async fn ibc_path(
        &self,
        chain_id_a: &str,
        chain_id_b: &str,
    ) -> StarshipClientResult<IBCPath> {
        let ibc_path_url = format!("{}/{}/{}", self.ibc_data_url(), chain_id_a, chain_id_b);
        eprintln!("ibc_paths_url: {:?}", ibc_path_url);

        let response = reqwest::get(&ibc_path_url).await?;
        let path: IBCPath = response.json().await?;
        Ok(path)
    }

    /// Get all the chain data for this registry.
    pub async fn chain_data(&self) -> StarshipClientResult<Vec<ChainData>> {
        let response = reqwest::get(&self.chains_data_url()).await?;
        let value: Value = response.json().await?;

        let chains: Vec<ChainData> = serde_json::from_value(value["chains"].clone()).unwrap();
        Ok(chains)
    }

    /// Get the first test account mnemonic from the chain registry.
    pub async fn test_mnemonic(&self, chain_id: &str) -> Result<String, StarshipClientError> {
        let url = self.get_url(&format!("chains/{}/keys", chain_id));
        let response = reqwest::get(&url).await?;
        let data: Mnemonics = response.json().await?;
        let first_test_account_mnemonic = data
            .genesis
            .first()
            .ok_or_else(|| StarshipClientError::MissingTestMnemonic(chain_id.to_string()))?
            .mnemonic
            .clone();
        Ok(first_test_account_mnemonic)
    }
}

#[derive(Deserialize, Debug)]
struct Record {
    #[allow(dead_code)]
    name: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    record_type: String,
    mnemonic: String,
}

#[derive(Deserialize, Debug)]
struct Mnemonics {
    genesis: Vec<Record>,
    #[allow(dead_code)]
    validators: Vec<Record>,
    #[allow(dead_code)]
    keys: Vec<Record>,
    #[allow(dead_code)]
    relayers: Vec<Record>,
}
