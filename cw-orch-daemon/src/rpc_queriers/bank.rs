use crate::{cosmos_modules, error::DaemonError, cosmos_rpc_query};
use cosmrs::{proto::cosmos::base::{query::v1beta1::PageRequest, v1beta1::Coin}, rpc::HttpClient, tx::MessageExt};

use super::RpcQuerier;

/// Queries for Cosmos Bank Module
pub struct Bank {
    client: HttpClient,
}

impl RpcQuerier for Bank {
    fn new(rpc: String) -> Self {
        Self { client: HttpClient::new(rpc.as_str()).unwrap() }
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
                let balance_response = cosmos_rpc_query!(
                    self,
                    bank,
                    "/cosmos.bank.v1beta1.Query/Balance",
                    QueryBalanceRequest {
                        address: address.into(),
                        denom: denom,
                    },
                    QueryBalanceResponse,
                );
                let coin = balance_response.balance.unwrap();
                Ok(vec![coin])
            }
            None => {
                let balance_response = cosmos_rpc_query!(
                    self,
                    bank,
                    "/cosmos.bank.v1beta1.Query/AllBalances",
                    QueryAllBalancesRequest {
                        address: address.into(),
                        pagination: None,
                    },
                    QueryAllBalancesResponse,
                );

                let coins = balance_response.balances;
                Ok(coins.into_iter().collect())
            }
        }
    }

    /// Query spendable balance for address
    pub async fn spendable_balances(
        &self,
        address: impl Into<String>,
    ) -> Result<Vec<Coin>, DaemonError> {
        let spendable_balances = cosmos_rpc_query!(
            self,
            bank,
            "/cosmos.bank.v1beta1.Query/SpendableBalances",
            QuerySpendableBalancesRequest {
                address: address.into(),
                pagination: None,
            },
            QuerySpendableBalancesResponse,
        );
        Ok(spendable_balances.balances)
    }

    /// Query total supply in the bank
    pub async fn total_supply(&self) -> Result<Vec<Coin>, DaemonError> {
        let total_supply = cosmos_rpc_query!(
            self,
            bank,
            "/cosmos.bank.v1beta1.Query/TotalSupply",
            QueryTotalSupplyRequest { pagination: None },
            QueryTotalSupplyResponse,
        );
        Ok(total_supply.supply)
    }

    /// Query total supply in the bank for a denom
    pub async fn supply_of(&self, denom: impl Into<String>) -> Result<Coin, DaemonError> {
        let supply_of = cosmos_rpc_query!(
            self,
            bank,
            "/cosmos.bank.v1beta1.Query/SupplyOf",
            QuerySupplyOfRequest {
                denom: denom.into()
            },
            QuerySupplyOfResponse,
        );
        Ok(supply_of.amount.unwrap())
    }

    /// Query params
    pub async fn params(&self) -> Result<cosmos_modules::bank::Params, DaemonError> {
        let params = 
            cosmos_rpc_query!(self, bank, "/cosmos.bank.v1beta1.Query/Params", QueryParamsRequest {}, QueryParamsResponse,);
        Ok(params.params.unwrap())
    }

    /// Query denom metadata
    pub async fn denom_metadata(
        &self,
        denom: impl Into<String>,
    ) -> Result<cosmos_modules::bank::Metadata, DaemonError> {
        let denom_metadata = cosmos_rpc_query!(
            self,
            bank,
            "/cosmos.bank.v1beta1.Query/DenomMetadata",
            QueryDenomMetadataRequest {
                denom: denom.into()
            },
            QueryDenomMetadataResponse,
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
        let denoms_metadata = cosmos_rpc_query!(
            self,
            bank,
            "/cosmos.bank.v1beta1.Query/DenomsMetadata",
            QueryDenomsMetadataRequest {
                pagination: pagination
            },
            QueryDenomsMetadataResponse,      
        );
        Ok(denoms_metadata.metadatas)
    }
}
