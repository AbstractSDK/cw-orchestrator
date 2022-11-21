use cosmrs::tendermint::{Block, Time};
use tonic::transport::Channel;

use crate::{cosmos_modules, BootError};

pub(super) struct DaemonQuerier;

impl DaemonQuerier {
    pub async fn latest_block(channel: Channel) -> Result<Block, BootError> {
        let mut client = cosmos_modules::tendermint::service_client::ServiceClient::new(channel);
        #[allow(deprecated)]
        let resp = client
            .get_latest_block(cosmos_modules::tendermint::GetLatestBlockRequest {})
            .await?
            .into_inner();
        Ok(Block::try_from(resp.block.unwrap())?)
    }

    pub async fn block_height(channel: Channel) -> Result<u64, BootError> {
        let block = Self::latest_block(channel).await?;
        Ok(block.header.height.value())
    }

    /// Returns the block timestamp (since unix epoch) in nanos
    pub async fn block_time(channel: Channel) -> Result<u128, BootError> {
        let block = Self::latest_block(channel).await?;
        Ok(block
            .header
            .time
            .duration_since(Time::unix_epoch())?
            .as_nanos())
    }

    pub async fn simulate_tx(channel: Channel, tx_bytes: Vec<u8>) -> Result<u64, BootError> {
        let mut client = cosmos_modules::tx::service_client::ServiceClient::new(channel);
        #[allow(deprecated)]
        let resp = client
            .simulate(cosmos_modules::tx::SimulateRequest { tx: None, tx_bytes })
            .await?
            .into_inner();
        let gas_used = resp.gas_info.unwrap().gas_used;
        Ok(gas_used)
    }
}
