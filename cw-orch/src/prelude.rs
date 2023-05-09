//! This file re-exports parts of the API of the cw-orchestrator crate to make use of it easier.
//!
//! ```rust
//! use cw_orch::prelude::*;
//! ```

// We don't want to re-export everything here!
// 1. Macros
// 2. Traits that need to be imported to preform actions on an interface
// 3. Traits that need to be imported to implement an interface
// 4. Objects that need to be available to implement required traits

// macros
pub use cw_orch_contract_derive::{contract, interface};
pub use cw_orch_fns_derive::{ExecuteFns, QueryFns};

// Contract traits
pub use crate::interface_traits::{
    CallAs, ContractInstance, CwOrcExecute, CwOrcInstantiate, CwOrcMigrate, CwOrcQuery,
    CwOrcUpload, ExecutableContract, InstantiableContract, MigratableContract, QueryableContract,
    Uploadable,
};

// Response trait
pub use crate::index_response::IndexResponse;

// Environment
pub use crate::environment::{CwEnv, TxResponse};

// Mock for testing
pub use crate::mock::core::Mock;

// error
pub use crate::error::CwOrcError;

// Paths for implementing `Uploadable`
pub use crate::paths::{ArtifactsDir, WasmPath};

// re-export as it is used in the public API
pub use cosmwasm_std::{Addr, Coin, Empty};
pub use cw_multi_test::{custom_app, BasicApp, Contract as MockContract, ContractWrapper};

// builder, core type, networks mod, queriers mod, traits
#[cfg(feature = "daemon")]
pub use crate::daemon::{
    builder::DaemonBuilder,
    core::Daemon,
    networks, queriers,
    traits::{MigrateHelpers, UploadHelpers},
};

/// Re-export trait and data required to fetch daemon data from chain-registry
#[cfg(feature = "daemon")]
pub use ibc_chain_registry::{chain::ChainData as ChainRegistryData, fetchable::Fetchable};
