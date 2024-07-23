// Only a simple implementation to not overload the tx builder

use cosmrs::{proto::cosmos::base::abci::v1beta1::TxResponse, rpc::HttpClient, tx::Raw};

use crate::{queriers::DaemonQuerier, DaemonError};
use cosmrs::rpc::Client;

/// Queries for Cosmos Bank Module
pub struct Tx {
    client: HttpClient,
}

impl DaemonQuerier for Tx {
    fn new(client: HttpClient) -> Self {
        Self { client }
    }
}

impl Tx {
    /// Query spendable balance for address
    pub async fn broadcast(&self, tx: Raw) -> Result<TxResponse, DaemonError> {
        let resp = self.client.broadcast_tx_commit(tx.to_bytes()?).await?;

        let check = resp.check_tx;
        Ok(TxResponse {
            height: resp.height.into(),
            txhash: resp.hash.to_string(),
            codespace: check.codespace,
            code: check.code.into(),
            data: "".to_string(),
            raw_log: check.log,
            logs: vec![],
            info: check.info,
            gas_wanted: check.gas_wanted,
            gas_used: check.gas_used,
            tx: None,
            timestamp: "".to_string(),
            events: vec![],
        })
    }
}
