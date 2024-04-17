//! This regroups all env variables used by cw-orch-daemon. It allows for easier documentation and env variable management
//! This is used to import environment variables with safe names (and at a centralized location)
//! To get the env variable parsed value, you can use
//! ```rust,no_run
//! use cw_orch_core::NetworkEnvVars;
//! let env_variable = NetworkEnvVars::load().unwrap().state_file;
//! ```

use std::env;

use cw_orch_core::CwEnvError;

pub const MAIN_MNEMONIC_ENV_NAME: &str = "MAIN_MNEMONIC";
pub const TEST_MNEMONIC_ENV_NAME: &str = "TEST_MNEMONIC";
pub const LOCAL_MNEMONIC_ENV_NAME: &str = "LOCAL_MNEMONIC";

#[derive(Default)]
pub struct NetworkEnvVars {
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
}

impl NetworkEnvVars {
    pub fn load() -> Result<Self, CwEnvError> {
        let mut env_values = NetworkEnvVars::default();

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
