//! This regroups all env variables used by cw-orch-daemon. It allows for easier documentation and env variable management
//! This is used to import environment variables with safe names (and at a centralized location)
//! To get the env variable, you can use
//! ```rust,no_run
//! use cw_orch_core::env::CwOrchEnvVars;
//! let env_variable = CwOrchEnvVars::StateFolder::parsed().unwrap();
//! ```
//! You can get the env variable name with :
//! ```rust,no_run
//! use cw_orch_core::env::CwOrchEnvVars;
//! let variable_name = CwOrchEnvVars::StateFolder::parsed()to_string();
//! ```

use std::{
    env::{self, VarError},
    path::PathBuf,
    str::FromStr,
};

use crate::CwEnvError;

/// Optional - Absolute Path
/// Defaults to "~./cw-orchestrator"
/// This is the folder in which states of contracts are saved
/// This is not enforced to be an absolute path but this is highly recommended
pub struct StateFolder;

/// Optional - Path
/// /// This is the name of the state file
/// If the path is relative, this is taken from StateFolder
/// Defaults to "state.json"
pub struct StateFile;

/// Optional - Path
/// Where cw-orch will look for wasm files. This is used by `ArtifactsDir::env()``
pub struct ArtifactsDir;

/// Optional - Float
/// Defaults to 1.3
/// This allows changing the gas buffer applied after tx simulation
pub struct GasBuffer;

/// Optional - Integer
/// Defaults to 50
/// This changes the number of tx queries before it fails if it doesn't find any result
pub struct MaxTxQueryRetries;
const MAX_TX_QUERY_RETRIES: usize = 50;

/// Optional - Integer
/// Defaults to 1
/// Minimum block speed in seconds. Useful when the block speeds are varying a lot
pub struct MinBlockSpeed;

/// Optional - Boolean
/// Defaults to false
/// If equals to true, will serialize the blockchain messages as json (for easy copying) instead of Rust Debug formatting
pub struct SerializeJson;

/// Optional - String
/// Mandatory when interacting with a daemon on mainnet
/// Mnemonic of the address interacting with a mainnet
pub struct MainMnemonic;

/// Optional - String
/// Mandatory when interacting with a daemon on mainnet
/// Mnemonic of the address interacting with a testnet
pub struct TestMnemonic;

/// Optional - String
/// Mandatory when interacting with a daemon on mainnet
/// Mnemonic of the address interacting with a localnet
pub struct LocalMnemonic;

pub trait EnvVar {
    type EnvVarType;
    const ENV_VAR_NAME: &'static str;

    fn _parse(value: String) -> Result<Self::EnvVarType, CwEnvError>;

    fn default() -> Option<Self::EnvVarType>;

    fn parsed() -> Result<Self::EnvVarType, CwEnvError> {
        let optional_var = Self::get();
        match optional_var {
            Ok(value) => Self::_parse(value),
            Err(_) => Self::default().ok_or(VarError::NotPresent.into()),
        }
    }

    fn get() -> Result<String, env::VarError> {
        env::var(Self::ENV_VAR_NAME)
    }
}

impl EnvVar for SerializeJson {
    type EnvVarType = bool;
    const ENV_VAR_NAME: &'static str = "CW_ORCH_SERIALIZE_JSON";

    fn _parse(v: String) -> Result<Self::EnvVarType, CwEnvError> {
        Ok(v == "true")
    }

    fn default() -> Option<Self::EnvVarType> {
        Some(false)
    }
}

impl EnvVar for MinBlockSpeed {
    type EnvVarType = u64;
    const ENV_VAR_NAME: &'static str = "CW_ORCH_MIN_BLOCK_SPEED";

    fn _parse(v: String) -> Result<Self::EnvVarType, CwEnvError> {
        Ok(v.parse()?)
    }

    fn default() -> Option<Self::EnvVarType> {
        Some(1)
    }
}

impl EnvVar for MaxTxQueryRetries {
    type EnvVarType = usize;
    const ENV_VAR_NAME: &'static str = "CW_ORCH_MAX_TX_QUERY_RETRIES";

    fn _parse(v: String) -> Result<Self::EnvVarType, CwEnvError> {
        Ok(v.parse()?)
    }
    fn default() -> Option<Self::EnvVarType> {
        Some(MAX_TX_QUERY_RETRIES)
    }
}

impl EnvVar for GasBuffer {
    type EnvVarType = f64;
    const ENV_VAR_NAME: &'static str = "CW_ORCH_GAS_BUFFER";

    fn _parse(v: String) -> Result<Self::EnvVarType, CwEnvError> {
        Ok(v.parse()?)
    }

    fn default() -> Option<Self::EnvVarType> {
        None
    }
}

impl EnvVar for ArtifactsDir {
    type EnvVarType = PathBuf;
    const ENV_VAR_NAME: &'static str = "ARTIFACTS_DIR";

    fn _parse(v: String) -> Result<Self::EnvVarType, CwEnvError> {
        let path: PathBuf = PathBuf::from_str(&v).unwrap();
        Ok(path)
    }

    fn default() -> Option<Self::EnvVarType> {
        None
    }
}

impl EnvVar for StateFile {
    type EnvVarType = String;
    const ENV_VAR_NAME: &'static str = "STATE_FILE";

    fn _parse(v: String) -> Result<Self::EnvVarType, CwEnvError> {
        Ok(v)
    }

    fn default() -> Option<Self::EnvVarType> {
        Some("state.json".to_string())
    }
}

impl EnvVar for StateFolder {
    type EnvVarType = PathBuf;
    const ENV_VAR_NAME: &'static str = "CW_ORCH_STATE_FOLDER";

    fn _parse(v: String) -> Result<Self::EnvVarType, CwEnvError> {
        let path: PathBuf = PathBuf::from_str(&v).unwrap();
        Ok(path)
    }

    fn default() -> Option<Self::EnvVarType> {
        dirs::home_dir().map(|home| home.join(".cw-orchestrator"))
    }
}

impl EnvVar for MainMnemonic {
    type EnvVarType = String;
    const ENV_VAR_NAME: &'static str = "MAIN_MNEMONIC";

    fn _parse(v: String) -> Result<Self::EnvVarType, CwEnvError> {
        Ok(v)
    }

    fn default() -> Option<Self::EnvVarType> {
        None
    }
}

impl EnvVar for TestMnemonic {
    type EnvVarType = String;
    const ENV_VAR_NAME: &'static str = "TEST_MNEMONIC";

    fn _parse(v: String) -> Result<Self::EnvVarType, CwEnvError> {
        Ok(v)
    }

    fn default() -> Option<Self::EnvVarType> {
        None
    }
}

impl EnvVar for LocalMnemonic {
    type EnvVarType = String;
    const ENV_VAR_NAME: &'static str = "LOCAL_MNEMONIC";

    fn _parse(v: String) -> Result<Self::EnvVarType, CwEnvError> {
        Ok(v)
    }

    fn default() -> Option<Self::EnvVarType> {
        None
    }
}
