pub use artifacts_dir::from_workspace;
pub use artifacts_dir::ArtifactsDir;
pub use wasm_path::WasmPath;

mod wasm_path {
    use crate::error::CwEnvError;
    use cosmwasm_std::{ensure_eq, Checksum};
    use std::{
        io::Read,
        path::{Path, PathBuf},
    };

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
    /// let checksum: cosmwasm_std::HexBinary = wasm_path.checksum().unwrap();
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
        pub fn checksum(&self) -> Result<Checksum, CwEnvError> {
            let mut file = std::fs::File::open(self.path())?;
            let mut wasm = Vec::<u8>::new();
            file.read_to_end(&mut wasm)?;
            Ok(Checksum::generate(&wasm))
        }
    }
}

mod artifacts_dir {
    const ARM_POSTFIX: &str = "-aarch64";

    use super::WasmPath;
    use crate::{
        build::BuildPostfix, env::ARTIFACTS_DIR_ENV_NAME, error::CwEnvError, log::local_target,
        CoreEnvVars,
    };

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
    /// use cw_orch_core::contract::{ArtifactsDir, WasmPath};
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
            let dir = CoreEnvVars::artifacts_dir()
                .unwrap_or_else(|| panic!("{} env variable not set", ARTIFACTS_DIR_ENV_NAME));
            Self::new(dir)
        }

        /// Creates an artifacts dir by searching for an artifacts directory by going up the file tree from start_path or the current directory
        pub fn auto(start_path: Option<String>) -> Self {
            // We find the artifacts dir automatically from the place that this function was invoked
            let workspace_dir = find_workspace_dir(start_path).join("artifacts");
            log::debug!(target: &local_target(), "Found artifacts dir at {:?}", workspace_dir);
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
            self.find_wasm_path_with_build_postfix(name, <BuildPostfix>::None)
        }

        /// Find a WASM file in the artifacts directory that contains the given contract name AND build post-fix.
        /// If a build with the post-fix is not found, the default build will be used.
        /// If none of the two are found, an error is returned.
        pub fn find_wasm_path_with_build_postfix(
            &self,
            name: &str,
            build_postfix: BuildPostfix,
        ) -> Result<WasmPath, CwEnvError> {
            let build_postfix: String = build_postfix.into();
            // Found artifacts priority respected

            let mut wasm_with_postfix = None;
            let mut arm_wasm_with_postfix = None;
            let mut default_wasm = None;
            let mut arm_default_wasm = None;

            for entry in fs::read_dir(self.path())?.flatten() {
                let path = entry.path();
                // Skip if not a wasm file
                if !path.is_file() || path.extension().unwrap_or_default() != "wasm" {
                    continue;
                }

                let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                // Wasm with build postfix, non-ARM
                if is_artifact_with_build_postfix(&file_name, name, &build_postfix) {
                    wasm_with_postfix = Some(file_name.into_owned());
                    // As it's highest priority we just the loop end here
                    break;
                }

                // Check other valid filenames
                if is_arm_artifact_with_build_postfix(&file_name, name, &build_postfix) {
                    // Wasm with build postfix, ARM
                    arm_wasm_with_postfix = Some(file_name.into_owned())
                } else if is_default_artifact(&file_name, name) {
                    // Wasm without build postfix, non-ARM
                    default_wasm = Some(file_name.into_owned())
                } else if is_default_arm_artifact(&file_name, name) {
                    // Wasm without build postfix, ARM
                    arm_default_wasm = Some(file_name.into_owned())
                }
            }

            let path_str = wasm_with_postfix
                .or(arm_wasm_with_postfix)
                .or(default_wasm)
                .or(arm_default_wasm)
                .ok_or_else(|| {
                    CwEnvError::WasmNotFound(
                        name.to_owned(),
                        self.path().to_str().unwrap_or_default().to_owned(),
                    )
                })?;
            WasmPath::new(self.path().join(path_str))
        }
    }

    fn is_artifact(file_name: &str, contract_name: &str) -> bool {
        file_name.contains(contract_name)
    }

    fn is_default_artifact(file_name: &str, contract_name: &str) -> bool {
        file_name.ends_with(format!("{contract_name}.wasm").as_str())
    }

    fn is_default_arm_artifact(file_name: &str, contract_name: &str) -> bool {
        file_name.ends_with(format!("{contract_name}{ARM_POSTFIX}.wasm").as_str())
    }

    fn is_artifact_with_build_postfix(
        file_name: &str,
        contract_name: &str,
        build_postfix: &str,
    ) -> bool {
        is_artifact(file_name, contract_name)
            && file_name.ends_with(format!("{build_postfix}.wasm").as_str())
    }

    fn is_arm_artifact_with_build_postfix(
        file_name: &str,
        contract_name: &str,
        build_postfix: &str,
    ) -> bool {
        is_artifact(file_name, contract_name)
            && file_name.ends_with(format!("{build_postfix}{ARM_POSTFIX}.wasm").as_str())
    }
}
