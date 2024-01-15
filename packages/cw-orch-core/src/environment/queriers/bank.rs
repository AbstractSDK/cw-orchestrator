use cosmwasm_std::Coin;

use crate::CwEnvError;
use std::fmt::Debug;

pub trait BankQuerierGetter<E> {
    type Querier: BankQuerier<Error = E>;
    fn bank_querier(&self) -> Self::Querier;
}
pub trait BankQuerier {
    type Error: Into<CwEnvError> + Debug;
    /// Query the bank balance of a given address
    /// If denom is None, returns all balances
    fn balance(
        &self,
        address: impl Into<String>,
        denom: Option<String>,
    ) -> Result<Vec<Coin>, Self::Error>;

    /// Query total supply in the bank
    fn total_supply(&self) -> Result<Vec<Coin>, Self::Error>;

    /// Query total supply in the bank for a denom
    fn supply_of(&self, denom: impl Into<String>) -> Result<Coin, Self::Error>;
}
