use std::{env, fs, path::PathBuf};

use crate::DaemonError;

use super::wasm_path::WasmPath;

/// Points to a directory containing wasm files
pub struct ArtifactsDir(PathBuf);

impl ArtifactsDir {
    /// Get the artifacts directory from the environment variable `ARTIFACTS_DIR`
    pub fn env() -> Option<Self> {
        env::var("ARTIFACTS_DIR").ok().map(Self::new)
    }

    pub fn new(path: impl Into<PathBuf>) -> Self {
        // TODO: check if path exists
        Self(path.into())
    }

    /// Get the path to the artifacts directory
    pub fn path(&self) -> &PathBuf {
        &self.0
    }

    /// Find a wasm file in the artifacts directory with the given name
    pub fn find_wasm_path(&self, name: &str) -> Result<WasmPath, DaemonError> {
        let path_str = fs::read_dir(self.path())?
            .find_map(|entry| {
                let path = entry.ok()?.path();
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                if path.is_file()
                    && path.extension().unwrap_or_default() == "wasm"
                    && file_name.contains(name)
                {
                    Some(file_name.into_owned())
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                DaemonError::StdErr(format!(
                    "Could not find wasm file with name {} in artifacts dir",
                    name,
                ))
            })?;
        WasmPath::new(self.path().join(path_str))
    }
}
