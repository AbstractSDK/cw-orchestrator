pub mod error;
pub mod infra;

pub type IcResult<T> = Result<T, error::InterchainError>;
