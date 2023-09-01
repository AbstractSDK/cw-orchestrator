# cw-orchestrator Changelog

## Unpublished

- Add ChannelAccess trait to be able to access the underyling gRPC tonic channel of the daemon 
- Added ibc denom query (for completeness and working with ibc token transfers)
- Added transaction by event query (mainly for querying packets in and out of the chain)

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
