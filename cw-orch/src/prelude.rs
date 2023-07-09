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
pub use crate::interface_traits::{
    CallAs, ContractInstance, CwOrchExecute, CwOrchInstantiate, CwOrchMigrate, CwOrchQuery,
    CwOrchUpload, ExecutableContract, InstantiableContract, MigratableContract, QueryableContract,
    Uploadable,
};

pub use crate::state::StateInterface;

// Response trait
pub use crate::index_response::IndexResponse;

// Environment
pub use crate::environment::{CwEnv, TxHandler, TxResponse};

// Mock for testing
pub use crate::mock::Mock;

// error
pub use crate::error::CwOrchError;

// Paths for implementing `Uploadable`
pub use crate::paths::{ArtifactsDir, WasmPath};

// re-export as it is used in the public API
pub use cosmwasm_std::{Addr, Coin, Empty};
pub use cw_multi_test::{Contract as MockContract, ContractWrapper};

// builder, core type, networks mod, queriers mod, traits
#[cfg(feature = "daemon")]
pub use crate::daemon::{
    networks,
    queriers,
    // sync helpers
    ConditionalMigrate,
    ConditionalUpload,
    Daemon,
    DaemonAsync,
    DaemonAsyncBuilder,
    // expose the sync variants
    DaemonBuilder,
};

// Remote mock
#[cfg(feature = "daemon")]
pub use crate::remote_mock::RemoteMock;

/// Re-export trait and data required to fetch daemon data from chain-registry
#[cfg(feature = "daemon")]
pub use ibc_chain_registry::{chain::ChainData as ChainRegistryData, fetchable::Fetchable};

pub use crate::paths::from_workspace as artifacts_dir_from_workspace;
