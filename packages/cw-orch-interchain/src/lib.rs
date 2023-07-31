pub mod packet_inspector;
mod error;
mod interchain_channel;
mod interchain_channel_builder;
mod interchain_env;

pub use error::InterchainError;

pub type IcResult<R> = Result<R, InterchainError>;