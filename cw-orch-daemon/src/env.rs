//! This regroups all env variables used by cw-orch-daemon. It allows for easier documentation and env variable management
//! This is used to import environment variables with safe names (and at a centralized location)
//! To get the env variable parsed value, you can use
//! ```rust,no_run
//! use cw_orch_daemon::env::DaemonEnvVars;
//! let env_variable = DaemonEnvVars::state_file();
//! ```

use std::{env, path::PathBuf, str::FromStr};

use cosmwasm_std::StdError;
use std::time::Duration;

const DEFAULT_TX_QUERY_RETRIES: usize = 50;

#[deprecated(since = "0.24.0", note = "Please use BLOCK_TIME_MIN_ENV_NAME instead")]
pub const MIN_BLOCK_SPEED_ENV_NAME: &str = "CW_ORCH_MIN_BLOCK_SPEED";

pub const BLOCK_TIME_MIN_ENV_NAME: &str = "CW_ORCH_MIN_BLOCK_TIME";
pub const BLOCK_TIME_MAX_ENV_NAME: &str = "CW_ORCH_MAX_BLOCK_TIME";
pub const STATE_FILE_ENV_NAME: &str = "STATE_FILE";
pub const GAS_BUFFER_ENV_NAME: &str = "CW_ORCH_GAS_BUFFER";
pub const MIN_GAS_ENV_NAME: &str = "CW_ORCH_MIN_GAS";
pub const MAX_TX_QUERIES_RETRY_ENV_NAME: &str = "CW_ORCH_MAX_TX_QUERY_RETRIES";
pub const WALLET_BALANCE_ASSERTION_ENV_NAME: &str = "CW_ORCH_WALLET_BALANCE_ASSERTION";
pub const LOGS_ACTIVATION_MESSAGE_ENV_NAME: &str = "CW_ORCH_LOGS_ACTIVATION_MESSAGE";

pub const MAIN_MNEMONIC_ENV_NAME: &str = "MAIN_MNEMONIC";
pub const TEST_MNEMONIC_ENV_NAME: &str = "TEST_MNEMONIC";
pub const LOCAL_MNEMONIC_ENV_NAME: &str = "LOCAL_MNEMONIC";
pub struct DaemonEnvVars {}
impl DaemonEnvVars {
    /// Optional - Path
    /// This is the path to the state file
    /// `folder/file.json` will resolve to `~/.cw-orchestrator/folder/file.json`
    /// `./folder/file.json` will resolve `$pwd/folder/file.json`
    /// `../folder/file.json` will resolve `$pwd/../folder/file.json`
    /// `/usr/var/file.json` will resolve to `/usr/var/file.json`
    /// Defaults to "~./cw-orchestrator/state.json"
    pub fn state_file() -> PathBuf {
        let state_file_string = env::var(STATE_FILE_ENV_NAME).unwrap_or("state.json".to_string());
        parse_with_log(state_file_string, STATE_FILE_ENV_NAME)
    }

    /// Optional - Float
    /// This allows changing the gas buffer applied after tx simulation
    /// If not specified, a more complex algorithm is applied for dealing with small gas fee cases
    pub fn gas_buffer() -> Option<f64> {
        if let Ok(str_value) = env::var(GAS_BUFFER_ENV_NAME) {
            Some(parse_with_log(str_value, GAS_BUFFER_ENV_NAME))
        } else {
            None
        }
    }

    /// Optional - Integer
    /// Defaults to 150_000
    /// Minimum gas amount. Useful when transaction still won't pass even when setting a high gas_buffer or for mixed transaction scripts
    pub fn min_gas() -> u64 {
        if let Ok(str_value) = env::var(MIN_GAS_ENV_NAME) {
            parse_with_log(str_value, MIN_GAS_ENV_NAME)
        } else {
            150_000
        }
    }

    /// Optional - Integer
    /// Defaults to [`DEFAULT_TX_QUERY_RETRIES`]
    /// This changes the number of tx queries before it fails if it doesn't find any result
    pub fn max_tx_query_retries() -> usize {
        if let Ok(str_value) = env::var(MAX_TX_QUERIES_RETRY_ENV_NAME) {
            parse_with_log(str_value, MAX_TX_QUERIES_RETRY_ENV_NAME)
        } else {
            DEFAULT_TX_QUERY_RETRIES
        }
    }

    /// Optional - Block time
    /// Defaults to 1s
    /// Minimum block time in `Duration`. Useful when the block speeds are varying a lot
    #[allow(deprecated)]
    pub fn min_block_time() -> Duration {
        if let Ok(str_value) =
            env::var(BLOCK_TIME_MIN_ENV_NAME).or(env::var(MIN_BLOCK_SPEED_ENV_NAME))
        {
            parse_block_time_duration(&str_value)
        } else {
            Duration::from_secs(1)
        }
    }

