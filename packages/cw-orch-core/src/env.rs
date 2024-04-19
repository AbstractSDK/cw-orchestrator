//! This regroups all env variables used by cw-orch-daemon. It allows for easier documentation and env variable management
//! This is used to import environment variables with safe names (and at a centralized location)
//! To get the env variable parsed value, you can use
//! ```rust,no_run
//! use cw_orch_core::CoreEnvVars;
//! let env_variable = CoreEnvVars::load().unwrap().artifacts_dir;
//! ```

use std::{env, path::PathBuf, str::FromStr};

use cosmwasm_std::StdError;

pub const ARTIFACTS_DIR_ENV_NAME: &str = "ARTIFACTS_DIR";
pub const SERIALIZE_ENV_NAME: &str = "CW_ORCH_SERIALIZE_JSON";
pub const DISABLE_MANUAL_INTERACTION_ENV_NAME: &str = "CW_ORCH_DISABLE_MANUAL_INTERACTION";

pub struct CoreEnvVars;

impl CoreEnvVars {
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
    pub fn artifacts_dir() -> Option<PathBuf> {
        if let Ok(str_value) = env::var(ARTIFACTS_DIR_ENV_NAME) {
            Some(parse_with_log(str_value, ARTIFACTS_DIR_ENV_NAME))
        } else {
            None
        }
    }

    /// Optional - Boolean
    /// Defaults to false
    /// If equals to true, will serialize the blockchain messages as json (for easy copying) instead of Rust Debug formatting
    pub fn serialize_json() -> bool {
        if let Ok(str_value) = env::var(SERIALIZE_ENV_NAME) {
            parse_with_log(str_value, SERIALIZE_ENV_NAME)
        } else {
            false
        }
    }

    /// Optional - boolean
    /// Defaults to "false"
    /// Disable manual interactions
    /// It allows to automate scripting and get rid of prompting
    pub fn disable_manual_interaction() -> bool {
        if let Ok(str_value) = env::var(DISABLE_MANUAL_INTERACTION_ENV_NAME) {
            parse_with_log(str_value, DISABLE_MANUAL_INTERACTION_ENV_NAME)
        } else {
            false
        }
    }
}

fn parse_with_log<F: FromStr<Err = E>, E: std::fmt::Display>(
    value: String,
    env_var_name: &str,
) -> F {
    value
        .parse()
        .map_err(|e| {
            StdError::generic_err(format!(
                "Couldn't parse content of env var {env_var_name}, error : {e}"
            ))
        })
        .unwrap()
}
