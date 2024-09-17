use crate::error::CwEnvError;
use cosmwasm_std::{ensure_eq, Checksum};
use std::{io::Read, path::PathBuf};

use super::github::GithubWasmPath;

/// Direct path to a `.wasm` file
/// Stored as `PathBuf` to avoid lifetimes.
/// Can be constructed from [`ArtifactsDir`](super::ArtifactsDir).
///
/// # Example
/// ```no_run
/// use cw_orch_core::contract::WasmPath;
///
/// // Create a new WasmPath from a path to a WASM file.
/// let wasm_path: WasmPath = WasmPath::new("path/to/contract.wasm").unwrap();
///
/// // Calculate the checksum of the WASM file.
/// let checksum: cosmwasm_std::Checksum = wasm_path.checksum().unwrap();
/// ```
#[derive(Debug, Clone)]
pub enum WasmPath {
    Path(PathBuf),
    Github(GithubWasmPath),
}

impl WasmPath {
    /// Create a new WasmPath from a path to a WASM file.
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, CwEnvError> {
        let path: PathBuf = path.into();
        assert!(
            path.exists(),
            "provided path {} does not exist",
            path.display()
        );
        ensure_eq!(
            path.extension(),
            Some("wasm".as_ref()),
            CwEnvError::NotWasm {}
        );
        Ok(Self::Path(path))
    }

    /// Get the content of the WASM file
    pub async fn wasm(&self) -> Result<Vec<u8>, CwEnvError> {
        match self {
            WasmPath::Path(path_buf) => {
                let mut file = std::fs::File::open(path_buf)?;
                let mut wasm = Vec::<u8>::new();
                file.read_to_end(&mut wasm)?;
                Ok(wasm)
            }
            WasmPath::Github(github) => github.wasm().await,
        }
    }

    /// Calculate the checksum of the WASM file.
    pub async fn checksum(&self) -> Result<Checksum, CwEnvError> {
        let wasm = self.wasm().await?;
        Ok(Checksum::generate(&wasm))
    }
}
