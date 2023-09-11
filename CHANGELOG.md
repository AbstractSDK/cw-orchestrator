# cw-orchestrator Changelog

## Unpublished

- Ability to use the ExecuteFns and QueryFns traits on Units and Unnamed enum variants by @Kayanski
- Added ibc denom query (for completeness and working with ibc token transfers)
- Added ibc connection-end query (for completeness and working with ibc connections)
- Added transaction by event query (mainly for querying packets in and out of the chain)
- Added helper to modify the chain id of cw-multi-test (Mock)
- Added a trait to be able to commit any transaction to chain (Protobuf any type)
- Added min gas and average gas utilization for computing the tx fee

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
