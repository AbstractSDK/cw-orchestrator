use cw_orch_starship::StarshipClientError;
use ibc_relayer_types::core::ics24_host::error::ValidationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterchainError {
    #[error("Error interacting with Starship {0}")]
    Docker(#[from] StarshipClientError),

    #[error("Error interacting with daemon {0}")]
    Daemon(#[from] crate::daemon::DaemonError),

    #[error("Error validating IBC structures {0}")]
    ValidationError(#[from] ValidationError),

    #[error("Error validating IBC structures {0}")]
    ICSChannel(#[from] ibc_relayer_types::core::ics04_channel::error::Error),

    #[error("Could not find hermes container. Ensure it is running.")]
    HermesContainerNotFound,

    #[error("daemon for chain {0} not found")]
    DaemonNotFound(String),

    #[error("chain config for chain {0} not found")]
    ChainConfigNotFound(String),

    #[error("Configuration already registered for chain {0}")]
    AlreadyRegistered(String),
}
