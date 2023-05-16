use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

// in this enum we setup our handlers to default errors
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error(transparent)]
    ContractOverflow(#[from] OverflowError),
}
