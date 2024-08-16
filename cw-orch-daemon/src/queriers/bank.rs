use crate::{cosmos_modules, error::DaemonError, senders::query::QuerySender, DaemonBase};
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use cosmwasm_std::{Addr, Coin, StdError};
use cw_orch_core::environment::{BankQuerier, Querier, QuerierGetter};
use tokio::runtime::Handle;
use tonic::transport::Channel;

/// Queries for Cosmos Bank Module
/// All the async function are prefixed with `_`
pub struct Bank {
    pub channel: Channel,
    pub rt_handle: Option<Handle>,
}

impl Bank {
    pub fn new<Sender: QuerySender>(daemon: &DaemonBase<Sender>) -> Self {
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

impl Querier for Bank {
    type Error = DaemonError;
}

impl<Sender: QuerySender> QuerierGetter<Bank> for DaemonBase<Sender> {
    fn querier(&self) -> Bank {
        Bank::new(self)
    }
}

impl Bank {
    /// Query the bank balance of a given address
    /// If denom is None, returns all balances
    pub async fn _balance(
        &self,
        address: &Addr,
        denom: Option<String>,
    ) -> Result<Vec<Coin>, DaemonError> {
        use cosmos_modules::bank::query_client::QueryClient;
        match denom {
            Some(denom) => {
                let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
                let request = cosmos_modules::bank::QueryBalanceRequest {
                    address: address.to_string(),
                    denom,
                };
                let resp = client.balance(request).await?.into_inner();
                let coin = resp.balance.unwrap();
                Ok(vec![cosmrs_to_cosmwasm_coin(coin)?])
            }
            None => {
                let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
                let request = cosmos_modules::bank::QueryAllBalancesRequest {
                    address: address.to_string(),
                    ..Default::default()
                };
                let resp = client.all_balances(request).await?.into_inner();
                Ok(cosmrs_to_cosmwasm_coins(resp.balances)?)
            }
        }
    }

    /// Query spendable balance for address
    pub async fn _spendable_balances(&self, address: &Addr) -> Result<Vec<Coin>, DaemonError> {
        let spendable_balances: cosmos_modules::bank::QuerySpendableBalancesResponse = cosmos_query!(
            self,
            bank,
            spendable_balances,
            QuerySpendableBalancesRequest {
                address: address.to_string(),
                pagination: None,
            }
        );
        Ok(cosmrs_to_cosmwasm_coins(spendable_balances.balances)?)
    }

    /// Query total supply in the bank
    pub async fn _total_supply(&self) -> Result<Vec<Coin>, DaemonError> {
        let total_supply: cosmos_modules::bank::QueryTotalSupplyResponse = cosmos_query!(
            self,
            bank,
            total_supply,
            QueryTotalSupplyRequest { pagination: None }
        );
        Ok(cosmrs_to_cosmwasm_coins(total_supply.supply)?)
    }

    /// Query total supply in the bank for a denom
    pub async fn _supply_of(&self, denom: impl Into<String>) -> Result<Coin, DaemonError> {
        let supply_of: cosmos_modules::bank::QuerySupplyOfResponse = cosmos_query!(
            self,
            bank,
            supply_of,
            QuerySupplyOfRequest {
                denom: denom.into()
            }
        );
        Ok(cosmrs_to_cosmwasm_coin(supply_of.amount.unwrap())?)
    }

    /// Query params
    pub async fn _params(&self) -> Result<cosmos_modules::bank::Params, DaemonError> {
        let params: cosmos_modules::bank::QueryParamsResponse =
            cosmos_query!(self, bank, params, QueryParamsRequest {});
        Ok(params.params.unwrap())
    }

    /// Query denom metadata
    pub async fn _denom_metadata(
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
    pub async fn _denoms_metadata(
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

pub fn cosmrs_to_cosmwasm_coin(
    c: cosmrs::proto::cosmos::base::v1beta1::Coin,
) -> Result<Coin, StdError> {
    Ok(Coin {
        amount: c.amount.parse()?,
        denom: c.denom,
    })
}

pub fn cosmrs_to_cosmwasm_coins(
    c: Vec<cosmrs::proto::cosmos::base::v1beta1::Coin>,
) -> Result<Vec<Coin>, StdError> {
    c.into_iter().map(cosmrs_to_cosmwasm_coin).collect()
}
impl BankQuerier for Bank {
    fn balance(
        &self,
        address: &Addr,
        denom: Option<String>,
    ) -> Result<Vec<cosmwasm_std::Coin>, Self::Error> {
        self.rt_handle
            .as_ref()
            .ok_or(DaemonError::QuerierNeedRuntime)?
            .block_on(self._balance(address, denom))
    }

    fn total_supply(&self) -> Result<Vec<cosmwasm_std::Coin>, Self::Error> {
        self.rt_handle
            .as_ref()
            .ok_or(DaemonError::QuerierNeedRuntime)?
            .block_on(self._total_supply())
    }

    fn supply_of(&self, denom: impl Into<String>) -> Result<cosmwasm_std::Coin, Self::Error> {
        self.rt_handle
            .as_ref()
            .ok_or(DaemonError::QuerierNeedRuntime)?
            .block_on(self._supply_of(denom))
    }
}
