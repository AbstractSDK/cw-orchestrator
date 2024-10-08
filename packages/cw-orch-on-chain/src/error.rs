#![allow(missing_docs)]

use cosmwasm_std::StdError;
use cw_orch_core::CwEnvError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OnChainError {
    #[error(transparent)]
    CosmwasmStd(#[from] cosmwasm_std::StdError),

    #[error(transparent)]
    Instantiate2(#[from] cosmwasm_std::Instantiate2AddressError),
}

impl From<OnChainError> for CwEnvError {
    fn from(val: OnChainError) -> Self {
        CwEnvError::AnyError(val.into())
    }
}

impl From<OnChainError> for StdError {
    fn from(val: OnChainError) -> Self {
        StdError::generic_err(val.to_string())
    }
}
