#![doc(html_logo_url = "https://raw.githubusercontent.com/AbstractSDK/assets/mainline/logo.svg")]
#![doc = include_str ! (concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![deny(missing_docs)]

// macros
pub use cw_orch_contract_derive::interface;
pub use cw_orch_fns_derive::{ExecuteFns, QueryFns};

/// Re-export anyhow for use in the macros
pub extern crate anyhow;

// prelude
#[cfg(not(target_arch = "wasm32"))]
pub mod prelude;

#[cfg(not(target_arch = "wasm32"))]
pub use cw_orch_core::{build, contract};

/// Related to execution environents and variables
#[cfg(not(target_arch = "wasm32"))]
pub mod environment {
    pub use cw_orch_core::env::{default_state_folder, CwOrchEnvVars};
    pub use cw_orch_core::environment::*;
}
#[cfg(not(target_arch = "wasm32"))]
pub use cw_orch_mock as mock;

/// Re-export tokio, the async runtime when using daemons.
#[cfg(feature = "daemon")]
pub extern crate tokio;
#[cfg(feature = "daemon")]
pub mod daemon;

#[cfg(not(target_arch = "wasm32"))]
mod error;
#[cfg(feature = "osmosis-test-tube")]
pub mod osmosis_test_tube;

#[cfg(feature = "snapshot-testing")]
pub mod snapshots;

// Rexporting for the macro to work properly
#[cfg(feature = "snapshot-testing")]
pub extern crate insta;
#[cfg(feature = "snapshot-testing")]
pub extern crate sanitize_filename;
