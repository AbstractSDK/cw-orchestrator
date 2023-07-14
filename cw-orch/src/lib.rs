#![doc(html_logo_url = "https://raw.githubusercontent.com/AbstractSDK/assets/mainline/logo.svg")]
#![doc = include_str ! (concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![deny(missing_docs)]

// macros
pub use cw_orch_contract_derive::{interface, interface_entry_point};
pub use cw_orch_fns_derive::{ExecuteFns, QueryFns};

/// Re-export anyhow for use in the macros
pub extern crate anyhow;

/// Re-export tokio, the async runtime when using daemons.
#[cfg(feature = "daemon")]
pub extern crate tokio;

// prelude
pub mod prelude;

pub mod contract;
#[cfg(feature = "daemon-without-runtime")]
pub mod daemon;

pub mod deploy;
pub mod environment;
mod error;
mod index_response;
mod interface_traits;
#[cfg(feature = "daemon-without-runtime")]
mod keys;
pub mod mock;
#[cfg(feature = "osmosis-test-tube")]
pub mod osmosis_test_tube;
mod paths;

pub mod state;

#[cfg(feature = "daemon")]
pub mod live_mock;
