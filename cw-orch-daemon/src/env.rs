//! This regroups all env variables used by cw-orch-daemon. It allows for easier documentation and env variable management
//! This is used to import environment variables with safe names (and at a centralized location)
//! To get the env variable parsed value, you can use
//! ```rust,no_run
//! use cw_orch_core::DaemonEnvVars;
//! let env_variable = DaemonEnvVars::load().unwrap().state_file;
//! ```

use std::{env, path::PathBuf, str::FromStr};

use cosmwasm_std::StdError;
use cw_orch_core::CwEnvError;

const DEFAULT_TX_QUERY_RETRIES: usize = 50;

pub const STATE_FILE_ENV_NAME: &str = "STATE_FILE";
pub const GAS_BUFFER_ENV_NAME: &str = "CW_ORCH_GAS_BUFFER";
pub const MIN_GAS_ENV_NAME: &str = "CW_ORCH_MIN_GAS";
pub const MAX_TX_QUERIES_RETRY_ENV_NAME: &str = "CW_ORCH_MAX_TX_QUERY_RETRIES";
pub const MIN_BLOCK_SPEED_ENV_NAME: &str = "CW_ORCH_MIN_BLOCK_SPEED";
pub const DISABLE_WALLET_BALANCE_ASSERTION_ENV_NAME: &str =
    "CW_ORCH_DISABLE_WALLET_BALANCE_ASSERTION";
pub const DISABLE_LOGS_ACTIVATION_MESSAGE_ENV_NAME: &str =
    "CW_ORCH_DISABLE_LOGS_ACTIVATION_MESSAGE";

pub struct DaemonEnvVars {
    /// Optional - Path
    /// This is the path to the state file
    /// `folder/file.json` will resolve to `~/.cw-orchestrator/folder/file.json`
    /// `./folder/file.json` will resolve `$pwd/folder/file.json`
    /// `../folder/file.json` will resolve `$pwd/../folder/file.json`
    /// `/usr/var/file.json` will resolve to `/usr/var/file.json`
    /// Defaults to "~./cw-orchestrator/state.json"
    pub state_file: PathBuf,

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

    /// Optional - boolean
    /// Defaults to "false"
    /// Disable wallet balance assertion.
    /// When balance assertion is enabled, it asserts that the balance of the sender is sufficient before submitting any transactions (during the simulation step)
    pub disable_wallet_balance_assertion: bool,

    /// Optional - boolean
    /// Defaults to "false"
    /// Disable the "Enable Logs" message
    /// It allows forcing cw-orch to not output anything
    pub disable_logs_message: bool,
}

/// Fetches the default state folder.
/// This function should only error if the home_dir is not set and the `dirs` library is unable to fetch it
/// This happens only in rare cases
pub fn default_state_folder() -> Result<PathBuf, StdError> {
    dirs::home_dir().map(|home| home.join(".cw-orchestrator"))
        .ok_or( StdError::generic_err(
            format!(
                "Your machine doesn't have a home folder. You can't use relative path for the state file such as 'state.json'. 
                Please use an absolute path ('/home/root/state.json') or a dot-prefixed-relative path ('./state.json') in the {} env variable.",
                STATE_FILE_ENV_NAME
            )))
}

impl Default for DaemonEnvVars {
    fn default() -> Self {
        DaemonEnvVars {
            state_file: PathBuf::from_str("state.json").unwrap(),
            gas_buffer: None,
            min_gas: None,
            max_tx_query_retries: DEFAULT_TX_QUERY_RETRIES,
            min_block_speed: 1,
            disable_wallet_balance_assertion: false,
            disable_logs_message: false,
        }
    }
}

impl DaemonEnvVars {
    pub fn load() -> Result<Self, CwEnvError> {
        let mut env_values = DaemonEnvVars::default();

        // Then we load the values from env
        if let Ok(str_value) = env::var(STATE_FILE_ENV_NAME) {
            env_values.state_file = PathBuf::from_str(&str_value).unwrap();
        }
        if let Ok(str_value) = env::var(GAS_BUFFER_ENV_NAME) {
            env_values.gas_buffer = Some(str_value.parse()?);
        }
        if let Ok(str_value) = env::var(MIN_GAS_ENV_NAME) {
            env_values.min_gas = Some(str_value.parse()?);
        }
        if let Ok(str_value) = env::var(MAX_TX_QUERIES_RETRY_ENV_NAME) {
            env_values.max_tx_query_retries = str_value.parse()?;
        }
        if let Ok(str_value) = env::var(MIN_BLOCK_SPEED_ENV_NAME) {
            env_values.min_block_speed = str_value.parse()?;
        }
        if let Ok(str_value) = env::var(DISABLE_WALLET_BALANCE_ASSERTION_ENV_NAME) {
            env_values.disable_wallet_balance_assertion = str_value.parse()?;
        }
        if let Ok(str_value) = env::var(DISABLE_LOGS_ACTIVATION_MESSAGE_ENV_NAME) {
            env_values.disable_logs_message = str_value.parse()?;
        }
        Ok(env_values)
    }
}
