use crate::error::CwEnvError;
use cosmwasm_std::{ensure_eq, Checksum};
use std::{io::Read, path::PathBuf};

use super::{github::GithubWasmPath, GithubWasmPathLocation};

/// Direct path to a `.wasm` file
/// Stored as `PathBuf` to avoid lifetimes.
/// Can be constructed from [`ArtifactsDir`](super::ArtifactsDir).
///
/// # Example
/// ```no_run
/// use cw_orch_core::contract::WasmPath;
///
/// // Create a new WasmPath from a path to a WASM file.
/// let wasm_path: WasmPath = WasmPath::path("path/to/contract.wasm").unwrap();
///
/// // Calculate the checksum of the WASM file.
/// let checksum: cosmwasm_std::Checksum = wasm_path.checksum().unwrap();
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum WasmPath {
    Path(PathBuf),
    Github(GithubWasmPath),
}

impl WasmPath {
    /// Create a new WasmPath from a path to a WASM file.
    pub fn path(path: impl Into<PathBuf>) -> Result<Self, CwEnvError> {
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

    /// Creates a new WasmPath from a github release asset
    pub fn github_release(
        owner: impl Into<String>,
        repo_name: impl Into<String>,
        release_tag: impl Into<String>,
        file_name: impl Into<String>,
    ) -> Self {
        WasmPath::Github(GithubWasmPath {
            owner: owner.into(),
            repo_name: repo_name.into(),
            location: GithubWasmPathLocation::Release {
                tag: release_tag.into(),
                file_name: file_name.into(),
            },
        })
    }

    /// Creates a new WasmPath from a github file
    pub fn github_file(
        owner: impl Into<String>,
        repo_name: impl Into<String>,
        reference: impl Into<String>,
        file_path: impl Into<String>,
    ) -> Self {
        WasmPath::Github(GithubWasmPath {
            owner: owner.into(),
            repo_name: repo_name.into(),
            location: GithubWasmPathLocation::File {
                reference: reference.into(),
                file_path: file_path.into(),
            },
        })
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
