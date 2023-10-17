#![allow(missing_docs)]

use std::{
    env,
    num::{ParseFloatError, ParseIntError},
};

use thiserror::Error;

/// cw-orchestrator error wrapper using thiserror.
#[derive(Error, Debug)]
pub enum CwEnvError {
    #[error(transparent)]
    CosmWasmError(#[from] cosmwasm_std::StdError),
    #[error("Code id for {0} not found in store")]
    CodeIdNotInStore(String),
    #[error("Contract address for {0} not found in store")]
    AddrNotInStore(String),
    #[error(transparent)]
    IOErr(#[from] ::std::io::Error),
    #[error("JSON Conversion Error")]
    SerdeJson(#[from] ::serde_json::Error),
    #[error(transparent)]
    EnvvarError(#[from] env::VarError),
    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("File must be a wasm file")]
    NotWasm,
    #[error("Could not find wasm file with name {0} in artifacts:{1} dir")]
    WasmNotFound(String, String),
    #[error("calling contract with unimplemented action")]
    NotImplemented,
    #[error(transparent)]
    AnyError(#[from] ::anyhow::Error),
    #[error("Generic Error {0}")]
    StdErr(String),
}

impl CwEnvError {
    pub fn root(&self) -> &dyn std::error::Error {
        match self {
            CwEnvError::AnyError(e) => e.root_cause(),
            _ => panic!("Unexpected error type"),
        }
    }

    pub fn downcast<E>(self) -> anyhow::Result<E>
    where
        E: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static,
    {
        match self {
            CwEnvError::AnyError(e) => e.downcast(),
            _ => panic!("Unexpected error type"),
        }
    }
}
