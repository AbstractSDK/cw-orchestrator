#![allow(missing_docs)]

use thiserror::Error;

#[cfg(feature = "daemon")]
use crate::daemon::DaemonError;

/// cw-orchestrator error wrapper using thiserror.
#[derive(Error, Debug)]
pub enum CwOrchError {
    #[cfg(feature = "daemon")]
    #[error(transparent)]
    DaemonError(#[from] DaemonError),
    #[cfg(feature = "osmosis-test-tube")]
    #[error(transparent)]
    TestTubeError(#[from] osmosis_test_tube::RunnerError),
    #[cfg(feature = "starship")]
    #[error(transparent)]
    Starship(#[from] cw_orch_starship::StarshipClientError),
    #[error("JSON Conversion Error")]
    SerdeJson(#[from] ::serde_json::Error),
    #[error(transparent)]
    CosmWasmError(#[from] cosmwasm_std::StdError),
    #[error(transparent)]
    AnyError(#[from] ::anyhow::Error),
    #[error(transparent)]
    IOErr(#[from] ::std::io::Error),
    #[error("Contract address for {0} not found in store")]
    AddrNotInStore(String),
    #[error("Code id for {0} not found in store")]
    CodeIdNotInStore(String),
    #[error("calling contract with unimplemented action")]
    NotImplemented,
    #[error("Generic Error {0}")]
    StdErr(String),
}

impl CwOrchError {
    pub fn root(&self) -> &dyn std::error::Error {
        match self {
            CwOrchError::AnyError(e) => e.root_cause(),
            _ => panic!("Unexpected error type"),
        }
    }
}
