pub mod docker;
pub mod error;
pub mod hermes;
pub mod infrastructure;

pub type IcResult<T> = Result<T, error::InterchainError>;
