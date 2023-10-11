use std::env;

/// This regroups all env variables used by cw-orch-daemon. It allows for easier documentation and env variable management
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
        env::var(self.name())
    }

    pub fn name(&self) -> String {
        match self {
            CwOrchEnvVars::StateFolder => "CW_ORCH_STATE_FOLDER".to_string(),
            CwOrchEnvVars::StateFile => "STATE_FILE".to_string(),
            CwOrchEnvVars::ArtifactsDir => "ARTIFACTS_DIR".to_string(),
            CwOrchEnvVars::GasBuffer => "CW_ORCH_GAS_BUFFER".to_string(),
            CwOrchEnvVars::MaxTxQueryRetries => "CW_ORCH_MAX_TX_QUERY_RETRIES".to_string(),
            CwOrchEnvVars::MinBlockSpeed => "CW_ORCH_MIN_BLOCK_SPEED".to_string(),
            CwOrchEnvVars::SerializeJson => "CW_ORCH_SERIALIZE_JSON".to_string(),

            CwOrchEnvVars::MainMnemonic => "MAIN_MNEMONIC".to_string(),
            CwOrchEnvVars::TestMnemonic => "TEST_MNEMONIC".to_string(),
            CwOrchEnvVars::LocalMnemonic => "LOCAL_MNEMONIC".to_string(),
        }
    }
}
