// Only a simple implementation to not overload the tx builder

use cosmrs::{rpc::HttpClient, tx::Raw, proto::cosmos::base::abci::v1beta1::TxResponse};

use crate::{queriers::DaemonQuerier, cosmos_rpc_query, DaemonError, cosmos_modules};


/// Queries for Cosmos Bank Module
pub struct Tx {
    client: HttpClient,
}

impl DaemonQuerier for Tx {
    fn new(client: HttpClient) -> Self {
        Self { client }
    }
}

impl Tx{

    /// Query spendable balance for address
    pub async fn broadcast(
        &self,
        tx: Raw,
    ) -> Result<TxResponse, DaemonError> {
        let resp = cosmos_rpc_query!(
            self,
            tx,
            "/cosmos.tx.v1beta1.Service/BroadcastTx",
            BroadcastTxRequest {
                tx_bytes: tx.to_bytes()?,
                mode: cosmos_modules::tx::BroadcastMode::Sync.into(),
            },
            BroadcastTxResponse,
        );
        Ok(resp.tx_response.unwrap())
    }
}