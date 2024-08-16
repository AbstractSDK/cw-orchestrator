use cosmwasm_std::{Addr, Coin};

use super::Querier;

pub trait BankQuerier: Querier {
    /// Query the bank balance of a given address
    /// If denom is None, returns all balances
    fn balance(&self, address: &Addr, denom: Option<String>) -> Result<Vec<Coin>, Self::Error>;

    /// Query total supply in the bank
    fn total_supply(&self) -> Result<Vec<Coin>, Self::Error>;

    /// Query total supply in the bank for a denom
    fn supply_of(&self, denom: impl Into<String>) -> Result<Coin, Self::Error>;
}
