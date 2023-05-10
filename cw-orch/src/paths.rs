pub use artifacts_dir::ArtifactsDir;
pub use wasm_path::WasmPath;

mod wasm_path {
    use crate::error::CwOrchError;
    use cosmwasm_std::ensure_eq;
    use std::path::{Path, PathBuf};

    /// Direct path to a `.wasm` file
    /// Stored as `PathBuf` to avoid lifetimes
    #[derive(Debug, Clone)]
    pub struct WasmPath(PathBuf);

    impl WasmPath {
        pub fn new(path: impl Into<PathBuf>) -> Result<Self, CwOrchError> {
            let path: PathBuf = path.into();
            assert!(
                path.exists(),
                "provided path {} does not exist",
                path.display()
            );
            ensure_eq!(
                path.extension(),
                Some("wasm".as_ref()),
                CwOrchError::StdErr("File must be a wasm file".into())
            );
            Ok(Self(path))
        }

        /// Get the path to the wasm file
        pub fn path(&self) -> &Path {
            self.0.as_path()
        }

        /// Calculate the checksum of the wasm file to compare against previous uploads
        pub fn checksum(&self, _id: &str) -> Result<String, CwOrchError> {
            let checksum = sha256::try_digest(self.path())?;
            Ok(checksum)
        }
    }
}

mod artifacts_dir {
    use std::{env, fs, path::PathBuf};

    use crate::error::CwOrchError;
    use crate::paths::wasm_path::WasmPath;

    /// Points to a directory containing wasm files
    pub struct ArtifactsDir(PathBuf);

    impl ArtifactsDir {
        /// Get the artifacts directory from the environment variable `ARTIFACTS_DIR`
        pub fn env() -> Self {
            let dir = env::var("ARTIFACTS_DIR").expect("ARTIFACTS_DIR env variable not set");
            Self::new(dir)
        }

        pub fn new(path: impl Into<PathBuf>) -> Self {
            let path: PathBuf = path.into();
            assert!(
                path.exists(),
                "provided path {} does not exist",
                path.display()
            );
            Self(path)
        }

        /// Get the path to the artifacts directory
        pub fn path(&self) -> &PathBuf {
            &self.0
        }

        /// Find a wasm file in the artifacts directory with the given name
        pub fn find_wasm_path(&self, name: &str) -> Result<WasmPath, CwOrchError> {
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
                    CwOrchError::StdErr(format!(
                        "Could not find wasm file with name {} in artifacts dir",
                        name,
                    ))
                })?;
            WasmPath::new(self.path().join(path_str))
        }
    }
}
