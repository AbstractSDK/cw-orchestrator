use std::{cmp::min, time::Duration};

use crate::{cosmos_modules, error::DaemonError, tx_resp::CosmTxResponse, Daemon};

use cosmrs::{
    proto::cosmos::{
        base::query::v1beta1::PageRequest,
        tx::v1beta1::{OrderBy, SimulateResponse},
    },
    tendermint::{Block, Time},
};
use cosmwasm_std::BlockInfo;
use cw_orch_core::{
    environment::{NodeQuerier, Querier, QuerierGetter},
    log::query_target,
    CwOrchEnvVars,
};
use tokio::runtime::Handle;
use tonic::transport::Channel;

/// Querier for the Tendermint node.
/// Supports queries for block and tx information
/// All the async function are prefixed with `_`
pub struct DaemonNodeQuerier {
    channel: Channel,
    rt_handle: Option<Handle>,
}

impl DaemonNodeQuerier {
    pub fn new(daemon: &Daemon) -> Self {
        Self {
            channel: daemon.channel(),
            rt_handle: Some(daemon.rt_handle.clone()),
        }
    }
    pub fn new_async(channel: Channel) -> Self {
        Self {
            channel,
            rt_handle: None,
        }
    }
}

impl QuerierGetter<DaemonNodeQuerier> for Daemon {
    fn querier(&self) -> DaemonNodeQuerier {
        DaemonNodeQuerier::new(self)
    }
}

impl Querier for DaemonNodeQuerier {
    type Error = DaemonError;
}

