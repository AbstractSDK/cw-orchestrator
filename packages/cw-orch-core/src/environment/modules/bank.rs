use cosmwasm_std::{Coin, Addr};

use crate::environment::TxHandler;

/// Allows to send bank related transactions and query the on-chain bank module
pub trait Bank: TxHandler {
    // Send coins to another account
    fn send(&self, recipient: Addr, funds: Vec<Coin>) -> Result<<Self as TxHandler>::Response, <Self as TxHandler>::Error>;

    // Send coins to another account
    fn balance(&self, denom: Option<String>) -> Result<Vec<Coin>, <Self as TxHandler>::Error>;

}
