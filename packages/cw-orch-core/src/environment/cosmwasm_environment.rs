//! Transactional traits for execution environments.

use super::{ChainState, IndexResponse};
use crate::{contract::interface_traits::Uploadable, error::CwEnvError};
use cosmwasm_std::{Addr, BlockInfo, Coin};
use dialoguer::Confirm;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

/// Signals a supported execution environment for CosmWasm contracts
pub trait CwEnv: TxHandler + Clone {}
impl<T: TxHandler + Clone> CwEnv for T {}

/// Response type for actions on an environment
pub type TxResponse<Chain> = <Chain as TxHandler>::Response;

/// Signer trait for chains.
/// Accesses the sender information from the chain object to perform actions.
pub trait TxHandler: ChainState + Clone {
    /// Response type for transactions on an environment.
    type Response: IndexResponse + Debug;
    /// Error type for transactions on an environment.
    type Error: Into<CwEnvError> + Debug;
    /// Source type for uploading to the environment.
    type ContractSource;

    type Sender: Clone;

    /// Gets the address of the current wallet used to sign transactions.
    fn sender(&self) -> Addr;

    /// Sets wallet to sign transactions.
    fn set_sender(&mut self, sender: Self::Sender);

    /// Wait for an amount of blocks.
    fn wait_blocks(&self, amount: u64) -> Result<(), Self::Error>;

    /// Wait for an amount of seconds.
    fn wait_seconds(&self, secs: u64) -> Result<(), Self::Error>;

    /// Wait for next block.
    fn next_block(&self) -> Result<(), Self::Error>;

    /// Return current block info see [`BlockInfo`].
    fn block_info(&self) -> Result<BlockInfo, Self::Error>;

    // Actions

    /// Uploads a contract to the chain.
    fn upload(&self, contract_source: &impl Uploadable) -> Result<Self::Response, Self::Error>;

    /// Send a InstantiateMsg to a contract.
    fn instantiate<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[cosmwasm_std::Coin],
    ) -> Result<Self::Response, Self::Error>;
    /// Send a ExecMsg to a contract.
    fn execute<E: Serialize + Debug>(
        &self,
        exec_msg: &E,
        coins: &[Coin],
        contract_address: &Addr,
    ) -> Result<Self::Response, Self::Error>;

    /// Send a QueryMsg to a contract.
    fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, Self::Error>;

    /// Send a MigrateMsg to a contract.
    fn migrate<M: Serialize + Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
        contract_address: &Addr,
    ) -> Result<Self::Response, Self::Error>;
}

pub struct WalletBalanceAssertionResult {
    pub assertion: bool,
    pub expected: Coin,
    pub current: Coin,
}

pub trait WalletBalanceAssertion: TxHandler {
    /// An internal function to check wether the wallet has enough balance to execute an action
    fn _assert_wallet_balance(
        &self,
        gas: u64,
    ) -> Result<WalletBalanceAssertionResult, <Self as TxHandler>::Error>;

    /// The result of a bad wallet balance on the program execution
    /// You don't need to reimplement this method
    fn assert_wallet_balance(&self, gas: u64) -> Result<(), CwEnvError> {
        let result = self._assert_wallet_balance(gas).map_err(Into::into)?;

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
