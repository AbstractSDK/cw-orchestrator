//! This module creates a trait `MutCwEnv`, that allows to create tests that are generic on all testing environments.
//! This allows to set balance and the block for instance

use super::{CwEnv, TxHandler};

pub trait BankSetter: TxHandler {
    fn set_balance(coins: Vec<Coin>) -> Result<(), <Self as TxHandler>::Error>;

    fn add_balance(coins: Vec<Coin>) -> Result<(), <Self as TxHandler>::Error>;
}

pub trait MutCwEnv: BankSetter + CwEnv {}
impl<T> MutCwEnv for T where T: BankSetter + CwEnv {}
