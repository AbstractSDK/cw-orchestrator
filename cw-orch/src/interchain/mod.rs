pub mod docker;
pub mod error;
pub mod hermes;
pub mod infrastructure;
pub mod interchain_channel;
pub mod interchain_channel_builder;
pub mod interchain_env;

pub type IcResult<T> = Result<T, error::InterchainError>;
