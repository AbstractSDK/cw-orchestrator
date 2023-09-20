use cosmwasm_std::{coin, Addr, Coin};

use crate::CwEnvError;

/// Allows to estimate the fee needed for transactions
pub trait FeeEstimation<Error> {
    // Estimates the fee needed for computation gas
    fn estimate_fee(&self, gas: u64) -> Result<Coin, Error>;
}
