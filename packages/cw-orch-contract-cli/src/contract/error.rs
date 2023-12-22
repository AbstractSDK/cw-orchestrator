use cw_orch::prelude::CwOrchError;
use inquire::error::InquireError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrchCliError {
    #[error("{0}")]
    InquireError(#[from] InquireError),

    #[error("{0}")]
    CwOrchError(#[from] CwOrchError),

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
}
