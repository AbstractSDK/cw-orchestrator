# Environment Variables

cw-orch leverages some environment variables to interact with contracts on actual blockchains. The environment variables are described here. You can find additional information about their usage, default values and types in the <a href="https://github.com/AbstractSDK/cw-orchestrator/blob/main/packages/cw-orch-core/src/env.rs" target="_blank">`cw-orch` repo</a>.

**IMPORTANT: Before proceeding, ensure that you add `.env` to your `.gitignore`. We are not responsible for any loss of funds due to leaked mnemonics.**

## Mnemonics

You can provide mnemonics directly to the `Daemon` using the following environment variables. Those variables will be used if the `mnemonic` setter is not used. See the [Daemon page](../integrations/daemon.md#configuration) for more detailed information on mnemonic usage.

- `MAIN_MNEMONIC` will be used when working with a Mainnet (`PHOENIX_1`, `OSMOSIS_1`...)
- `TEST_MNEMONIC` will be used when working with a Testnet (`PISCO_1`, `UNI_6`...)
- `LOCAL_MNEMONIC` will be used when working locally (`LOCAL_JUNO`...)

**Only 24-word mnemonics are supported at this time.** If you're experienced with keychain and private key management we'd really appreciate your help in adding support for other formats. Please reach out to us on <a href="https://discord.gg/uch3Tq3aym" target="_blank">Discord</a> if you're interested in helping out.

## Saving and Loading State

### STATE_FILE

Optional, accepted values: Path to a valid file
Default value: `~./cw-orchestrator/state.json`

This environment variable indicates the location of the state file that `cw-orch` will use to save on-chain state (addresses and code ids). Here is the behavior of this env variable:

- `folder/file.json` will resolve to `~/.cw-orchestrator/folder/file.json`
- `./folder/file.json` will resolve `$pwd/folder/file.json`
- `../folder/file.json` will resolve `$pwd/../folder/file.json`
- `/usr/var/file.json` will resolve to `/usr/var/file.json`

### ARTIFACTS_DIR

Optional, accepted values: Path to a valid directory

Path where the wasms will be fetched. This is used by `ArtifactsDir::env()` inside the code.
This is used in case you are using different tools than what is produced by <a href="https://github.com/CosmWasm/rust-optimizer" target="_blank">rust-optimizer</a>. Prefer using the following macro if you are using `wasm`s produced by `rust-optimizer`:

```rust,ignore
artifacts_dir_from_workspace!()
    .find_wasm_path("contract_name")
    .unwrap()
```

## Transaction options

### CW_ORCH_GAS_BUFFER

Optional, accepted values: float

This allows changing the gas buffer applied after tx simulation. Use this in case a transaction is blocked for insufficient gas reasons.

### CW_ORCH_MIN_GAS

Optional, accepted values: integer

Minimum gas amount for every transaction. Useful when transaction still won't pass even when setting a high gas_buffer or for mixed transaction scripts.

### CW_ORCH_MAX_TX_QUERY_RETRIES

Optional, accepted values: integer
Defaults to `50`.

Changes the number of tx queries (~1 query per block) before it fails if it doesn't find any result. Useful if the chain is slow or if the transaction has low gas price.

### CW_ORCH_MIN_BLOCK_SPEED

Optional, accepted value: integer in seconds
Defaults to `1`.

Minimum block speed in seconds. This is used internally by `cw-orch` when broadcasting transactions. Useful when the block speeds are varying a lot

### CW_ORCH_DISABLE_WALLET_BALANCE_ASSERTION

Optional, accepted values: `true`, `false`
Defaults to `false`

By default, `cw-orch` verifies that the account has sufficient balance to pay for gas fees. If it detects that the balance is too low, it propmts the user to fill up their wallet with gas tokens and allows for retrying at the press of a button.

If set to `true`, it won't check the user has enough balance before broadcasting transactions.

### CW_ORCH_DISABLE_MANUAL_INTERACTION

Optional, accepted values: `true`, `false`
Defaults to `false`

Some actions require user-intervention. If set to true, this disables those interventions. Useful when working in scripts that can't handle user-intervention.

Supported actions:

- Balance checks. When set to `true`, if the gas token balance is too low to submit a transaction, it will error. See [Disable balance assertion](#cw_orch_disable_wallet_balance_assertion).
- Deployment checks. When set to `true`, if no deployment file is detected when deploying a structure using the `Deploy::multi_deploy` function, it will deploy to all provided chains without asking for approval.

## Logging

### RUST_LOG

Optional, accepted values: `debug`, `error`, `info`, `warn`,  `trace`

 `RUST_LOG` defines the Log level for the application. This is actually a `Rust` flag that we use inside `cw-orch`. If working with this environment variable, don't forget to also initialize a logger at the beginning of you application to be able to see the output. You can work with <a href="https://crates.io/crates/pretty_env_logger/" target="_blank">pretty_env_logger</a> for instance.

### CW_ORCH_SERIALIZE_JSON

Optional, accepted values: `false`, `true`

If equals to `true`, in the output logs, cw-orch will serialize the contract messages (instantiate, execute, query,... ) as JSON. This replaces the standard Rust Debug formatting and allows for easy copying and sharing of the executed messages.

### CW_ORCH_DISABLE_LOGS_ACTIVATION_MESSAGE

Optional, accepted values: `false`, `true`

By default if the logs are not enabled, `cw-orch` wil print a warning message to invite users to activate the logging capabilities of cw-orch. if equals to `true`, this env variable disables the warning message.
