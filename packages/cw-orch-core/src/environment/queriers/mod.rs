use cosmwasm_std::{Addr, BlockInfo};
use serde::{de::DeserializeOwned, Serialize};

use self::{bank::BankQuerierGetter, node::NodeQuerierGetter, wasm::WasmQuerierGetter};
use crate::CwEnvError;
use std::fmt::Debug;

pub mod bank;
pub mod node;
pub mod wasm;

/// This trait regroups all querier traits + adds high level interfaces to access some elements faster
pub trait QueryHandler: NodeQuerierGetter + BankQuerierGetter + WasmQuerierGetter {
    type Error: Into<CwEnvError> + Debug;

    /// Wait for an amount of blocks.
    fn wait_blocks(&self, amount: u64) -> Result<(), Self::Error>;

    /// Wait for an amount of seconds.
    fn wait_seconds(&self, secs: u64) -> Result<(), Self::Error>;

    /// Wait for next block.
    fn next_block(&self) -> Result<(), Self::Error>;

    /// Return current block info see [`BlockInfo`].
    fn block_info(&self) -> Result<BlockInfo, Self::Error>;

    /// Send a QueryMsg to a contract.
    fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, Self::Error>;
}
