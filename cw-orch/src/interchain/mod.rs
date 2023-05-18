pub mod docker;
pub mod error;
pub mod hermes;
pub mod infrastructure;
pub mod follow_ibc_execution;

pub type IcResult<T> = Result<T, error::InterchainError>;
