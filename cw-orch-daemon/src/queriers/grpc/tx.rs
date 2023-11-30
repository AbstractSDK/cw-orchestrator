// Only a simple implementation to not overload the tx builder

use cosmrs::{proto::cosmos::base::abci::v1beta1::TxResponse, tx::Raw};
use tonic::transport::Channel;

use crate::{cosmos_modules, queriers::DaemonQuerier, DaemonError};

/// Queries for Cosmos Bank Module
pub struct Tx {
    channel: Channel,
}

impl DaemonQuerier for Tx {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Tx {
    /// Query spendable balance for address
    pub async fn broadcast(&self, tx: Raw) -> Result<TxResponse, DaemonError> {
        let mut client =
            cosmos_modules::tx::service_client::ServiceClient::new(self.channel.clone());

        let resp = client
            .broadcast_tx(cosmos_modules::tx::BroadcastTxRequest {
                tx_bytes: tx.to_bytes()?,
                mode: cosmos_modules::tx::BroadcastMode::Sync.into(),
            })
            .await?
            .into_inner();

        Ok(resp.tx_response.unwrap())
    }
}
