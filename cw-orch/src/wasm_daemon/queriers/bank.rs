use cosmos_client::client::Rpc;
use cosmrs::proto::cosmos::base::{query::v1beta1::PageRequest, v1beta1::Coin};

use crate::wasm_daemon::{cosmos_modules, error::WasmDaemonError};

use super::WasmDaemonQuerier;

/// Queries for Cosmos Bank Module
pub struct Bank {
    rpc: Rpc,
}

impl WasmDaemonQuerier for Bank {
    fn new(rpc: Rpc) -> Self {
        Self { rpc }
    }
}

impl Bank {
    /// Query the bank balance of a given address
    /// If denom is None, returns all balances
    pub async fn balance(
        &self,
        address: impl Into<String>,
        denom: Option<String>,
    ) -> Result<Vec<Coin>, WasmDaemonError> {
        let address = address.into();
        match denom {
            Some(denom) => {
                let resp = self.rpc.bank.balance(&address, &denom).await?;
                let coin = resp.balance.unwrap();
                Ok(vec![coin])
            }
            None => {
                let resp = self.rpc.bank.all_balances(&address, None).await?;
                let coins = resp.balances;
                Ok(coins.into_iter().collect())
            }
        }
    }

    /// Query spendable balance for address
    pub async fn spendable_balances(
        &self,
        address: impl Into<String>,
    ) -> Result<Vec<Coin>, WasmDaemonError> {
        let spendable_balances = self.rpc.bank.spendable_balances(&address.into(), None).await?;
        Ok(spendable_balances.balances)
    }

    /// Query total supply in the bank
    pub async fn total_supply(&self) -> Result<Vec<Coin>, WasmDaemonError> {
        let total_supply = self.rpc.bank.total_supply(None).await?;
        Ok(total_supply.supply)
    }


    /// Query total supply in the bank for a denom
    pub async fn supply_of(&self, denom: impl Into<String>) -> Result<Coin, WasmDaemonError> {
        let supply_of = self.rpc.bank.supply_of(&denom.into()).await?;
        Ok(supply_of.amount.unwrap())
    }


    /// Query params
    pub async fn params(&self) -> Result<cosmos_modules::bank::Params, WasmDaemonError> {
        let params = self.rpc.bank.params().await?;
        Ok(params.params.unwrap())
    }


    /// Query denom metadata
    pub async fn denom_metadata(
        &self,
        denom: impl Into<String>,
    ) -> Result<cosmos_modules::bank::Metadata, WasmDaemonError> {
        let denom_metadata = self.rpc.bank.denom_metadata(&denom.into()).await?;
        Ok(denom_metadata.metadata.unwrap())
    }


    /// Query denoms metadata with pagination
    ///
    /// see [PageRequest] for pagination
    pub async fn denoms_metadata(
        &self,
        pagination: Option<PageRequest>,
    ) -> Result<Vec<cosmos_modules::bank::Metadata>, WasmDaemonError> {
        let denoms_metadata = self.rpc.bank.denoms_metadata(pagination).await?;
        Ok(denoms_metadata.metadatas)
    }

}
