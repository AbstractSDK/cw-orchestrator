# Environment Variables

cw-orch leverages some environment variables to interact with contracts on actual blockchains. The environment variables are described here. You can find additional information about their usage, default values and types in the [`cw-orch` repo].

**IMPORTANT: Before proceeding, ensure that you add `.env` to your `.gitignore`. We are not responsible for any loss of funds due to leaked mnemonics.**

Here are the optional environment variables:

```bash
# .env

# info, debug, trace (if using env_logger for logging)
RUST_LOG=info

# Where the contract wasms are located (used by ArtifactsDir::env())
ARTIFACTS_DIR="../artifacts"

# where to store the state of your deployments (default: ./state.json)
STATE_FILE="./my_state.json"

# Mnemonics of the account that will be used to sign transactions
# Can optionally be set on DaemonBuilder as well.
MAIN_MNEMONIC="" # Necessary if interacting with a cw-orch-daemon on mainnet
TEST_MNEMONIC="" # Necessary if interacting with a cw-orch-daemon on testnet
LOCAL_MNEMONIC="" # Necessary if interacting with a cw-orch-daemon locally


## Additional configuration variables. These are optional. We show default values here : 
# Optional - Float. This allows changing the gas buffer applied after tx simulation
CW_ORCH_GAS_BUFFER = 1.3 
# Optional - Integer. This changes the number of tx queries before it fails if it doesn't find any result
CW_ORCH_MAX_TX_QUERY_RETRIES = 50 
# Optional - Integer. Minimum block speed in seconds. Useful when the block speeds are varying a lot
CW_ORCH_MIN_BLOCK_SPEED = 1 
# Optional - String. If equals to "true", will serialize the blockchain messages as json (for easy copying) instead of Rust Debug formatting
CW_ORCH_SERIALIZE_JSON = "false" 
# Optional - Absolute Path. Sets the directory where the state file will be saved.
# This is not enforced to be an absolute path but this is highly recommended
CW_ORCH_STATE_FOLDER = "~/.cw-orchestrator"

```

## Mnemonics

**Only 24-word mnemonics are supported at this time.** If you're experienced with keychain and private key management we'd really appreciate your help in adding support for other formats. Please reach out to us on [Discord](https://discord.gg/uch3Tq3aym) if you're interested in helping out.
