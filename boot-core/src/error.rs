#![allow(missing_docs)]

use thiserror::Error;

use crate::daemon::error::DaemonError;

#[derive(Error, Debug)]
pub enum BootError {
    #[error(transparent)]
    DaemonError(#[from] DaemonError),
    #[error("JSON Conversion Error")]
    SerdeJson(#[from] ::serde_json::Error),
    #[error(transparent)]
    CosmWasmError(#[from] cosmwasm_std::StdError),
    #[error(transparent)]
    AnyError(#[from] ::anyhow::Error),
    #[error("Contract address for {0} not found in store")]
    AddrNotInStore(String),
    #[error("Code id for {0} not found in store")]
    CodeIdNotInStore(String),
    #[error("calling contract with unimplemented action")]
    NotImplemented,
    #[error("Generic Error {0}")]
    StdErr(String),
}

impl BootError {
    pub fn root(&self) -> &dyn std::error::Error {
        match self {
            BootError::AnyError(e) => e.root_cause(),
            _ => panic!("Unexpected error type"),
        }
    }
}
