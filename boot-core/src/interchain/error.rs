use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterchainError {
    #[error("Error interacting with docker {0}")]
    Docker(#[from] ::bollard::errors::Error),
    #[error("Error interacting with daemon {0}")]
    Daemon(#[from] crate::daemon::error::DaemonError),
    #[error("Could not find hermes container. Ensure it is running.")]
    HermesContainerNotFound,
}