    /// Optional - Block time
    /// Defaults to None
    /// Maximum block time in `Duration`. Useful when the block speeds are varying a lot
    pub fn max_block_time() -> Option<Duration> {
        if let Ok(str_value) = env::var(BLOCK_TIME_MAX_ENV_NAME) {
            Some(parse_block_time_duration(&str_value))
        } else {
            None
        }
    }

    /// Optional - boolean
    /// Defaults to "true"
    /// Disable wallet balance assertion.
    /// When balance assertion is enabled, it asserts that the balance of the sender is sufficient before submitting any transactions (during the simulation step)
    pub fn wallet_balance_assertion() -> bool {
        if let Ok(str_value) = env::var(WALLET_BALANCE_ASSERTION_ENV_NAME) {
            parse_with_log(str_value, WALLET_BALANCE_ASSERTION_ENV_NAME)
        } else {
            true
        }
    }

    /// Optional - boolean
    /// Defaults to "true"
    /// Disable the "Enable Logs" message
    /// It allows forcing cw-orch to not output anything
    pub fn logs_message() -> bool {
        if let Ok(str_value) = env::var(LOGS_ACTIVATION_MESSAGE_ENV_NAME) {
            parse_with_log(str_value, LOGS_ACTIVATION_MESSAGE_ENV_NAME)
        } else {
            true
        }
    }

    /// Optional - String
    /// Mandatory when interacting with a daemon on mainnet
    /// Mnemonic of the address interacting with a mainnet
    pub fn main_mnemonic() -> Option<String> {
        env::var(MAIN_MNEMONIC_ENV_NAME).ok()
    }

    /// Optional - String
    /// Mandatory when interacting with a daemon on mainnet
    /// Mnemonic of the address interacting with a testnet
    pub fn test_mnemonic() -> Option<String> {
        env::var(TEST_MNEMONIC_ENV_NAME).ok()
    }

    /// Optional - String
    /// Mandatory when interacting with a daemon on mainnet
    /// Mnemonic of the address interacting with a localnet
    pub fn local_mnemonic() -> Option<String> {
        env::var(LOCAL_MNEMONIC_ENV_NAME).ok()
    }
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

fn parse_with_log<F: FromStr<Err = E>, E: std::fmt::Display>(
    value: String,
    env_var_name: &str,
) -> F {
    match value.parse() {
        Ok(parsed) => parsed,
        Err(e) => panic!("Couldn't parse content of env var {env_var_name}, error : {e}"),
    }
}

/// Parse block time duration from duration string
/// Takes duration in format `{integer}{duration_specifier}`
///
/// # Duration specifiers
///
/// * `s` - Block time in seconds
/// * `ms` - Block time in milliseconds
/// * `` - Defaults to `s`
///
/// # Examples
///
/// - "123s" == Duration::from_secs(123)
/// - "321ms" == Duration::from_millis(321)
/// - "42" == Duration::from_secs(42)
fn parse_block_time_duration(raw_duration: &str) -> Duration {
    let (digits, duration_specifier) = match raw_duration.find(|c: char| !c.is_ascii_digit()) {
        // Found non-digit character, split string
        Some(char_idx) => {
            let (digits, not_digits) = raw_duration.split_at(char_idx);
            (digits, not_digits.trim())
        }
        // Default to seconds
        None => (raw_duration, "s"),
    };

    let duration: u64 = match digits.parse() {
        Ok(duration) => duration,
        Err(e) => panic!("Couldn't parse content of block time, error: {e}"),
    };

    match duration_specifier {
        "s" => Duration::from_secs(duration),
        "ms" => Duration::from_millis(duration),
        _ => panic!("Couldn't parse content of block time, error: unexpected token after digits"),
    }
}

#[cfg(test)]
mod test_parse {
    use super::*;

    #[test]
    fn test_parse_block_time_duration() {
        assert_eq!(parse_block_time_duration("123s"), Duration::from_secs(123));
        assert_eq!(
            parse_block_time_duration("321ms"),
            Duration::from_millis(321)
        );
        assert_eq!(parse_block_time_duration("42"), Duration::from_secs(42));
        assert_eq!(
            parse_block_time_duration("12345 s"),
            Duration::from_secs(12345)
        );
        assert_eq!(
            parse_block_time_duration("54321 ms "),
            Duration::from_millis(54321)
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_empty_block_time_duration() {
        parse_block_time_duration("");
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_token_block_time_duration() {
        parse_block_time_duration("45d");
    }

    #[test]
    #[should_panic]
    fn test_parse_invalid_format_block_time_duration() {
        parse_block_time_duration("s54");
    }
}
