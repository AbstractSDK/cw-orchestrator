# cw-orchestrator Changelog

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
