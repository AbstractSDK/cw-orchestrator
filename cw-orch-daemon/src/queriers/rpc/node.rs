use std::{cmp::min, time::Duration};

use cosmrs::{
    proto::cosmos::base::query::v1beta1::PageRequest,
    rpc::{Client, HttpClient}, tendermint::{Block, Time},
};
use cosmrs::tendermint::block::Height;
use cosmrs::tendermint::node;

use crate::{cosmos_modules, cosmos_rpc_query, error::DaemonError, queriers::{DaemonQuerier, MAX_TX_QUERY_RETRIES}, tx_resp::CosmTxResponse};

/// Querier for the Tendermint node.
/// Supports queries for block and tx information
/// @TODO: all tendermint queries should use the tendermint-rpc explicitly instead of hitting the tendermint node with the typed queries.
pub struct Node {
    client: HttpClient,
}

impl DaemonQuerier for Node {
    fn new(client: HttpClient) -> Self {
        Self { client }
    }
}


impl Node {
    /// Returns node info
    pub async fn info(
        &self,
    ) -> Result<node::Info, DaemonError> {

        let stats = self.client.status().await?;

        Ok(stats.node_info)
    }

    /// Queries node syncing
    pub async fn syncing(&self) -> Result<bool, DaemonError> {

        let resp = cosmos_rpc_query!(
            self,
            tendermint,
            "/cosmos.base.tendermint.v1beta1.Service/GetSyncing",
            GetSyncingRequest {},
            GetSyncingResponse,
        );

        Ok(resp.syncing)
    }

    /// Returns latest block information
    pub async fn latest_block(&self) -> Result<Block, DaemonError> {
        let resp = self.client.latest_block().await?;

        Ok(resp.block)
    }

    /// Returns block information fetched by height
    pub async fn block_by_height(&self, height: u64) -> Result<Block, DaemonError> {
        let resp = self.client.block(Height::try_from(height).unwrap()).await?;

        Ok(resp.block)
    }

    /// Return the average block time for the last 50 blocks or since inception
    /// This is used to estimate the time when a tx will be included in a block
    pub async fn average_block_speed(&self, multiplier: Option<f32>) -> Result<u64, DaemonError> {
        // get latest block time and height
        let mut latest_block = self.latest_block().await?;
        let latest_block_time = latest_block.header.time;
        let mut latest_block_height = latest_block.header.height.value();

        while latest_block_height <= 1 {
            // wait to get some blocks
            tokio::time::sleep(Duration::from_secs(1)).await;
            latest_block = self.latest_block().await?;
            latest_block_height = latest_block.header.height.value();
        }

        // let avg period
        let avg_period = min(latest_block_height - 1, 50);

        // get block time for block avg_period blocks ago
        let block_avg_period_ago = self
            .block_by_height(latest_block_height - avg_period)
            .await?;
        let block_avg_period_ago_time = block_avg_period_ago.header.time;

        // calculate average block time
        let average_block_time = latest_block_time.duration_since(block_avg_period_ago_time)?;
        let average_block_time = average_block_time.as_secs() / avg_period;

        // multiply by multiplier if provided
        let average_block_time = match multiplier {
            Some(multiplier) => (average_block_time as f32 * multiplier) as u64,
            None => average_block_time,
        };

        Ok(std::cmp::max(average_block_time, 1))
    }

    /// Returns latests validator set
    pub async fn latest_validator_set(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::tendermint::GetLatestValidatorSetResponse, DaemonError> {

        let resp = cosmos_rpc_query!(
            self,
            tendermint,
            "/cosmos.base.tendermint.v1beta1.Service/GetLatestValidatorSet",
            GetLatestValidatorSetRequest {
                pagination: pagination,
            },
            GetLatestValidatorSetResponse,
        );

        Ok(resp)
    }

    /// Returns latests validator set fetched by height
    pub async fn validator_set_by_height(
        &self,
        height: i64,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::tendermint::GetValidatorSetByHeightResponse, DaemonError> {

        let resp = cosmos_rpc_query!(
            self,
            tendermint,
            "/cosmos.base.tendermint.v1beta1.Service/GetValidatorSetByHeight",
            GetValidatorSetByHeightRequest { height: height, pagination: pagination },
            GetValidatorSetByHeightResponse,
        );

        Ok(resp)
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
        log::debug!("Simulating transaction");

        // We use this allow deprecated for the tx field of the simulate request (but we set it to None, so that's ok)
        #[allow(deprecated)]
        let resp = cosmos_rpc_query!(
            self,
            tx,
            "/cosmos.tx.v1beta1.Service/Simulate",
            SimulateRequest { tx: None, tx_bytes: tx_bytes },
            SimulateResponse,
        );

        let gas_used = resp.gas_info.unwrap().gas_used;

        log::debug!("Gas used in simulation: {:?}", gas_used);
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
    pub async fn find_tx(&self, hash: String) -> Result<CosmTxResponse, DaemonError> {
        self.find_tx_with_retries(hash, MAX_TX_QUERY_RETRIES).await
    }

    /// Find TX by hash with a given amount of retries
    pub async fn find_tx_with_retries(
        &self,
        hash: String,
        retries: usize,
    ) -> Result<CosmTxResponse, DaemonError> {

        let mut block_speed = self.average_block_speed(Some(0.7)).await?;

        let hexed_hash = hex::decode(hash.clone())?.try_into().unwrap();

        for _ in 0..retries {
            let tx_r = self.client.tx(hexed_hash, false).await;

            match tx_r {
                Ok(tx) => {
                    log::debug!("TX found: {:?}", tx);
                    return Ok(tx.into());
                }
                Err(err) => {
                    // increase wait time
                    block_speed = (block_speed as f64 * 1.6) as u64;
                    log::debug!("TX not found with error: {:?}", err);
                    log::debug!("Waiting {block_speed} seconds");
                    tokio::time::sleep(Duration::from_secs(block_speed)).await;
                }
            }
        }

        // return error if tx not found by now
        Err(DaemonError::TXNotFound(hash, retries))
    }
}
