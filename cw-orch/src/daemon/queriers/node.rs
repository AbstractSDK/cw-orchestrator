use std::{cmp::min, time::Duration};

use crate::daemon::{cosmos_modules, error::DaemonError, tx_resp::CosmTxResponse};

use cosmrs::{
    proto::cosmos::{
        base::query::v1beta1::PageRequest,
        tx::v1beta1::{OrderBy, SimulateResponse},
    },
    tendermint::{Block, Time},
};
use tonic::transport::Channel;

use super::DaemonQuerier;

const MAX_TX_QUERY_RETRIES: usize = 20;

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
    /// Returns node info
    pub async fn info(
        &self,
    ) -> Result<cosmos_modules::tendermint::GetNodeInfoResponse, DaemonError> {
        let mut client =
            cosmos_modules::tendermint::service_client::ServiceClient::new(self.channel.clone());

        let resp = client
            .get_node_info(cosmos_modules::tendermint::GetNodeInfoRequest {})
            .await?
            .into_inner();

        Ok(resp)
    }

    /// Queries node syncing
    pub async fn syncing(&self) -> Result<bool, DaemonError> {
        let mut client =
            cosmos_modules::tendermint::service_client::ServiceClient::new(self.channel.clone());

        let resp = client
            .get_syncing(cosmos_modules::tendermint::GetSyncingRequest {})
            .await?
            .into_inner();

        Ok(resp.syncing)
    }

    /// Returns latests block information
    pub async fn latest_block(&self) -> Result<Block, DaemonError> {
        let mut client =
            cosmos_modules::tendermint::service_client::ServiceClient::new(self.channel.clone());

        let resp = client
            .get_latest_block(cosmos_modules::tendermint::GetLatestBlockRequest {})
            .await?
            .into_inner();

        Ok(Block::try_from(resp.block.unwrap())?)
    }

    /// Returns block information fetched by height
    pub async fn block_by_height(&self, height: u64) -> Result<Block, DaemonError> {
        let mut client =
            cosmos_modules::tendermint::service_client::ServiceClient::new(self.channel.clone());

        let resp = client
            .get_block_by_height(cosmos_modules::tendermint::GetBlockByHeightRequest {
                height: height as i64,
            })
            .await?
            .into_inner();

        Ok(Block::try_from(resp.block.unwrap())?)
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

        Ok(average_block_time)
    }

    /// Returns latests validator set
    pub async fn latest_validator_set(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::tendermint::GetLatestValidatorSetResponse, DaemonError> {
        let mut client =
            cosmos_modules::tendermint::service_client::ServiceClient::new(self.channel.clone());

        let resp = client
            .get_latest_validator_set(cosmos_modules::tendermint::GetLatestValidatorSetRequest {
                pagination,
            })
            .await?
            .into_inner();

        Ok(resp)
    }

    /// Returns latests validator set fetched by height
    pub async fn validator_set_by_height(
        &self,
        height: i64,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::tendermint::GetValidatorSetByHeightResponse, DaemonError> {
        let mut client =
            cosmos_modules::tendermint::service_client::ServiceClient::new(self.channel.clone());

        let resp = client
            .get_validator_set_by_height(
                cosmos_modules::tendermint::GetValidatorSetByHeightRequest { height, pagination },
            )
            .await?
            .into_inner();

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
        let mut client =
            cosmos_modules::tx::service_client::ServiceClient::new(self.channel.clone());
        #[allow(deprecated)]
        let resp: SimulateResponse = client
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

    /// Returns the chain id by querying it from the latest block
    pub async fn chain_id(&self) -> Result<String, DaemonError> {
        let block = self.latest_block().await?;
        Ok(block.header.chain_id.to_string())
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
        let mut client =
            cosmos_modules::tx::service_client::ServiceClient::new(self.channel.clone());

        let request = cosmos_modules::tx::GetTxRequest { hash: hash.clone() };
        let block_speed = self.average_block_speed(Some(0.7)).await?.max(1);

        for _ in 0..retries {
            match client.get_tx(request.clone()).await {
                Ok(tx) => {
                    let resp = tx.into_inner().tx_response.unwrap();
                    log::debug!("TX found: {:?}", resp);
                    return Ok(resp.into());
                }
                Err(err) => {
                    log::debug!("TX not found with error: {:?}", err);
                    log::debug!("Waiting {block_speed} seconds");
                    tokio::time::sleep(Duration::from_secs(block_speed)).await;
                }
            }
        }

        // return error if tx not found by now
        Err(DaemonError::TXNotFound(hash, retries))
    }

    /// Find TX by events
    pub async fn find_tx_by_events(
        &self,
        events: Vec<String>,
        page: Option<u64>,
        order_by: Option<OrderBy>,
    ) -> Result<Vec<CosmTxResponse>, DaemonError> {
        self.find_tx_by_events_with_retries(events, page, order_by, false, MAX_TX_QUERY_RETRIES)
            .await
    }

    /// Find Tx by events and waits for until there is a non-empty response
    pub async fn find_some_tx_by_events(
        &self,
        events: Vec<String>,
        page: Option<u64>,
        order_by: Option<OrderBy>,
    ) -> Result<Vec<CosmTxResponse>, DaemonError> {
        self.find_tx_by_events_with_retries(events, page, order_by, true, MAX_TX_QUERY_RETRIES)
            .await
    }

    /// Find TX by events with  :
    /// 1. Specify if an empty tx object is a valid response
    /// 2. Specify a given amount of retries
    pub async fn find_tx_by_events_with_retries(
        &self,
        events: Vec<String>,
        page: Option<u64>,
        order_by: Option<OrderBy>,
        retry_on_empty: bool,
        retries: usize,
    ) -> Result<Vec<CosmTxResponse>, DaemonError> {
        let mut client =
            cosmos_modules::tx::service_client::ServiceClient::new(self.channel.clone());

        #[allow(deprecated)]
        let request = cosmos_modules::tx::GetTxsEventRequest {
            events: events.clone(),
            page: page.unwrap_or(0),
            limit: 100,
            pagination: None, // This is not used, so good.
            order_by: order_by.unwrap_or(OrderBy::Desc).into(),
        };

        for _ in 0..retries {
            match client.get_txs_event(request.clone()).await {
                Ok(tx) => {
                    let resp = tx.into_inner().tx_responses;
                    if retry_on_empty && resp.is_empty() {
                        log::debug!("Not TX by events found");
                        log::debug!("Waiting 10s");
                        tokio::time::sleep(Duration::from_secs(10)).await;
                    } else {
                        log::debug!(
                            "TX found by events: {:?}",
                            resp.iter().map(|t| t.txhash.clone())
                        );
                        return Ok(resp.iter().map(|r| r.clone().into()).collect());
                    }
                }
                Err(err) => {
                    log::debug!("TX not found with error: {:?}", err);
                    log::debug!("Waiting 10s");
                    tokio::time::sleep(Duration::from_secs(10)).await;
                }
            }
        }
        // return error if tx not found by now
        Err(DaemonError::TXNotFound(
            format!("with events {:?}", events),
            MAX_TX_QUERY_RETRIES,
        ))
    }
}
