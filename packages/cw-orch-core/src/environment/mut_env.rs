//! This module creates a trait `MutCwEnv`, that allows to create tests that are generic on all testing environments.
//! This allows to set balance and the block for instance

use super::{BankQuerier, CwEnv, TxHandler};
use cosmwasm_std::{Addr, Coin};
use cw_utils::NativeBalance;

pub trait MutCwEnv: BankSetter + CwEnv {}

pub trait BankSetter: BankQuerier {
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
        let current_balance = self.balance(address, None)?;
        let future_balance = NativeBalance(current_balance) + NativeBalance(amount);
        // Set the balance with more funds
        self.set_balance(address, future_balance.into_vec())?;
        Ok(())
    }
}

impl<T> MutCwEnv for T where T: BankSetter + CwEnv {}
