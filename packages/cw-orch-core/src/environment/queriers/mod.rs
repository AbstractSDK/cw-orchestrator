use cosmwasm_std::{Addr, BlockInfo};
use serde::{de::DeserializeOwned, Serialize};

use self::{bank::BankQuerier, env::EnvironmentQuerier, node::NodeQuerier, wasm::WasmQuerier};
use crate::CwEnvError;
use std::fmt::Debug;

pub mod bank;
pub mod env;
pub mod node;
pub mod wasm;

/// This trait acts as the high-level trait bound for supported queries on a `CwEnv` environment.
/// It also implements some high-level functionality to make it easy to access.
pub trait QueryHandler: DefaultQueriers {
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

pub trait QuerierGetter<Q: Querier> {
    fn querier(&self) -> Q;
}

pub trait Querier {
    type Error: Into<CwEnvError> + Debug;
}

pub trait DefaultQueriers:
    QuerierGetter<Self::B> + QuerierGetter<Self::W> + QuerierGetter<Self::N> + EnvironmentQuerier
{
    type B: BankQuerier;
    type W: WasmQuerier;
    type N: NodeQuerier;

    fn bank_querier(&self) -> Self::B {
        self.querier()
    }

    fn wasm_querier(&self) -> Self::W {
        self.querier()
    }

    fn node_querier(&self) -> Self::N {
        self.querier()
    }
}
