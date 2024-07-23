// Only a simple implementation to not overload the tx builder
use tonic::transport::Channel;

use crate::{cosmos_query, queriers::DaemonQuerier, DaemonError};

/// Queries for Cosmos Bank Module
pub struct Auth {
    channel: Channel,
}

impl DaemonQuerier for Auth {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Auth {
    /// Query spendable balance for address
    pub async fn account(&self, address: impl Into<String>) -> Result<Vec<u8>, DaemonError> {
        let resp = cosmos_query!(
            self,
            auth,
            account,
            QueryAccountRequest {
                address: address.into()
            }
        );
        Ok(resp.account.unwrap().value)
    }
}
