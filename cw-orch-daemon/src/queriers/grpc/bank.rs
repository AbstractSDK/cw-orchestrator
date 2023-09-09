use crate::{cosmos_modules, error::DaemonError};
use cosmrs::proto::cosmos::base::{query::v1beta1::PageRequest, v1beta1::Coin};
use tonic::transport::Channel;

use crate::queriers::DaemonQuerier;

/// Queries for Cosmos Bank Module
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
    pub async fn balance(
        &self,
        address: impl Into<String>,
        denom: Option<String>,
    ) -> Result<Vec<Coin>, DaemonError> {
        match denom {
            Some(denom) => {
                let resp = cosmos_query!(
                    self,
                    bank,
                    balance,
                    QueryBalanceRequest {
                        address: address.into(),
                        denom: denom,
                    }
                );
                let coin = resp.balance.unwrap();
                Ok(vec![coin])
            }
            None => {
                let resp = cosmos_query!(
                    self,
                    bank,
                    all_balances,
                    QueryAllBalancesRequest {
                        address: address.into(),
                        pagination: None,
                    }
                );

                let coins = resp.balances;
                Ok(coins.into_iter().collect())
            }
        }
    }

    /// Query spendable balance for address
    pub async fn spendable_balances(
        &self,
        address: impl Into<String>,
    ) -> Result<Vec<Coin>, DaemonError> {
        let spendable_balances: cosmos_modules::bank::QuerySpendableBalancesResponse = cosmos_query!(
            self,
            bank,
            spendable_balances,
            QuerySpendableBalancesRequest {
                address: address.into(),
                pagination: None,
            }
        );
        Ok(spendable_balances.balances)
    }

    /// Query total supply in the bank
    pub async fn total_supply(&self) -> Result<Vec<Coin>, DaemonError> {
        let total_supply: cosmos_modules::bank::QueryTotalSupplyResponse = cosmos_query!(
            self,
            bank,
            total_supply,
            QueryTotalSupplyRequest { pagination: None }
        );
        Ok(total_supply.supply)
    }

    /// Query total supply in the bank for a denom
    pub async fn supply_of(&self, denom: impl Into<String>) -> Result<Coin, DaemonError> {
        let supply_of: cosmos_modules::bank::QuerySupplyOfResponse = cosmos_query!(
            self,
            bank,
            supply_of,
            QuerySupplyOfRequest {
                denom: denom.into()
            }
        );
        Ok(supply_of.amount.unwrap())
    }

    /// Query params
    pub async fn params(&self) -> Result<cosmos_modules::bank::Params, DaemonError> {
        let params: cosmos_modules::bank::QueryParamsResponse =
            cosmos_query!(self, bank, params, QueryParamsRequest {});
        Ok(params.params.unwrap())
    }

    /// Query denom metadata
    pub async fn denom_metadata(
        &self,
        denom: impl Into<String>,
    ) -> Result<cosmos_modules::bank::Metadata, DaemonError> {
        let denom_metadata: cosmos_modules::bank::QueryDenomMetadataResponse = cosmos_query!(
            self,
            bank,
            denom_metadata,
            QueryDenomMetadataRequest {
                denom: denom.into()
            }
        );
        Ok(denom_metadata.metadata.unwrap())
    }

    /// Query denoms metadata with pagination
    ///
    /// see [PageRequest] for pagination
    pub async fn denoms_metadata(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<Vec<cosmos_modules::bank::Metadata>, DaemonError> {
        let denoms_metadata: cosmos_modules::bank::QueryDenomsMetadataResponse = cosmos_query!(
            self,
            bank,
            denoms_metadata,
            QueryDenomsMetadataRequest {
                pagination: pagination
            }
        );
        Ok(denoms_metadata.metadatas)
    }
}
