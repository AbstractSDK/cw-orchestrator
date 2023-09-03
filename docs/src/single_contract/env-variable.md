# Environment Variables

cw-orch needs some environment variables to be set in order to function properly when running as an executable.

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
LOCAL_MNEMONIC=""
TEST_MNEMONIC=""
MAIN_MNEMONIC=""
```

## Mnemonics

**Only 24-word mnemonics are supported at this time.** If you're experienced with keychain and private key management we'd really appreciate your help in adding support for other formats. Please reach out to us on [Discord](https://discord.gg/uch3Tq3aym) if you're interested in helping out.
