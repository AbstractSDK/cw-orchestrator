use crate::{daemon::cosmos_modules, DaemonError};
use cosmrs::tendermint::{Block, Time};
use tonic::transport::Channel;

use super::DaemonQuerier;

/// Queries the node for information
pub struct Bank {
    channel: Channel,
}

impl DaemonQuerier for Bank {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Bank {
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
}
