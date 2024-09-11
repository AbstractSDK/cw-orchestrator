# cw-orchestrator Changelog

## Unpublished

 - Add methods to set the private key and mnemonic of an existing sender
 - Deprecate `authz_granter` and `fee_granter` on `Daemon` struct
  
### Breaking

## 0.25.0

- `is_test` Added for Daemon Builders, when set to `true` will use temporary file for state
- Chain configs now can be edited from a networks config file. It will read `~/.cw-orchestrator/networks.json`, see example `networks.json.example`
- New package `cw-orch-neutron-test-tube`
- Chain configs now can be edited from a networks config file. It will read `~/.cw-orchestrator/networks.toml`, see example `networks.toml.example`
- Added `load_network` for `DaemonBuilder`, defaults to `true`. Set to `false` to avoid loading networks from `~/.cw-orchestrator/networks.toml`
- New environment variable for `cw-orch-starship`: `CW_ORCH_STARSHIP_CONFIG_PATH` to specify path of starship config that's currently in use.
- 3+ chain support for `cw-orch-starship`.

### Breaking

- Daemon state got flattened from `.chain_name.chain_id` to `.chain_id` to simplify state parsing.
- Rename `DaemonInterchainEnv` to `DaemonInterchain` for consistency.

## 0.24.1

- Added async query functions generations with cw_orch::QueryFns
- Re-export ibc-relayer-types inside cw-orch-interchain for ease of use
- Deprecate cw-orch-core `TxHandler::sender` in favor of `TxHandler::sender_addr`
- Implement `SenderBuilder`, `QuerySender` and `TxSender` which allow for customizing the transaction commitment logic.
- Can now easily build `QueryOnlyDaemon` which will only expose query functions.
- Changed cw-orch-interchain InterchainEnv API
  - `chain` --> `get_chain`
  - `follow_packet` --> `await_single_packet`
  - `wait_ibc` --> `await_packets`
  - `check_ibc` --> `await_and_check_packets`
  - `follow_packets_from_tx_hash` --> `await_packets_for_txhash`
- Better Docs for interchain, cw-orch and clone-testing
- Added max block time environment variable `CW_ORCH_MAX_BLOCK_TIME`
- `CW_ORCH_MIN_GAS` Now defaults to 150_000 instead of 0, making it more reliable for txs that cost little gas

### Breaking

- Refactor `Daemon` builder pattern to allow for custom senders.
- Update `Daemon` / `DaemonAsync` implementations to reflect customizable sender.
- Deprecated `CW_ORCH_MIN_BLOCK_SPEED` in favor of `CW_ORCH_MIN_BLOCK_TIME`

## cw-orch-daemon 0.23.5

- Fixed Get Tx By Events compatibility with Cosmos SDK 0.50+ for Daemon
- Fix Generics on QueryMsg and Return types

## 0.23.0

- Added a test to make sure the derive macros stay compatible with new cw-orch versions
- Changed the derive macros import from cw_orch to cw_orch_core. This allows changing the cw-orch API without breaking the derive macros.
- Cw-orch mock env info doesn't error when using chain ids that don't match the `osmosis-1` pattern
- Add interchain capabilites as well as clone-testing
- Bumped MSRV to 1.73 because of dependency `cosmwasm-vm@1.5.5`
- Remove `impl_into`, the old `impl_into` behavior is now the default behavior
- EXCITING FEATURE : Added an item and a map query method to be able to query cw-storage-plus structure outside of contracts easily
- Add `flush_state` method for Local Chain Daemons
- cw-orch-interchain now errors on checking transactions for IBC packets if NO packets were found
- `DaemonState` removed from `Sender`
- `Channel` moved from `DaemonState` to `Sender`
- `DaemonState` write-locks file unless it's read-only, meaning it will panic at initialization if other process holds lock of it
- `DaemonState` now can be safely used between threads and processes
- Two non-related Daemon's can't use same file for writing simultaneously (cloned or rebuilt are related Daemon)
- Writing to a file happens when all Daemon's that use same file dropped instead of hot writes
- `force_write` added to the `DaemonState` to allow force write of the state
- Added `event_attr_values` to get all the attribute values corresponding to a key
- Added `remove_{address,code_id}` functions to be able to erase an entry in state. Involves core, mock, daemon, osmosis-test-tube, clone-testing
- Added `state` to DaemonBuilder to be able to share state between daemons
- Added `write_on_change` flag for writing to a `DaemonState` file on every change

### Breaking

