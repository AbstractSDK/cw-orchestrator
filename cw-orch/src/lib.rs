#![doc(html_logo_url = "https://raw.githubusercontent.com/AbstractSDK/assets/mainline/logo.svg")]
#![doc = include_str ! ("../README.md")]
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
#[cfg(feature = "daemon")]
pub mod daemon;

pub mod deploy;
pub mod environment;
mod error;
mod index_response;
mod interface_traits;
#[cfg(feature = "daemon")]
mod keys;
pub mod mock;
mod paths;
pub mod remote_mock;

pub mod state;

#[cfg(feature = "daemon")]
pub mod live_mock;
