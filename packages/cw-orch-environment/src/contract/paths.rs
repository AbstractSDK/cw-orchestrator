pub use artifacts_dir::from_workspace;
pub use artifacts_dir::ArtifactsDir;
pub use wasm_path::WasmPath;

mod wasm_path {
    use crate::error::CwEnvError;
    use cosmwasm_std::ensure_eq;
    use sha256::TrySha256Digest;
    use std::path::{Path, PathBuf};

    /// Direct path to a `.wasm` file
    /// Stored as `PathBuf` to avoid lifetimes.
    /// Can be constructed from [`ArtifactsDir`](super::ArtifactsDir).
    ///
    /// # Example
    /// ```no_run
    /// use cw_orch_environment::contract::WasmPath;
    ///
    /// // Create a new WasmPath from a path to a WASM file.
    /// let wasm_path: WasmPath = WasmPath::new("path/to/contract.wasm").unwrap();
    ///
    /// // Calculate the checksum of the WASM file.
    /// let checksum: String = wasm_path.checksum().unwrap();
    /// ```
    #[derive(Debug, Clone)]
    pub struct WasmPath(PathBuf);

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
            Ok(Self(path))
        }

        /// Get the path to the WASM file
        pub fn path(&self) -> &Path {
            self.0.as_path()
        }

        /// Calculate the checksum of the WASM file.
        pub fn checksum(&self) -> Result<String, CwEnvError> {
            let checksum = self.path().digest()?;
            Ok(checksum)
        }
    }
}

mod artifacts_dir {
    use super::WasmPath;
    use crate::error::CwEnvError;

    use std::{env, fs, path::PathBuf};

    pub fn find_workspace_dir(start_path: Option<String>) -> ::std::path::PathBuf {
        let crate_path = start_path.unwrap_or(env!("CARGO_MANIFEST_DIR").to_string());
        let mut current_dir = ::std::path::PathBuf::from(crate_path);
        match find_workspace_dir_worker(&mut current_dir) {
            Some(path) => path,
            None => current_dir,
        }
    }

    fn find_workspace_dir_worker(dir: &mut ::std::path::PathBuf) -> Option<::std::path::PathBuf> {
        loop {
            let artifacts_dir = dir.join("artifacts");
            if ::std::fs::metadata(&artifacts_dir).is_ok() {
                return Some(dir.clone());
            }
            // First we pop the dir
            if !dir.pop() {
                return None;
            }
        }
    }

    #[macro_export]
    /// Creates an [`ArtifactsDir`] from the current workspace by searching the file tree for a directory named `artifacts`.
    /// It does this by reading the CARGO_MANIFEST_DIR environment variable and going up the file tree until it finds the `artifacts` directory.
    macro_rules! from_workspace {
        () => {
            ArtifactsDir::auto(Some(env!("CARGO_MANIFEST_DIR").to_string()))
        };
    }
    pub use from_workspace;

    /// Points to a directory containing WASM files
    ///
    /// # Example
    /// ```no_run
    /// use cw_orch_environment::contract::{ArtifactsDir, WasmPath};
    /// // Get the artifacts directory from the environment variable `ARTIFACTS_DIR`.
    /// let artifact_dir = ArtifactsDir::env();
    ///
    /// // Or create a new one.
    /// let artifact_dir = ArtifactsDir::new("path/to/artifacts");
    ///
    /// // Get a path to a WASM file that contains the string "my_contract".
    /// let wasm_path: WasmPath = artifact_dir.find_wasm_path("my_contract").unwrap();
    /// ```
    pub struct ArtifactsDir(PathBuf);

    impl ArtifactsDir {
        /// Get the artifacts directory from the environment variable `ARTIFACTS_DIR`.
        pub fn env() -> Self {
            let dir = env::var("ARTIFACTS_DIR").expect("ARTIFACTS_DIR env variable not set");
            Self::new(dir)
        }

        /// Creates an artifacts dir by searching for an artifacts directory by going up the file tree from start_path or the current directory
        pub fn auto(start_path: Option<String>) -> Self {
            // We find the artifacts dir automatically from the place that this function was invoked
            let workspace_dir = find_workspace_dir(start_path).join("artifacts");
            log::debug!("Found artifacts dir at {:?}", workspace_dir);
            Self::new(workspace_dir)
        }

        /// Create a new ArtifactsDir from a path to a directory containing WASM files.
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

        /// Find a WASM file in the artifacts directory that contains the given name.
        pub fn find_wasm_path(&self, name: &str) -> Result<WasmPath, CwEnvError> {
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
                    CwEnvError::WasmNotFound(
                        name.to_owned(),
                        self.path().to_str().unwrap_or_default().to_owned(),
                    )
                })?;
            WasmPath::new(self.path().join(path_str))
        }
    }
}
