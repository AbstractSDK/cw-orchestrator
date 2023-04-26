use crate::{daemon::cosmos_modules, DaemonError};
use cosmrs::proto::cosmos::base::{query::v1beta1::PageRequest, v1beta1::Coin};
use tonic::transport::Channel;

use super::DaemonQuerier;

/// Queries for CosmWasm Bank Module
pub struct Bank {
    channel: Channel,
}

impl DaemonQuerier for Bank {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Bank {
    /// Query the bank balance of a given address
    /// If denom is None, returns all balances
    pub async fn coin_balance(
        &self,
        address: impl Into<String>,
        denom: Option<impl Into<String>>,
    ) -> Result<Vec<Coin>, DaemonError> {
        use cosmos_modules::bank::query_client::QueryClient;
        match denom {
            Some(denom) => {
                let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
                let request = cosmos_modules::bank::QueryBalanceRequest {
                    address: address.into(),
                    denom: denom.into(),
                };
                let resp = client.balance(request).await?.into_inner();
                let coin = resp.balance.unwrap();
                Ok(vec![coin])
            }
            None => {
                let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
                let request = cosmos_modules::bank::QueryAllBalancesRequest {
                    address: address.into(),
                    ..Default::default()
                };
                let resp = client.all_balances(request).await?.into_inner();
                let coins = resp.balances;
                Ok(coins.into_iter().collect())
            }
        }
    }

    /// Query spendable balance for address
    pub async fn spendable_balances(
        &self,
        address: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::bank::QuerySpendableBalancesResponse, DaemonError> {
        let spendable_balances: cosmos_modules::bank::QuerySpendableBalancesResponse = cosmos_query!(
            self,
            bank,
            spendable_balances,
            QuerySpendableBalancesRequest {
                address: address.into(),
                pagination: pagination,
            }
        );
        Ok(spendable_balances)
    }

    /// Query total supply in the bank
    pub async fn total_supply(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::bank::QueryTotalSupplyResponse, DaemonError> {
        let total_supply: cosmos_modules::bank::QueryTotalSupplyResponse = cosmos_query!(
            self,
            bank,
            total_supply,
            QueryTotalSupplyRequest {
                pagination: pagination,
            }
        );
        Ok(total_supply)
    }

    /// Query total supply in the bank for a denom
    pub async fn supply_of(
        &self,
        denom: impl Into<String>,
    ) -> Result<cosmos_modules::bank::QuerySupplyOfResponse, DaemonError> {
        let supply_of: cosmos_modules::bank::QuerySupplyOfResponse = cosmos_query!(
            self,
            bank,
            supply_of,
            QuerySupplyOfRequest {
                denom: denom.into()
            }
        );
        Ok(supply_of)
    }

    /// Query params
    pub async fn params(&self) -> Result<cosmos_modules::bank::QueryParamsResponse, DaemonError> {
        let params: cosmos_modules::bank::QueryParamsResponse =
            cosmos_query!(self, bank, params, QueryParamsRequest {});
        Ok(params)
    }

    /// Query denom metadata
    pub async fn denom_metadata(
        &self,
        denom: impl Into<String>,
    ) -> Result<cosmos_modules::bank::QueryDenomMetadataResponse, DaemonError> {
        let denom_metadata: cosmos_modules::bank::QueryDenomMetadataResponse = cosmos_query!(
            self,
            bank,
            denom_metadata,
            QueryDenomMetadataRequest {
                denom: denom.into()
            }
        );
        Ok(denom_metadata)
    }

    /// Query denoms metadata
    pub async fn denoms_metadata(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::bank::QueryDenomsMetadataResponse, DaemonError> {
        let denoms_metadata: cosmos_modules::bank::QueryDenomsMetadataResponse = cosmos_query!(
            self,
            bank,
            denoms_metadata,
            QueryDenomsMetadataRequest {
                pagination: pagination
            }
        );
        Ok(denoms_metadata)
    }
}
