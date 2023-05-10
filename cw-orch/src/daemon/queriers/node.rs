use std::time::Duration;

use crate::{
    daemon::{cosmos_modules, tx_resp::CosmTxResponse},
    DaemonError,
};

use cosmrs::{
    proto::cosmos::{base::query::v1beta1::PageRequest, tx::v1beta1::{SimulateResponse, OrderBy}},
    tendermint::{Block, Time},
};
use tokio::time::sleep;
use tonic::transport::Channel;

use super::DaemonQuerier;

const MAX_TX_QUERY_RETRIES: usize = 5;

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

        #[allow(deprecated)]
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

        #[allow(deprecated)]
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

        #[allow(deprecated)]
        let resp = client
            .get_latest_block(cosmos_modules::tendermint::GetLatestBlockRequest {})
            .await?
            .into_inner();

        Ok(Block::try_from(resp.block.unwrap())?)
    }

    /// Returns latests validator set
    pub async fn latest_validator_set(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::tendermint::GetLatestValidatorSetResponse, DaemonError> {
        let mut client =
            cosmos_modules::tendermint::service_client::ServiceClient::new(self.channel.clone());

        #[allow(deprecated)]
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

        #[allow(deprecated)]
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

    pub async fn chain_id(&self) -> Result<String, DaemonError> {
        let block = self.latest_block().await?;
        Ok(block.header.chain_id.to_string())
    }

    /// Find TX by hash
    pub async fn find_tx_by_hash(&self, hash: String) -> Result<CosmTxResponse, DaemonError> {
        let mut client =
            cosmos_modules::tx::service_client::ServiceClient::new(self.channel.clone());

        let request = cosmos_modules::tx::GetTxRequest { hash: hash.clone() };

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
        // return error if tx not found by now
        Err(DaemonError::TXNotFound(hash, MAX_TX_QUERY_RETRIES))
    }

    /// Find TX by events
    pub async fn find_tx_by_events(&self, events: Vec<String>, page: Option<PageRequest>, order_by: Option<OrderBy>) -> Result<Vec<CosmTxResponse>, DaemonError> {
        let mut client =
            cosmos_modules::tx::service_client::ServiceClient::new(self.channel.clone());

        let request = cosmos_modules::tx::GetTxsEventRequest{ 
            events: events.clone(),
            pagination: page,
            order_by: order_by.unwrap_or(OrderBy::Desc).into()
        };

        for _ in 0..MAX_TX_QUERY_RETRIES {
            match client.get_txs_event(request.clone()).await {
                Ok(tx) => {
                    let resp = tx.into_inner().tx_responses;
                    if resp.is_empty() {
                        log::debug!("Not TX by events found");
                        log::debug!("Waiting 10s");
                        sleep(Duration::from_secs(10)).await;
                    }else{
                        log::debug!("TX found by events: {:?}", resp.iter().map(|t| t.txhash.clone()));
                        return Ok(resp.iter().map(|r| r.clone().into()).collect());
                    }
                }
                Err(err) => {
                    log::debug!("TX not found with error: {:?}", err);
                    log::debug!("Waiting 10s");
                    sleep(Duration::from_secs(10)).await;
                }
            }
        }
        // return error if tx not found by now
        Err(DaemonError::TXNotFound(format!("with events {:?}", events), MAX_TX_QUERY_RETRIES))
    }
}
