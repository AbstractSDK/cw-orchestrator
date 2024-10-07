# Cosmos Hub

The Cosmos Hub is the first of thousands of interconnected blockchains that will eventually comprise the Cosmos Network. The primary token of the Cosmos Hub is the ATOM, but the Hub will support many tokens in the future.

[Cosmos Hub Website](https://cosmos.network/)

```rust,ignore
{{#include ../../../packages/cw-orch-networks/src/networks/cosmos.rs:cosmos}}
```

## Usage

See how to setup your main function in the [main function](../contracts/scripting.md#main-function) section. Update the network passed into the `Daemon` builder to be `networks::COSMOS_TESTNET`.
## References

- [Cosmos Hub Documentation](https://hub.cosmos.network/main)
- [Cosmos Hub Discord](https://discord.gg/interchain)
