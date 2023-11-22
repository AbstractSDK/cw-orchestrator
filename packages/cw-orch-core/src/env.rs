//! This regroups all env variables used by cw-orch-daemon. It allows for easier documentation and env variable management
//! This is used to import environment variables with safe names (and at a centralized location)
//! To get the env variable parsed value, you can use
//! ```rust,no_run
//! use cw_orch_core::CwOrchEnvVars;
//! let env_variable = CwOrchEnvVars::load().unwrap().state_folder;
//! ```

use std::{env, path::PathBuf, str::FromStr};

use crate::CwEnvError;

const DEFAULT_TX_QUERY_RETRIES: usize = 50;

pub const STATE_FOLDER_ENV_NAME: &str = "CW_ORCH_STATE_FOLDER";
pub const STATE_FILE_ENV_NAME: &str = "STATE_FILE";
pub const ARTIFACTS_DIR_ENV_NAME: &str = "ARTIFACTS_DIR";
pub const GAS_BUFFER_ENV_NAME: &str = "CW_ORCH_GAS_BUFFER";
pub const MIN_GAS_ENV_NAME: &str = "CW_ORCH_MIN_GAS";
pub const MAX_TX_QUERIES_RETRY_ENV_NAME: &str = "CW_ORCH_MAX_TX_QUERY_RETRIES";
pub const MIN_BLOCK_SPEED_ENV_NAME: &str = "CW_ORCH_MIN_BLOCK_SPEED";
pub const SERIALIZE_ENV_NAME: &str = "CW_ORCH_SERIALIZE_JSON";
pub const DISABLE_WALLET_BALANCE_ASSERTION_ENV_NAME: &str =
    "CW_ORCH_DISABLE_WALLET_BALANCE_ASSERTION";
pub const DISABLE_MANUAL_INTERACTION_ENV_NAME: &str = "CW_ORCH_DISABLE_MANUAL_INTERACTION";
pub const MAIN_MNEMONIC_ENV_NAME: &str = "MAIN_MNEMONIC";
pub const TEST_MNEMONIC_ENV_NAME: &str = "TEST_MNEMONIC";
pub const LOCAL_MNEMONIC_ENV_NAME: &str = "LOCAL_MNEMONIC";

pub struct CwOrchEnvVars {
    /// Optional - Absolute Path
    /// Defaults to "~./cw-orchestrator"
    /// This is the folder in which states of contracts are saved
    /// This is not enforced to be an absolute path but this is highly recommended
    pub state_folder: Option<PathBuf>,

    /// Optional - Path
    /// /// This is the name of the state file
    /// If the path is relative, this is taken from StateFolder
    /// Defaults to "state.json"
    pub state_file: PathBuf,

    /// Optional - Path
    /// Where cw-orch will look for wasm files. This is used by `ArtifactsDir::env()``
    pub artifacts_dir: Option<PathBuf>,

    /// Optional - Float
    /// This allows changing the gas buffer applied after tx simulation
    /// If not specified, a more complex algorithm is applied for dealing with small gas fee cases
    pub gas_buffer: Option<f64>,

    /// Optional - Integer
    /// Defaults to None
    /// Minimum gas amount. Useful when transaction still won't pass even when setting a high gas_buffer or for mixed transaction scripts
    pub min_gas: Option<u64>,

    /// Optional - Integer
    /// Defaults to [`DEFAULT_TX_QUERY_RETRIES`]
    /// This changes the number of tx queries before it fails if it doesn't find any result
    pub max_tx_query_retries: usize,

    /// Optional - Integer
    /// Defaults to 1
    /// Minimum block speed in seconds. Useful when the block speeds are varying a lot
    pub min_block_speed: u64,

    /// Optional - Boolean
    /// Defaults to false
    /// If equals to true, will serialize the blockchain messages as json (for easy copying) instead of Rust Debug formatting
    pub serialize_json: bool,

