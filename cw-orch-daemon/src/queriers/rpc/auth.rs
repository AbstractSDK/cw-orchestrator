// Only a simple implementation to not overload the tx builder

use cosmrs::rpc::HttpClient;

use crate::{queriers::DaemonQuerier, cosmos_rpc_query, DaemonError};


/// Queries for Cosmos Auth Module
pub struct Auth {
    client: HttpClient,
}

impl DaemonQuerier for Auth {
    fn new(client: HttpClient) -> Self {
        Self { client }
    }
}

impl Auth{

    /// Query the account
    pub async fn account(
        &self,
        address: impl Into<String>,
    ) -> Result<Vec<u8>, DaemonError> {
        let resp = cosmos_rpc_query!(
            self,
            auth,
            "/cosmos.auth.v1beta1.Query/Account",
            QueryAccountRequest { address: address.into() },
            QueryAccountResponse,
        );
        Ok(resp.account.unwrap().value)
    }
}