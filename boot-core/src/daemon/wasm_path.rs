use std::path::{Path, PathBuf};

use cosmwasm_std::ensure_eq;

use crate::DaemonError;

/// Direct path to a `.wasm` file
// Store as `PathBuf` to avoid lifetimes
pub struct WasmPath(PathBuf);

impl WasmPath {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, DaemonError> {
        let path: PathBuf = path.into();
        path.try_exists()?;
        ensure_eq!(
            path.extension(),
            Some("wasm".as_ref()),
            DaemonError::StdErr("File must be a wasm file".into())
        );
        Ok(Self(path))
    }

    /// Get the path to the wasm file
    pub fn path(&self) -> &Path {
        self.0.as_path()
    }

    /// Calculate the checksum of the wasm file to compare against previous uploads
    pub fn checksum(&self, _id: &str) -> Result<String, DaemonError> {
        let checksum = sha256::try_digest(self.path())?;
        Ok(checksum)
    }
}