    /// Optional - String
    /// Mandatory when interacting with a daemon on mainnet
    /// Mnemonic of the address interacting with a mainnet
    pub main_mnemonic: Option<String>,

    /// Optional - String
    /// Mandatory when interacting with a daemon on mainnet
    /// Mnemonic of the address interacting with a testnet
    pub test_mnemonic: Option<String>,

    /// Optional - String
    /// Mandatory when interacting with a daemon on mainnet
    /// Mnemonic of the address interacting with a localnet
    pub local_mnemonic: Option<String>,

    /// Optional - boolean
    /// Defaults to "false"
    /// Disable wallet balance assertion.
    /// When balance assertion is enabled, it asserts that the balance of the sender is sufficient before submitting any transactions (during the simulation step)
    pub disable_wallet_balance_assertion: bool,

    /// Optional - boolean
    /// Defaults to "false"
    /// Disable manual interactions
    /// It allows to automate scripting and get rid of prompting
    pub disable_manual_interaction: bool,
}

impl Default for CwOrchEnvVars {
    fn default() -> Self {
        CwOrchEnvVars {
            state_folder: dirs::home_dir().map(|home| home.join(".cw-orchestrator")),
            state_file: PathBuf::from_str("state.json").unwrap(),
            artifacts_dir: None,
            gas_buffer: None,
            min_gas: None,
            max_tx_query_retries: DEFAULT_TX_QUERY_RETRIES,
            min_block_speed: 1,
            serialize_json: false,
            main_mnemonic: None,
            test_mnemonic: None,
            local_mnemonic: None,
            disable_wallet_balance_assertion: false,
            disable_manual_interaction: false,
        }
    }
}

impl CwOrchEnvVars {
    pub fn load() -> Result<Self, CwEnvError> {
        let mut env_values = CwOrchEnvVars::default();

        // Then we load the values from env
        if let Ok(str_value) = env::var(STATE_FOLDER_ENV_NAME) {
            env_values.state_folder = Some(PathBuf::from_str(&str_value).unwrap());
        }
        if let Ok(str_value) = env::var(STATE_FILE_ENV_NAME) {
            env_values.state_file = PathBuf::from_str(&str_value).unwrap();
        }
        if let Ok(str_value) = env::var(ARTIFACTS_DIR_ENV_NAME) {
            env_values.artifacts_dir = Some(PathBuf::from_str(&str_value).unwrap());
        }
        if let Ok(str_value) = env::var(GAS_BUFFER_ENV_NAME) {
            env_values.gas_buffer = Some(str_value.parse()?);
        }
        if let Ok(str_value) = env::var(MAX_TX_QUERIES_RETRY_ENV_NAME) {
            env_values.max_tx_query_retries = str_value.parse()?;
        }
        if let Ok(str_value) = env::var(MIN_BLOCK_SPEED_ENV_NAME) {
            env_values.min_block_speed = str_value.parse()?;
        }
        if let Ok(str_value) = env::var(SERIALIZE_ENV_NAME) {
            env_values.serialize_json = str_value.parse()?;
        }
        if let Ok(str_value) = env::var(DISABLE_WALLET_BALANCE_ASSERTION_ENV_NAME) {
            env_values.disable_wallet_balance_assertion = str_value.parse()?;
        }
        if let Ok(str_value) = env::var(DISABLE_MANUAL_INTERACTION_ENV_NAME) {
            env_values.disable_manual_interaction = str_value.parse()?;
        }
        if let Ok(str_value) = env::var(MAIN_MNEMONIC_ENV_NAME) {
            env_values.main_mnemonic = Some(str_value);
        }
        if let Ok(str_value) = env::var(TEST_MNEMONIC_ENV_NAME) {
            env_values.test_mnemonic = Some(str_value);
        }
        if let Ok(str_value) = env::var(LOCAL_MNEMONIC_ENV_NAME) {
            env_values.local_mnemonic = Some(str_value);
        }
        Ok(env_values)
    }
}
