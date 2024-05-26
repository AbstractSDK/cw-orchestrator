#![allow(missing_docs)]

use cosmwasm_std::StdError;
use cw_orch_interchain_core::{channel::InterchainChannel, types::NetworkId, InterchainError};
use thiserror::Error;
use tonic::transport::Channel;

#[derive(Error, Debug)]
pub enum InterchainDaemonError {
    #[error(transparent)]
    InterchainError(#[from] InterchainError),

    #[error(transparent)]
    StdError(#[from] StdError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("You have interrupted the script execution")]
    ManualInterruption,

    #[error("Error interacting with starship {0}")]
    Starship(#[from] cw_orch_starship::client::StarshipClientError),

    #[error("Error interacting with daemon {0}")]
    Daemon(#[from] cw_orch_daemon::DaemonError),

    #[error("Error validating IBC structures {0}")]
    ValidationError(#[from] ibc_relayer_types::core::ics24_host::error::ValidationError),

    #[error("Error validating IBC structures {0}")]
    ICSChannel(#[from] ibc_relayer_types::core::ics04_channel::error::Error),

    #[error("Could not find hermes container. Ensure it is running.")]
    HermesContainerNotFound,

    #[error("daemon for chain {0} not found")]
    DaemonNotFound(String),

    #[error("Channel creation events not found from chain {src_chain} on following channel : {channel:?}")]
    ChannelCreationEventsNotFound {
        src_chain: NetworkId,
        channel: InterchainChannel<Channel>,
    },

    #[error("Configuration already registered for chain {0}")]
    AlreadyRegistered(String),
}

impl From<InterchainDaemonError> for InterchainError {
    fn from(value: InterchainDaemonError) -> Self {
        InterchainError::GenericError(value.to_string())
    }
}
