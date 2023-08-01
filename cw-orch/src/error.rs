#![allow(missing_docs)]

use cw_orch_environment::CwEnvError;
use cw_orch_starship::StarshipClientError;
use ibc_relayer_types::core::ics24_host::error::ValidationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CwOrchError {
    #[error("Error: {0}")]
    CwEnvError(#[from] CwEnvError),

    #[cfg(feature="interchain")]
    #[error("Error interacting with interchain elements {0}")]
    InterchainError(#[from] crate::interchain::InterchainError),

    #[error("Error interacting with starhip {0}")]
    StarshipError(#[from] StarshipClientError),

}