- Daemon : Changed return types on daemon queriers to match CosmWasm std types
- Daemon: Added below second block time.
- Cw-orch : Separate osmosis test tube from cw-orch. Its not available in its own crate `cw-orch-osmosis-test-tube`
- Simplify the generated macros to allow for `impl Into<Type>` on `Uint*` and `String` types.
- Fns Derive Macros: Namespace the fns derive attributes with `cw-orch(<attribute>)`. For instance, `#[cw_orch(payable)]`.
- Clone-testing : Remove rt in Mock State creation (daemon doesn't need it anymore)

## 0.22.0

- Updated osmosis test tube to 24.0.1 ,that avoids re-compiling osmosis test tube
- Added `balance` query at the root of QueryHandler
- Added DaemonBuilder configuration for grpc url and fee overwriting
- Removed IBC chain registry from cw-orch-networks. Using the custom `ChainInfo` and `ChainInfoOwned` types
- Fixed broken documentation links
- Separate Env variables and define them in the crates where they are used
- Removed self from the methods inside Uploadable trait
- Current Status : Breaking

## 0.21.2

- Allow cw-orch wasm compilation without features
- Transaction Response now inspects logs and events to find matching events.

## 0.21.1

- Remove mandatory runtimes when building Daemon
- Allow cw-orch to compile for a wasm target without adding features
- Changed GRPC url for Local Terra

## 0.21.0

- Updated cw-multi-test to allow for IBC packet timeout

## 0.20.1

- Fix ARM path derivation for wasm
- Fix state file path creation on cw-orch-daemon
- Added addr_make_with_balance

## 0.20.0

- Changed behavior for default constructor implementation by macro "interface" --> Added possibility to have a fixed ID
- Added unified querier interface for all CwEnv
- Updated multiple dependencies
- Added authz, fee granter and hd_indec support for the `Daemon` object
- Remove Fee Granter env variable (now incorporated in code inside the `DaemonBuilder`)
- Replaces Rc by Arc in DaemonAsync for use in an async distributed environment
- Added instantiate2 support on CwEnv
- Removed secp dependency (gotten through the bitcoin dependency)
- Removed duplicates in disabled logs env
- Moved DeployDetails to its own trait : `EnvironmentQuerier`
- Modified gas prices for networks
- Fixed the artifacts resolution in case of a build postfix

## 0.19.1

- Fix: Min gas env variable processing.
- Fix: Added specific local hash computation possibility

## 0.19.0

- Add `MutCwEnv` for manipulating testing environments.
- Add `BankQuerier` as trait bound to `CwEnv`.
- Add `WasmCodeQuerier` as trait bound to `CwEnv`.
- Changed the snapshot feature for better snapshot readability.
- Added Readonly flag to the daemon state
- Added min gas fee environment variable
- Updated `cosmrs` to `0.15.0`
- Updated tonic to `0.10.2`
- Bumped MSRV to 1.72 because of dependency `cosmrs@0.15.0`

## 0.18.2

- Added Snapshot-Testing

## 0.18.1

- Fix : Added daemon flag on the networks import

## 0.18.0

- Added wallet balance assertions to avoid erroring when the wallet doesn't have enough balance when submitting a daemon transaction
- Added doravota network
- Corrected Osmosis-1 network
- Better handling of env variables --> Crate is becoming easier to maintain and document

## 0.17.0

- Added possibility to not panic in parse network by @Buckram123
- Added stargate feature to cw-multi-test
- Added `Deploy` to prelude
- Add ability to provide custom state in `Deploy::set_contracts_state`
- Breaking change: remove the `&self` dependency for the `Deploy::deployed_state_file_path` method
- Using `dirs` instead of `shellexpand` for getting the default cw-orch state dir.
- Exposed the state_dir location
- Added better env variable management by @Kayanski
- Added message to enable logging if not enabled
- Removed unused dependencies
- Added snapshot testing in mock env by @Kayanski (feature flagged)

## v0.16.4 [20th September 2023]

- Added automatically derived docs
- Added gzip support for smaller files sent on-chain
- Added faster error when broadcast returns a non-0 error code

## v0.16.3 [19th September 2023]

- Changed cw-multi-test to an in-house cw-multi-test version
- Fixed finding state file using cw-orch-daemon, the Home folder is now recognized from "~"

## v0.16.2 [12th September 2023]

- Fix finding state file using cw-orch-daemon

## v0.16.1 [12th September 2023]

- Fix dependencies on the traits::Stargate trait and on the osmosis-test-tube implementation
- Made the chain_id mandatory in `Mock::with_chain_id`

## v0.16.0 [11th September 2023]

- Ability to use the ExecuteFns and QueryFns traits on Units and Unnamed enum variants by @Kayanski
- Added Sei Network
- Make the broadcast function public on Sender
- Added a `call_as` helper on TxHandler Trait
- Added optional disable of sorting of ExecuteFns and QueryFns fields by @Kayanski
- Added automatic tx retrying in case of an account sequence error by @Kayanski
- Made transaction retrying more modular (for adding more retry cases) by Kayanski
- Added ibc denom query (for completeness and working with ibc token transfers)
- Added ibc connection-end query (for completeness and working with ibc connections)
- Added transaction by event query (mainly for querying packets in and out of the chain)
- Added helper to modify the chain id of cw-multi-test (Mock)
- Added a trait to be able to commit any transaction to chain (Protobuf any type)
- Added min gas and average gas utilization for computing the tx fee
- Added Install Readme
- Change the state file default location for relative paths `./` --> `~/.cw-orchestrator`
- Added env variables for customizing experience

## v0.15.0

- Add `add_balance` function on the `Mock` type.

## v0.10.0

- Update `CallAs` to be generic over environments.
- Use updated derive macros .
- Add `store_on` function on `Deploy` trait.

## v0.8.0

- Redo crate API exposure
- Remove prelude module

## v0.4.0

- Expose DaemonQuerier.
- Require Chain reference instead of owned type on Contract constructor.
