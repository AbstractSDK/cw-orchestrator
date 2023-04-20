use std::time::Duration;

use crate::{
    daemon::{cosmos_modules, tx_resp::CosmTxResponse},
    DaemonError,
};

use cosmrs::tendermint::{Block, Time};
use tokio::time::sleep;
use tonic::transport::Channel;

use super::DaemonQuerier;

const MAX_TX_QUERY_RETRIES: u64 = 5;

/// Querier for the Tendermint node.
/// Supports queries for block and tx information
pub struct Node {
    channel: Channel,
}

impl DaemonQuerier for Node {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Node {
    /// Returns the latest block information from the daemon.
    pub async fn latest_block(&self) -> Result<Block, DaemonError> {
        let mut client =
            cosmos_modules::tendermint::service_client::ServiceClient::new(self.channel.clone());

        #[allow(deprecated)]
        let resp = client
            .get_latest_block(cosmos_modules::tendermint::GetLatestBlockRequest {})
            .await?
            .into_inner();

        Ok(Block::try_from(resp.block.unwrap())?)
    }

    /// Returns current block height
    pub async fn block_height(&self) -> Result<u64, DaemonError> {
        let block = self.latest_block().await?;
        Ok(block.header.height.value())
    }

    /// Returns the block timestamp (since unix epoch) in nanos
    pub async fn block_time(&self) -> Result<u128, DaemonError> {
        let block = self.latest_block().await?;
        Ok(block
            .header
            .time
            .duration_since(Time::unix_epoch())?
            .as_nanos())
    }

    /// Simulate TX
    pub async fn simulate_tx(&self, tx_bytes: Vec<u8>) -> Result<u64, DaemonError> {
        let mut client =
            cosmos_modules::tx::service_client::ServiceClient::new(self.channel.clone());
        #[allow(deprecated)]
        let resp = client
            .simulate(cosmos_modules::tx::SimulateRequest { tx: None, tx_bytes })
            .await?
            .into_inner();
        let gas_used = resp.gas_info.unwrap().gas_used;
        Ok(gas_used)
    }

    /// Returns all the block info
    pub async fn block_info(&self) -> Result<cosmwasm_std::BlockInfo, DaemonError> {
        let block = self.latest_block().await?;
        let since_epoch = block.header.time.duration_since(Time::unix_epoch())?;
        let time = cosmwasm_std::Timestamp::from_nanos(since_epoch.as_nanos() as u64);
        Ok(cosmwasm_std::BlockInfo {
            height: block.header.height.value(),
            time,
            chain_id: block.header.chain_id.to_string(),
        })
    }

    /// Find TX by hash
    pub async fn find_tx_by_hash(&self, hash: String) -> Result<CosmTxResponse, DaemonError> {
        let mut client =
            cosmos_modules::tx::service_client::ServiceClient::new(self.channel.clone());

        let request = cosmos_modules::tx::GetTxRequest { hash };

        for _ in 0..MAX_TX_QUERY_RETRIES {
            match client.get_tx(request.clone()).await {
                Ok(tx) => {
                    let resp = tx.into_inner().tx_response.unwrap();
                    log::debug!("TX found: {:?}", resp);
                    return Ok(resp.into());
                }
                Err(err) => {
                    log::debug!("TX not found with error: {:?}", err);
                    log::debug!("Waiting 10s");
                    sleep(Duration::from_secs(10)).await;
                }
            }
        }

        panic!(
            "couldn't find transaction after {} attempts!",
            MAX_TX_QUERY_RETRIES
        );
    }
}
