#![allow(missing_docs)]

use cw_orch_core::CwEnvError;
use thiserror::Error;

/// cw-orchestrator error wrapper using thiserror.
#[derive(Error, Debug)]
pub enum CwOrchError {
    #[error(transparent)]
    CwEnv(#[from] cw_orch_core::CwEnvError),
    #[cfg(feature = "daemon")]
    #[error(transparent)]
    DaemonError(#[from] cw_orch_daemon::DaemonError),
    #[cfg(feature = "osmosis-test-tube")]
    #[error(transparent)]
    TestTube(#[from] osmosis_test_tube::RunnerError),
    #[error(transparent)]
    AnyError(#[from] ::anyhow::Error),
    #[error(transparent)]
    IOErr(#[from] ::std::io::Error),
    #[error("Generic Error {0}")]
    StdErr(String),
}

impl From<CwOrchError> for CwEnvError {
    fn from(val: CwOrchError) -> Self {
        CwEnvError::AnyError(val.into())
    }
}

impl CwOrchError {
    pub fn root(&self) -> &dyn std::error::Error {
        match self {
            CwOrchError::AnyError(e) => e.root_cause(),
            CwOrchError::CwEnv(CwEnvError::AnyError(e)) => e.root_cause(),
            _ => panic!("Unexpected error type"),
        }
    }

    pub fn downcast<E>(self) -> anyhow::Result<E>
    where
        E: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static,
    {
        match self {
            CwOrchError::AnyError(e) => e.downcast(),
            CwOrchError::CwEnv(CwEnvError::AnyError(e)) => e.downcast(),
            _ => panic!("Unexpected error type"),
        }
    }
}
