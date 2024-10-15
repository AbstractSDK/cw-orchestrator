#![warn(missing_docs)]
//! Implementation of the interchain environment for the daemon chain type.
//! This also adds more helpers in the daemon case

mod channel_creator;
pub mod error;
mod interchain_env;
pub mod packet_inspector;
// Tracking IBC state
pub mod ibc_tracker;
pub mod interchain_log;

pub use error::InterchainDaemonError;

/// Alias for an interchain Daemon Result
pub type IcDaemonResult<R> = Result<R, InterchainDaemonError>;

/// We want to export some major elements
pub use channel_creator::{ChannelCreationValidator, ChannelCreator};

pub use interchain_env::DaemonInterchain;
pub use interchain_env::Mnemonic;
