#![doc(html_logo_url = "https://raw.githubusercontent.com/AbstractSDK/assets/mainline/logo.svg")]
#![doc = include_str ! (concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![deny(missing_docs)]

// macros
pub use cw_orch_contract_derive::interface;
pub use cw_orch_fns_derive::{ExecuteFns, QueryFns};

// prelude
pub mod prelude;

mod error;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "daemon")]
pub mod daemon;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "snapshot-testing")]
pub mod snapshots;

/// Re-export anyhow for use in the macros
pub extern crate anyhow;
// This re-export should not be touched or the derive macros WILL break
pub use cw_orch_core as core;
pub use cw_orch_core::{build, contract};
/// Related to execution environments
pub mod environment {
    pub use cw_orch_core::environment::*;
}

#[cfg(not(target_arch = "wasm32"))]
/// used to avoid repeating the #[cfg(not(target_arch = "wasm32"))] macro for each export
pub mod wasm_protected {

    /// Related environment variables definition
    pub mod env_vars {
        pub use cw_orch_core::CoreEnvVars;
        #[cfg(feature = "daemon")]
        pub use cw_orch_daemon::{env::default_state_folder, env::DaemonEnvVars};
    }
    pub use cw_orch_mock as mock;

    // Rexporting for the macro to work properly
    #[cfg(feature = "snapshot-testing")]
    pub extern crate insta;
    #[cfg(feature = "snapshot-testing")]
    pub extern crate sanitize_filename;
}

#[cfg(not(target_arch = "wasm32"))]
pub use wasm_protected::*;
