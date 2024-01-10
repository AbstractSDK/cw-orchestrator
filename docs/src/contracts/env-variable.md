# Environment Variables

cw-orch leverages some environment variables to interact with contracts on actual blockchains. The environment variables are described here. You can find additional information about their usage, default values and types in the <a href="https://github.com/AbstractSDK/cw-orchestrator/blob/main/packages/cw-orch-core/src/env.rs" target="_blank">`cw-orch` repo</a>.

**IMPORTANT: Before proceeding, ensure that you add `.env` to your `.gitignore`. We are not responsible for any loss of funds due to leaked mnemonics.**

Here are the optional environment variables:

```bash
# .env

# info, debug, trace (if using env_logger for logging)
RUST_LOG=info

# Where the contract wasms are located (used by ArtifactsDir::env())
ARTIFACTS_DIR="../artifacts"

# Optional - Path. 
# Sets the location of the state file for your deployments (default: ~./cw-orchestrator/state.json)
# `folder/file.json` will resolve to `~/.cw-orchestrator/folder/file.json`
# `./folder/file.json` will resolve `$pwd/folder/file.json`
# `/usr/var/file.json` will resolve to `/usr/var/file.json`
STATE_FILE="./my_state.json"

# Mnemonics of the account that will be used to sign transactions
# Can optionally be set on DaemonBuilder as well.
MAIN_MNEMONIC="" # Necessary if interacting with a cw-orch-daemon on mainnet
TEST_MNEMONIC="" # Necessary if interacting with a cw-orch-daemon on testnet
LOCAL_MNEMONIC="" # Necessary if interacting with a cw-orch-daemon locally


## Additional configuration variables. These are optional. We show default values here: 
# Optional - Float. This allows changing the gas buffer applied after tx simulation
CW_ORCH_GAS_BUFFER = 1.3 
# Optional - Integer. This changes the number of tx queries before it fails if it doesn't find any result
CW_ORCH_MAX_TX_QUERY_RETRIES = 50 
# Optional - Integer. Minimum block speed in seconds. Useful when the block speeds are varying a lot
CW_ORCH_MIN_BLOCK_SPEED = 1 
# Optional - String. If equals to "true", will serialize the blockchain messages as json (for easy copying) instead of Rust Debug formatting
CW_ORCH_SERIALIZE_JSON = "false" 
# Optional - Integer. This allows setting a minimum of gas used when broadcasting transactions
CW_ORCH_MIN_GAS = 100000
# Optional - boolean.  Disable the "Enable Logs" message.
CW_ORCH_DISABLE_ENABLE_LOGS_MESSAGE = "false"
```

## Mnemonics

**Only 24-word mnemonics are supported at this time.** If you're experienced with keychain and private key management we'd really appreciate your help in adding support for other formats. Please reach out to us on <a href="https://discord.gg/uch3Tq3aym" target="_blank">Discord</a> if you're interested in helping out.
