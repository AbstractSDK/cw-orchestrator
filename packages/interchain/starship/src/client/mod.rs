//! Starship client allowing to interact
//! This is rather low level. Most functions are exposed but rarely used outside of the starship struct in lib.rs

mod core;
mod error;
pub mod faucet;
pub mod registry;

pub use crate::client::core::StarshipClient;
pub use error::StarshipClientError;

/// Custom Result that is used to simplify return types
pub type StarshipClientResult<T> = Result<T, error::StarshipClientError>;
