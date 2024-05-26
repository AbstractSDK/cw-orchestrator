use cosmwasm_std::StdError;
use cw_orch_interchain_core::InterchainError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterchainMockError {
    #[error(transparent)]
    InterchainError(#[from] InterchainError),

    #[error(transparent)]
    Any(#[from] anyhow::Error),

    #[error(transparent)]
    StdError(#[from] StdError),

    #[error("Error validating IBC structures {0}")]
    ValidationError(#[from] ibc_relayer_types::core::ics24_host::error::ValidationError),

    #[error("Error validating IBC structures {0}")]
    ICSChannel(#[from] ibc_relayer_types::core::ics04_channel::error::Error),

    #[error("Configuration already registered for chain {0}")]
    AlreadyRegistered(String),

    #[error("mock for chain {0} not found")]
    MockNotFound(String),
}

impl From<InterchainMockError> for InterchainError {
    fn from(value: InterchainMockError) -> Self {
        InterchainError::GenericError(value.to_string())
    }
}
