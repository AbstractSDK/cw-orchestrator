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

pub use crate::deploy::Deploy;

pub use crate::environment::StateInterface;

// Response trait
pub use crate::environment::IndexResponse;

// Environment
pub use crate::environment::{CwEnv, QueryHandler, TxHandler, TxResponse};
pub use cw_orch_core::environment::queriers::*;

// Mock for testing
pub use crate::mock::Mock;

// OsmosisTestTube for testing
#[cfg(feature = "osmosis-test-tube")]
pub use crate::osmosis_test_tube::OsmosisTestTube;

// error
pub use crate::error::CwOrchError;

// Paths for implementing `Uploadable`
pub use crate::contract::{ArtifactsDir, WasmPath};

// re-export as it is used in the public API
pub use cosmwasm_std::{Addr, Coin, Empty};
pub use cw_multi_test::{Contract as MockContract, ContractWrapper};

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
