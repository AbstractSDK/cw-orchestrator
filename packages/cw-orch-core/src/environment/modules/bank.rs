use cosmwasm_std::Coin;

use super::fee_estimation::FeeEstimation;

/// Allows to send bank related transactions and query the on-chain bank module
pub trait Bank<Response, Error>: FeeEstimation<Error> + std::fmt::Debug {
    // Send coins to another account
    fn send(&self, funds: Vec<Coin>, receiver: Self) -> Result<Response, Error>;

    // Send coins to another account
    fn balance(&self, denom: Option<String>) -> Result<Vec<Coin>, Error>;

    fn wallet_balance_assertion(&self, gas: u64) -> Result<WalletBalanceAssertionResult, Error> {
        let estimated_fee = self.estimate_fee(gas)?;

        let balance = self.balance(Some(estimated_fee.denom.clone()))?;

        log::debug!(
            "Checking balance {:?}, address {:?}. Expecting {:?}",
            balance.clone(),
            self,
            estimated_fee.clone()
        );

        Ok(WalletBalanceAssertionResult {
            expected: estimated_fee.clone(),
            current: balance[0].clone(),
            assertion: balance[0].amount >= estimated_fee.amount,
        })
    }
}

pub struct WalletBalanceAssertionResult {
    pub assertion: bool,
    pub expected: Coin,
    pub current: Coin,
}
