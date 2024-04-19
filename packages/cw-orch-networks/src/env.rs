//! This regroups all env variables used by cw-orch-daemon. It allows for easier documentation and env variable management
//! This is used to import environment variables with safe names (and at a centralized location)
//! To get the env variable parsed value, you can use
//! ```rust,no_run
//! use cw_orch_networks::NetworkEnvVars;
//! let env_variable = NetworkEnvVars::main_mnemonic();
//! ```

use std::env;

pub const MAIN_MNEMONIC_ENV_NAME: &str = "MAIN_MNEMONIC";
pub const TEST_MNEMONIC_ENV_NAME: &str = "TEST_MNEMONIC";
pub const LOCAL_MNEMONIC_ENV_NAME: &str = "LOCAL_MNEMONIC";

pub struct NetworkEnvVars;

impl NetworkEnvVars {
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
