//! Glob (*) import me to get all the types you need to get started with cw-orch.
//!
//! ```rust
//! use cw_orch::prelude::*;
//! ```

// We don't want to re-export everything here!
// 1. Macros
// 2. Traits that need to be imported to preform actions on an interface
// 3. Traits that need to be imported to implement an interface
// 4. Objects that need to be available to implement required traits

// Contract traits
pub use crate::contract::interface_traits::{
    CallAs, ConditionalMigrate, ConditionalUpload, ContractInstance, CwOrchExecute,
    CwOrchInstantiate, CwOrchMigrate, CwOrchQuery, CwOrchUpload, ExecutableContract,
    InstantiableContract, MigratableContract, QueryableContract, Uploadable,
};

pub use cw_orch_core::contract::Deploy;

pub use crate::environment::StateInterface;
pub use crate::environment::ChainState;

// Response trait
pub use crate::environment::IndexResponse;

// Environment
pub use crate::environment::{
    BankQuerier, BankSetter, CwEnv, DefaultQueriers, EnvironmentInfo, EnvironmentQuerier,
    NodeQuerier, QuerierGetter, QueryHandler, TxHandler, TxResponse, WasmQuerier,
};

// Chains
pub use crate::environment::{ChainInfo, ChainInfoOwned};

// Mock for testing
pub use crate::mock::{Mock, MockBech32};

// error
pub use crate::error::CwOrchError;

// Paths for implementing `Uploadable`
pub use crate::contract::{ArtifactsDir, WasmPath};

// re-export as it is used in the public API
pub use crate::mock::cw_multi_test::{Contract as MockContract, ContractWrapper};
pub use cosmwasm_std::{Addr, Coin, Empty};

// builder, core type, networks mod, queriers mod, traits
#[cfg(feature = "daemon")]
pub use crate::daemon::{
    live_mock,
    queriers,
    // sync helpers
    Daemon,
    DaemonAsync,
    DaemonAsyncBuilder,
    // expose the sync variants
    DaemonBuilder,
};

#[cfg(feature = "daemon")]
pub use cw_orch_networks::networks;

pub use crate::contract::artifacts_dir_from_workspace;

pub use cw_orch_traits::*;

#[cfg(feature = "snapshot-testing")]
pub use crate::take_storage_snapshot;
