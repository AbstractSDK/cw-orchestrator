//! Rust implementation for interacting with a faucet similar to https://github.com/cosmos/cosmjs/tree/main/packages/faucet
use serde::{Deserialize, Serialize};

use super::{error::StarshipClientError, registry::URL, StarshipClientResult};

/// Faucet structure that allows interacting with an online faucet
#[derive(Debug, Clone)]
pub struct Faucet(URL);

/// Faucet request type
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    /// Address of the address asking for funds
    pub address: String,
    /// Denom asked for
    pub denom: String,
}

impl Faucet {
    /// Get a faucet object from a url and port
    pub async fn new(url: impl ToString, port: impl ToString) -> Self {
        let path = format!("{}:{}", url.to_string(), port.to_string());
        // Assert that the faucet is reachable
        let client = reqwest::Client::new();
        client
            .get(format!("http://{path}/status"))
            .send()
            .await
            .map_err(|e| StarshipClientError::FaucetError(e.to_string()))
            .unwrap();
        Self(path)
    }

    /// Requests funds for a given address
    /// Returns as soon as the funds are distributed to the address
    pub async fn request_funds(
        &self,
        address: impl ToString,
        denom: impl ToString,
    ) -> StarshipClientResult<()> {
        let faucet = &self.0;
        let url = format!("http://{}/{}", faucet, address.to_string());
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .json(&Request {
                address: address.to_string(),
                denom: denom.to_string(),
            })
            .send()
            .await
            .map_err(|e| StarshipClientError::FaucetError(e.to_string()))?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(StarshipClientError::FaucetError(response.text().await?))
        }
    }
}
