# Environment Variables

```bash
# .env

# info, debug, trace
RUST_LOG=info

# where the contract wasms are located
ARTIFACTS_DIR="../artifacts"

# where to store the output state data
DAEMON_STATE_PATH="./daemon_state.json"

# Mnemonics of the account that will be used to sign transactions
LOCAL_MNEMONIC=""
TEST_MNEMONIC=""
MAIN_MNEMONIC=""
```

> IMPORTANT: Make sure to exclude the .env file in your gitignore.