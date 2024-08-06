#![allow(missing_docs)]
use cw_orch_core::CwEnvError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StarshipClientError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),

    #[error("Error connecting to faucet at {0}")]
    FaucetError(String),

    #[error("Error connecting to registry at {0}")]
    RegistryError(String),

    #[error("Could not find hermes for these chains on localhost. Ensure it is running.")]
    HermesNotFound,

    #[error("daemon for chain {0} not found")]
    DaemonNotFound(String),

    #[error("chain config for chain {0} not found")]
    ChainConfigNotFound(String),

    #[error("There was a mismatch in the number of chains between the config object: {0} and the starship instance: {1}")]
    StarshipConfigMismatch(usize, usize),

    #[error("Configuration already registered for chain {0}")]
    AlreadyRegistered(String),

    #[error("Missing test mnemonic for chain {0}")]
    MissingTestMnemonic(String),

    #[error(transparent)]
    Kube(#[from] kube::Error),

    #[error(transparent)]
    StdIo(#[from] std::io::Error),

    #[error("Channel creation failed {0}-{1}, reason: {2}")]
    ChannelCreationFailure(String, String, String),
}

impl From<StarshipClientError> for CwEnvError {
    fn from(val: StarshipClientError) -> Self {
        CwEnvError::AnyError(val.into())
    }
}
