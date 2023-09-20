use cosmwasm_std::Coin;
use dialoguer::Confirm;
use crate::{environment::TxHandler, CwEnvError};

use super::Bank;

/// Allows to estimate the fee needed for transactions
pub trait FeeEstimation: TxHandler {
    // Estimates the fee needed for computation gas
    fn estimate_fee(&self, gas: u64) -> Result<Coin, <Self as TxHandler>::Error>;
}

/// Allows to verify the wallet has the right balance for submitting a tx
pub trait WalletBalanceAssertion: FeeEstimation + Bank{

    // Returns an object allowing to know if the sender has enough balance to execute the transaction
    // Usually unused
    fn _wallet_balance_assertion(&self, gas: u64) -> Result<WalletBalanceAssertionResult, <Self as TxHandler>::Error> {
        let estimated_fee = self.estimate_fee(gas)?;

        let balance = self.balance(Some(estimated_fee.denom.clone()))?;

        log::debug!(
            "Checking balance {:?}, on chain {}, for address {:?}. Expecting {:?}",
            balance.clone(),
            self.block_info()?.chain_id,
            self.sender(),
            estimated_fee.clone()
        );

        Ok(WalletBalanceAssertionResult {
            expected: estimated_fee.clone(),
            current: balance[0].clone(),
            assertion: balance[0].amount >= estimated_fee.amount,
        })
    }

    fn assert_wallet_balance(&self, gas: u64) -> Result<(), CwEnvError>{
        let result = self._wallet_balance_assertion(gas).map_err(Into::into)?;

        if result.assertion {
            log::debug!("The wallet has enough balance to deploy");
            return Ok(());
        }

        // Needs to be pushed on the daemon impl
        // log::debug!(
        //     "Checking balance {} on chain {}, address {}. Expecting {}{}",
        //     balance.amount,
        //     chain_info.chain_id,
        //     chain.sender(),
        //     fee,
        //     fee_token.denom.as_str()
        // );

        // If there is not enough asset balance, we need to warn the user
        if Confirm::new()
            .with_prompt(format!(
                "Not enough funds on chain {} at address {} to deploy the contract. 
                    Needed: {} but only have: {}.
                    Press 'y' when the wallet balance has been increased to resume deployment",
                self.block_info().map_err(Into::into)?.chain_id,
                self.sender(),
                result.expected,
                result.current
            ))
            .interact()?
        {
            // We retry asserting the balance
            self.assert_wallet_balance(gas)
        } else {
            Err(CwEnvError::NotEnoughBalance {
                expected: result.expected,
                current: result.current,
            })
        }
    }
}


impl<T: FeeEstimation + Bank> WalletBalanceAssertion for T{}

pub struct WalletBalanceAssertionResult {
    pub assertion: bool,
    pub expected: Coin,
    pub current: Coin,
}
