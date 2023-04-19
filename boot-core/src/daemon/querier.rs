use std::time::Duration;

use super::{cosmos_modules, tx_resp::CosmTxResponse};
use super::error::DaemonError;
use cosmrs::tendermint::{Block, Time};
use cosmwasm_std::Coin;
use tokio::time::sleep;
use tonic::transport::Channel;
pub struct DaemonQuerier;

impl DaemonQuerier {
    pub async fn latest_block(channel: Channel) -> Result<Block, DaemonError> {
        let mut client = cosmos_modules::tendermint::service_client::ServiceClient::new(channel);
        #[allow(deprecated)]
        let resp = client
            .get_latest_block(cosmos_modules::tendermint::GetLatestBlockRequest {})
            .await?
            .into_inner();
        Ok(Block::try_from(resp.block.unwrap())?)
    }

    pub async fn block_height(channel: Channel) -> Result<u64, DaemonError> {
        let block = Self::latest_block(channel).await?;
        Ok(block.header.height.value())
    }

    /// Returns the block timestamp (since unix epoch) in nanos
    #[allow(unused)]
    pub async fn block_time(channel: Channel) -> Result<u128, DaemonError> {
        let block = Self::latest_block(channel).await?;
        Ok(block
            .header
            .time
            .duration_since(Time::unix_epoch())?
            .as_nanos())
    }

    pub async fn simulate_tx(channel: Channel, tx_bytes: Vec<u8>) -> Result<u64, DaemonError> {
        let mut client = cosmos_modules::tx::service_client::ServiceClient::new(channel);
        #[allow(deprecated)]
        let resp = client
            .simulate(cosmos_modules::tx::SimulateRequest { tx: None, tx_bytes })
            .await?
            .into_inner();
        let gas_used = resp.gas_info.unwrap().gas_used;
        Ok(gas_used)
    }

    pub async fn code_id_hash(channel: Channel, code_id: u64) -> Result<String, DaemonError> {
        use cosmos_modules::cosmwasm::query_client::*;
        use cosmos_modules::cosmwasm::QueryCodeRequest;
        // query hash of code-id
        let mut client: QueryClient<Channel> = QueryClient::new(channel);
        let request = QueryCodeRequest { code_id };
        let resp = client.code(request).await?.into_inner();
        let contract_hash = resp.code_info.unwrap().data_hash;
        let on_chain_hash = base16::encode_lower(&contract_hash);
        Ok(on_chain_hash)
    }

    pub async fn contract_info(
        channel: Channel,
        address: impl Into<String>,
    ) -> Result<cosmos_modules::cosmwasm::ContractInfo, DaemonError> {
        use cosmos_modules::cosmwasm::query_client::*;
        use cosmos_modules::cosmwasm::QueryContractInfoRequest;
        // query hash of code-id
        let mut client: QueryClient<Channel> = QueryClient::new(channel);
        let request = QueryContractInfoRequest {
            address: address.into(),
        };
        let resp = client.contract_info(request).await?.into_inner();
        let contract_info = resp.contract_info.unwrap();
        Ok(contract_info)
    }

    pub async fn find_tx_by_hash(
        channel: Channel,
        hash: impl Into<String>,
    ) -> Result<CosmTxResponse, DaemonError> {
        let mut client = cosmos_modules::tx::service_client::ServiceClient::new(channel);
        let attempts = 5;
    
        let request = cosmos_modules::tx::GetTxRequest { hash: hash.into() };
    
        for _ in 0..attempts {
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
    
        panic!("couldn't find transaction after {} attempts!", attempts);
    }

    /// Query the bank balance of a given address
    /// If denom is None, returns all balances
    pub async fn coin_balance(
        channel: Channel,
        address: impl Into<String>,
        denom: Option<impl Into<String>>,
    ) -> Result<Vec<Coin>, DaemonError> {
        use cosmos_modules::bank::query_client::QueryClient;
        if let Some(denom) = denom {
            let mut client: QueryClient<Channel> = QueryClient::new(channel);
            let request = cosmos_modules::bank::QueryBalanceRequest {
                address: address.into(),
                denom: denom.into(),
            };
            let resp = client.balance(request).await?.into_inner();
            let coin = resp.balance.unwrap();
            Ok(vec![coin.into()])
        } else {
            let mut client: QueryClient<Channel> = QueryClient::new(channel);
            let request = cosmos_modules::bank::QueryAllBalancesRequest {
                address: address.into(),
                ..Default::default()
            };
            let resp = client.all_balances(request).await?.into_inner();
            let coins = resp.balances;
            Ok(coins.into_iter().map(|c| c.into()).collect())
        } 
    }

    
}
