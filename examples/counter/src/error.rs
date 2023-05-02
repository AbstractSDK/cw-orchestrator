use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

/// This overrides the ContractError enum defined in cw721-base
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error(transparent)]
    ContractOverflow(#[from] OverflowError),
}
