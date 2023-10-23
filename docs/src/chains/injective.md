# Injective

Injective is a unique blockchain tailored for finance, offering out-of-the-box modules like a fully decentralized orderbook. As an open smart contracts platform, it hosts a suite of decentralized apps designed for optimal user experience. Dive into Injective and unlock efficient capital allocation in decentralized financial markets.

[Visit Injective's Website](https://injective.com/)

```rust,ignore
{{#include ../../../packages/cw-orch-networks/src/networks/injective.rs:injective}}
```

## Usage

To interact with contracts on Injective, first enable the `eth` feature for cw-orchestrator. Injective supports EVM-based addresses, and this will enable their use within cw-orchestrator.

See how to setup your main function in the [main function](../contracts/scripting.md#main-function) section. Update the network passed into the `Daemon` builder to be `networks::INJECTIVE_1`.

## References

- [Injective Documentation](https://docs.injective.network/)
- [Injective Discord](https://discord.gg/injective)
