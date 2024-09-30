//! This crate aims at providing types, structures and traits for implementing an IBC execution/analysis environment
//! It provides helpers and a common structure to make the code as environment agnostic as possible
//! Just like cw-orch as a whole it aims at unifying the developer experience inside tests/scripts/maintenance processes.

#![warn(missing_docs)]
pub mod channel;

/// Contains definitions of the main trait exposed by this crate
pub mod env;

/// Contains default acknowledgment parsers
mod ack_parser;
mod error;

/// Type definition for interchain structure and return types
pub mod results;
pub(crate) mod analysis;
pub(crate) mod packet;
pub(crate) mod tx;
pub(crate) mod ibc_query;

pub use ack_parser::IbcAckParser;
pub use env::InterchainEnv;
pub use error::InterchainError;
pub use ibc_query::IbcQueryHandler;
