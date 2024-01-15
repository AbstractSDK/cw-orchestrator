//! Transactional traits for execution environments.

use super::{queriers::QueryHandler, ChainState, IndexResponse};
use crate::{contract::interface_traits::Uploadable, error::CwEnvError};
use cosmwasm_std::{Addr, Binary, Coin};
use serde::Serialize;
use std::fmt::Debug;

/// Signals a supported execution environment for CosmWasm contracts
pub trait CwEnv: TxHandler + QueryHandler + Clone {}
impl<T: TxHandler + QueryHandler + Clone> CwEnv for T {}

/// Response type for actions on an environment
pub type TxResponse<Chain> = <Chain as TxHandler>::Response;

/// Signer trait for chains.
/// Accesses the sender information from the chain object to perform actions.
pub trait TxHandler: ChainState + Clone {
    /// Response type for transactions on an environment.
    type Response: IndexResponse + Debug + Send + Clone;
    /// Error type for transactions on an environment.
    type Error: Into<CwEnvError> + Debug + std::error::Error;
    /// Source type for uploading to the environment.
    type ContractSource;

    type Sender: Clone;

    /// Gets the address of the current wallet used to sign transactions.
    fn sender(&self) -> Addr;

    /// Sets wallet to sign transactions.
    fn set_sender(&mut self, sender: Self::Sender);

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

    /// Send a Instantiate2Msg to a contract.
    fn instantiate2<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[cosmwasm_std::Coin],
        salt: Binary,
        fix_msg: bool,
    ) -> Result<Self::Response, Self::Error>;

    /// Send a ExecMsg to a contract.
    fn execute<E: Serialize + Debug>(
        &self,
        exec_msg: &E,
        coins: &[Coin],
        contract_address: &Addr,
    ) -> Result<Self::Response, Self::Error>;

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
