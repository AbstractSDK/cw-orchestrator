//! This module creates a trait `MutCwEnv`, that allows to create tests that are generic on all testing environments.
//! This allows to set balance and the block for instance

use super::{
    queriers::{bank::BankQuerier, QuerierGetter},
    QueryHandler, TxHandler,
};
use cosmwasm_std::{Addr, Coin};
use cw_utils::NativeBalance;

/// Describes a structure that contains an underlying execution environment
pub trait Environment<Chain> {
    /// Returns the chain that this structure executes on
    fn environment(&self) -> &Chain;
}

/// Signals a supported execution environment for CosmWasm contracts
pub trait CwEnv: TxHandler + QueryHandler + Clone {}
impl<T: TxHandler + QueryHandler + Clone> CwEnv for T {}

pub trait MutCwEnv: BankSetter + CwEnv {}
impl<T> MutCwEnv for T where T: BankSetter + CwEnv {}

pub trait BankSetter: TxHandler + QuerierGetter<Self::T> {
    type T: BankQuerier<Error = Self::Error>;

    fn set_balance(
        &mut self,
        address: &Addr,
        amount: Vec<Coin>,
    ) -> Result<(), <Self as TxHandler>::Error>;

    fn add_balance(
        &mut self,
        address: &Addr,
        amount: Vec<Coin>,
    ) -> Result<(), <Self as TxHandler>::Error> {
        // Query the current balance of the account
        let current_balance = QuerierGetter::<Self::T>::querier(self).balance(address, None)?;
        let future_balance = NativeBalance(current_balance) + NativeBalance(amount);
        // Set the balance with more funds
        self.set_balance(address, future_balance.into_vec())?;
        Ok(())
    }
}
