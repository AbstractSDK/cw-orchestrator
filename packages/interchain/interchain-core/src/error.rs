//! Generic Error type for interchain errors

#![allow(missing_docs)]
use cosmwasm_std::{Binary, StdError};
use ibc_relayer_types::core::ics24_host::error::ValidationError;
use thiserror::Error;

use cw_orch_core::CwEnvError;

#[derive(Error, Debug)]
pub enum InterchainError {
    #[error("{0}")]
    GenericError(String),

    #[error(transparent)]
    CwOrchError(#[from] CwEnvError),

    #[error(transparent)]
    StdError(#[from] StdError),

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

    #[error("Chain `{0}` not found in the interchain environment. Please register it before analyzing transactions")]
    ChainNotFound(String),

    #[error("Packet relaying failed, timeout received")]
    PacketTimeout {},

    #[error(
        "Acknowledgement decoding failed with ack: {0:x?}, tried decoding it as json : {1:x?}. Try using `assert_custom` instead !"
    )]
    AckDecodingFailed(Binary, String),

    #[error("No matching packets were found matching the given parsing function")]
    NoMatchingPacketFound(),

    #[error("Some packets were not parsed by the given parsing functions")]
    RemainingPackets {},

    #[error("No packets were found while following packets")]
    NoPacketsFound {},

    #[error("Failure acknowledgment received: {0:?}")]
    FailedAckReceived(String),
}