impl DaemonNodeQuerier {
    /// Returns node info
    pub async fn _info(
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
    pub async fn _syncing(&self) -> Result<bool, DaemonError> {
        let mut client =
            cosmos_modules::tendermint::service_client::ServiceClient::new(self.channel.clone());

        let resp = client
            .get_syncing(cosmos_modules::tendermint::GetSyncingRequest {})
            .await?
            .into_inner();

        Ok(resp.syncing)
    }

    /// Returns latests block information
    pub async fn _latest_block(&self) -> Result<Block, DaemonError> {
        let mut client =
            cosmos_modules::tendermint::service_client::ServiceClient::new(self.channel.clone());

        let resp = client
            .get_latest_block(cosmos_modules::tendermint::GetLatestBlockRequest {})
            .await?
            .into_inner();

        Ok(Block::try_from(resp.block.unwrap())?)
    }

    /// Returns block information fetched by height
    pub async fn _block_by_height(&self, height: u64) -> Result<Block, DaemonError> {
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
    pub async fn _average_block_speed(&self, multiplier: Option<f32>) -> Result<u64, DaemonError> {
        // get latest block time and height
        let mut latest_block = self._latest_block().await?;
        let latest_block_time = latest_block.header.time;
        let mut latest_block_height = latest_block.header.height.value();

        while latest_block_height <= 1 {
            // wait to get some blocks
            tokio::time::sleep(Duration::from_secs(1)).await;
            latest_block = self._latest_block().await?;
            latest_block_height = latest_block.header.height.value();
        }

        // let avg period
        let avg_period = min(latest_block_height - 1, 50);

        // get block time for block avg_period blocks ago
        let block_avg_period_ago = self
            ._block_by_height(latest_block_height - avg_period)
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
    pub async fn _latest_validator_set(
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
    pub async fn _validator_set_by_height(
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
    pub async fn _block_height(&self) -> Result<u64, DaemonError> {
        let block = self._latest_block().await?;
        Ok(block.header.height.value())
    }

    /// Returns the block timestamp (since unix epoch) in nanos
    pub async fn _block_time(&self) -> Result<u128, DaemonError> {
        let block = self._latest_block().await?;
        Ok(block
            .header
            .time
            .duration_since(Time::unix_epoch())?
            .as_nanos())
    }

    /// Simulate TX
    pub async fn _simulate_tx(&self, tx_bytes: Vec<u8>) -> Result<u64, DaemonError> {
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
    pub async fn _block_info(&self) -> Result<cosmwasm_std::BlockInfo, DaemonError> {
        let block = self._latest_block().await?;

        block_to_block_info(block)
    }

    /// Find TX by hash
    pub async fn _find_tx(&self, hash: String) -> Result<CosmTxResponse, DaemonError> {
        self._find_tx_with_retries(hash, CwOrchEnvVars::load()?.max_tx_query_retries)
            .await
    }

    /// Find TX by hash with a given amount of retries
    pub async fn _find_tx_with_retries(
        &self,
        hash: String,
        retries: usize,
    ) -> Result<CosmTxResponse, DaemonError> {
        let mut client =
            cosmos_modules::tx::service_client::ServiceClient::new(self.channel.clone());

        let request = cosmos_modules::tx::GetTxRequest { hash: hash.clone() };
        let mut block_speed = self._average_block_speed(Some(0.7)).await?;
        block_speed = block_speed.max(CwOrchEnvVars::load()?.min_block_speed);

        for _ in 0..retries {
            match client.get_tx(request.clone()).await {
                Ok(tx) => {
                    let resp = tx.into_inner().tx_response.unwrap();
                    log::debug!(target: &query_target(), "TX found: {:?}", resp);
                    return Ok(resp.into());
                }
                Err(err) => {
                    // increase wait time
                    block_speed = (block_speed as f64 * 1.6) as u64;
                    log::debug!(target: &query_target(), "TX not found with error: {:?}", err);
                    log::debug!(target: &query_target(), "Waiting {block_speed} seconds");
                    tokio::time::sleep(Duration::from_secs(block_speed)).await;
                }
            }
        }

        // return error if tx not found by now
        Err(DaemonError::TXNotFound(hash, retries))
    }

    /// Find TX by events
    pub async fn _find_tx_by_events(
        &self,
        events: Vec<String>,
        page: Option<u64>,
        order_by: Option<OrderBy>,
    ) -> Result<Vec<CosmTxResponse>, DaemonError> {
        self._find_tx_by_events_with_retries(
            events,
            page,
            order_by,
            false,
            CwOrchEnvVars::load()?.max_tx_query_retries,
        )
        .await
    }

    /// Find Tx by events
    /// This function will consider that no transactions found is an error
    /// This either returns a non empty vector or errors
    pub async fn _find_some_tx_by_events(
        &self,
        events: Vec<String>,
        page: Option<u64>,
        order_by: Option<OrderBy>,
    ) -> Result<Vec<CosmTxResponse>, DaemonError> {
        self._find_tx_by_events_with_retries(
            events,
            page,
            order_by,
            true,
            CwOrchEnvVars::load()?.max_tx_query_retries,
        )
        .await
    }

    /// Find TX by events with  :
    /// 1. Specify if an empty tx object is a valid response
    /// 2. Specify a given amount of retries
    pub async fn _find_tx_by_events_with_retries(
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
                        log::debug!(target: &query_target(), "No TX found with events {:?}", events);
                        log::debug!(target: &query_target(), "Waiting 10s");
                        tokio::time::sleep(Duration::from_secs(10)).await;
                    } else {
                        log::debug!(
                            target: &query_target(),
                            "TX found by events: {:?}",
                            resp.iter().map(|t| t.txhash.clone())
                        );
                        return Ok(resp.iter().map(|r| r.clone().into()).collect());
                    }
                }
                Err(err) => {
                    log::debug!(target: &query_target(), "TX not found with error: {:?}", err);
                    log::debug!(target: &query_target(), "Waiting 10s");
                    tokio::time::sleep(Duration::from_secs(10)).await;
                }
            }
        }
        // return error if tx not found by now
        Err(DaemonError::TXNotFound(
            format!("with events {:?}", events),
            CwOrchEnvVars::load()?.max_tx_query_retries,
        ))
    }
}

// Now we define traits

impl NodeQuerier for DaemonNodeQuerier {
    type Response = CosmTxResponse;

    fn latest_block(&self) -> Result<cosmwasm_std::BlockInfo, Self::Error> {
        self.rt_handle
            .as_ref()
            .ok_or(DaemonError::QuerierNeedRuntime)?
            .block_on(self._block_info())
    }

    fn block_by_height(&self, height: u64) -> Result<cosmwasm_std::BlockInfo, Self::Error> {
        let block = self
            .rt_handle
            .as_ref()
            .ok_or(DaemonError::QuerierNeedRuntime)?
            .block_on(self._block_by_height(height))?;

        block_to_block_info(block)
    }

    fn block_height(&self) -> Result<u64, Self::Error> {
        self.rt_handle
            .as_ref()
            .ok_or(DaemonError::QuerierNeedRuntime)?
            .block_on(self._block_height())
    }

    fn block_time(&self) -> Result<u128, Self::Error> {
        self.rt_handle
            .as_ref()
            .ok_or(DaemonError::QuerierNeedRuntime)?
            .block_on(self._block_time())
    }

    fn simulate_tx(&self, tx_bytes: Vec<u8>) -> Result<u64, Self::Error> {
        self.rt_handle
            .as_ref()
            .ok_or(DaemonError::QuerierNeedRuntime)?
            .block_on(self._simulate_tx(tx_bytes))
    }

    fn find_tx(&self, hash: String) -> Result<Self::Response, Self::Error> {
        self.rt_handle
            .as_ref()
            .ok_or(DaemonError::QuerierNeedRuntime)?
            .block_on(self._find_tx(hash))
    }
}

fn block_to_block_info(block: Block) -> Result<BlockInfo, DaemonError> {
    let since_epoch = block.header.time.duration_since(Time::unix_epoch())?;
    let time = cosmwasm_std::Timestamp::from_nanos(since_epoch.as_nanos() as u64);
    Ok(cosmwasm_std::BlockInfo {
        height: block.header.height.value(),
        time,
        chain_id: block.header.chain_id.to_string(),
    })
}
