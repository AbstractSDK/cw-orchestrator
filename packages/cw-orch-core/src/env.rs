//! This regroups all env variables used by cw-orch-daemon. It allows for easier documentation and env variable management
//! This is used to import environment variables with safe names (and at a centralized location)
//! To get the env variable parsed value, you can use
//! ```rust,no_run
//! use cw_orch_core::CwOrchEnvVars;
//! let env_variable = CwOrchEnvVars::load().unwrap().state_file;
//! ```

use std::{env, path::PathBuf, str::FromStr};

use crate::CwEnvError;

pub const ARTIFACTS_DIR_ENV_NAME: &str = "ARTIFACTS_DIR";
pub const SERIALIZE_ENV_NAME: &str = "CW_ORCH_SERIALIZE_JSON";
pub const DISABLE_MANUAL_INTERACTION_ENV_NAME: &str = "CW_ORCH_DISABLE_MANUAL_INTERACTION";

#[derive(Default)]
pub struct CoreEnvVars {
    // /// Optional - Path
    // /// This is the path to the state file
    // /// `folder/file.json` will resolve to `~/.cw-orchestrator/folder/file.json`
    // /// `./folder/file.json` will resolve `$pwd/folder/file.json`
    // /// `../folder/file.json` will resolve `$pwd/../folder/file.json`
    // /// `/usr/var/file.json` will resolve to `/usr/var/file.json`
    // /// Defaults to "~./cw-orchestrator/state.json"
    // pub state_file: PathBuf,
    /// Optional - Path
    /// Where cw-orch will look for wasm files. This is used by `ArtifactsDir::env()``
    pub artifacts_dir: Option<PathBuf>,

    /// Optional - Boolean
    /// Defaults to false
    /// If equals to true, will serialize the blockchain messages as json (for easy copying) instead of Rust Debug formatting
    pub serialize_json: bool,

    /// Optional - boolean
    /// Defaults to "false"
    /// Disable manual interactions
    /// It allows to automate scripting and get rid of prompting
    pub disable_manual_interaction: bool,
}

impl CoreEnvVars {
    pub fn load() -> Result<Self, CwEnvError> {
        let mut env_values = CoreEnvVars::default();

        if let Ok(str_value) = env::var(ARTIFACTS_DIR_ENV_NAME) {
            env_values.artifacts_dir = Some(PathBuf::from_str(&str_value).unwrap());
        }
        if let Ok(str_value) = env::var(SERIALIZE_ENV_NAME) {
            env_values.serialize_json = str_value.parse()?;
        }
        if let Ok(str_value) = env::var(DISABLE_MANUAL_INTERACTION_ENV_NAME) {
            env_values.disable_manual_interaction = str_value.parse()?;
        }
        Ok(env_values)
    }
}
