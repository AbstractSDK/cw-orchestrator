use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterchainError {
    #[error("Error interacting with docker {0}")]
    Docker(#[from] ::bollard::errors::Error),
    #[error("Error interacting with daemon {0}")]
    Daemon(#[from] crate::daemon::DaemonError),
    #[error("Could not find hermes container. Ensure it is running.")]
    HermesContainerNotFound,
    #[error("daemon for chain {0} not found")]
    DaemonNotFound(String),
}
