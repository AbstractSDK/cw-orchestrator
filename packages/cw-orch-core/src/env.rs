use std::{env, fmt::Display};

/// This regroups all env variables used by cw-orch-daemon. It allows for easier documentation and env variable management
/// This is used to import environment variables with safe names (and at a centralized location)
/// To get the env variable, you can use
/// ```rust,no_run
/// use cw_orch_core::env::CwOrchEnvVars;
/// let env_variable = CwOrchEnvVars::StateFolder.get().unwrap();
/// ```
/// You can get the env variable name with :
/// ```rust,no_run
/// use cw_orch_core::env::CwOrchEnvVars;
/// let variable_name = CwOrchEnvVars::StateFolder.to_string();
/// ```
pub enum CwOrchEnvVars {
    /// Optional - Absolute Path
    /// Defaults to "~./cw-orchestrator"
    /// This is the folder in which states of contracts are saved
    /// This is not enforced to be an absolute path but this is highly recommended
    StateFolder,

    /// Optional
    /// This is the name of the state file
    /// If the path is relative, this is taken from StateFolder
    /// Defaults to "state.json"
    StateFile,

    /// Optional
    /// Where cw-orch will look for wasm files. This is used by `ArtifactsDir::env()``
    ArtifactsDir,

    /// Optional - Float
    /// Defaults to 1.3
    /// This allows changing the gas buffer applied after tx simulation
    GasBuffer,

    /// Optional - Integer
    /// Defaults to 50
    /// This changes the number of tx queries before it fails if it doesn't find any result
    MaxTxQueryRetries,

    /// Optional - Integer
    /// Defaults to 1
    /// Minimum block speed in seconds. Useful when the block speeds are varying a lot
    MinBlockSpeed,

    /// Optional - String
    /// Defaults to "false"
    /// If equals to "true", will serialize the blockchain messages as json (for easy copying) instead of Rust Debug formatting
    SerializeJson,

    /// Optional - String
    /// Mandatory when interacting with a daemon on mainnet
    /// Mnemonic of the address interacting with a mainnet
    MainMnemonic,

    /// Optional - String
    /// Mandatory when interacting with a daemon on mainnet
    /// Mnemonic of the address interacting with a testnet
    TestMnemonic,

    /// Optional - String
    /// Mandatory when interacting with a daemon on mainnet
    /// Mnemonic of the address interacting with a localnet
    LocalMnemonic,
}

impl CwOrchEnvVars {
    pub fn get(&self) -> Result<String, env::VarError> {
        env::var(self.to_string())
    }
}

impl Display for CwOrchEnvVars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            CwOrchEnvVars::StateFolder => "CW_ORCH_STATE_FOLDER",
            CwOrchEnvVars::StateFile => "STATE_FILE",
            CwOrchEnvVars::ArtifactsDir => "ARTIFACTS_DIR",
            CwOrchEnvVars::GasBuffer => "CW_ORCH_GAS_BUFFER",
            CwOrchEnvVars::MaxTxQueryRetries => "CW_ORCH_MAX_TX_QUERY_RETRIES",
            CwOrchEnvVars::MinBlockSpeed => "CW_ORCH_MIN_BLOCK_SPEED",
            CwOrchEnvVars::SerializeJson => "CW_ORCH_SERIALIZE_JSON",

            CwOrchEnvVars::MainMnemonic => "MAIN_MNEMONIC",
            CwOrchEnvVars::TestMnemonic => "TEST_MNEMONIC",
            CwOrchEnvVars::LocalMnemonic => "LOCAL_MNEMONIC",
        })
    }
}
