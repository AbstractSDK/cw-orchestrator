//! Transactional traits for execution environments.

use super::{ChainState, IndexResponse};
use crate::{
    contract::interface_traits::{ContractInstance, Uploadable},
    error::CwEnvError,
};
use cosmwasm_std::{Addr, BlockInfo, Coin, ContractInfoResponse};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

/// Signals a supported execution environment for CosmWasm contracts
pub trait CwEnv: TxHandler + BankQuerier + WasmCodeQuerier + Clone {}
impl<T: TxHandler + BankQuerier + WasmCodeQuerier + Clone> CwEnv for T {}

/// Response type for actions on an environment
pub type TxResponse<Chain> = <Chain as TxHandler>::Response;

/// Signer trait for chains.
/// Accesses the sender information from the chain object to perform actions.
pub trait TxHandler: ChainState + Clone {
    /// Response type for transactions on an environment.
    type Response: IndexResponse + Debug + Send + Clone;
    /// Error type for transactions on an environment.
    type Error: Into<CwEnvError> + Debug + std::error::Error + Send + Sync + 'static;
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

    /// Clones the chain with a different sender.
    /// Usually used to call a contract as a different sender.
    fn call_as(&self, sender: &<Self as TxHandler>::Sender) -> Self {
        let mut chain = self.clone();
        chain.set_sender(sender.clone());
        chain
    }
}

pub trait WasmCodeQuerier: TxHandler + Clone {
    /// Returns the checksum of provided code_id
    fn contract_hash(&self, code_id: u64) -> Result<String, <Self as TxHandler>::Error>;
    /// Returns the code_info structure of the provided contract
    fn contract_info<T: ContractInstance<Self>>(
        &self,
        contract: &T,
    ) -> Result<ContractInfoResponse, <Self as TxHandler>::Error>;

    /// Returns the checksum of the WASM file if the env supports it. Will re-upload every time if not supported.
    fn local_hash<T: Uploadable + ContractInstance<Self>>(
        &self,
        contract: &T,
    ) -> Result<String, CwEnvError> {
        contract.wasm().checksum()
    }
}

pub trait BankQuerier: TxHandler {
    /// Query the bank balance of a given address
    /// If denom is None, returns all balances
    fn balance(
        &self,
        address: impl Into<String>,
        denom: Option<String>,
    ) -> Result<Vec<Coin>, <Self as TxHandler>::Error>;

    /// Query total supply in the bank for a denom
    fn supply_of(&self, denom: impl Into<String>) -> Result<Coin, <Self as TxHandler>::Error>;
}
