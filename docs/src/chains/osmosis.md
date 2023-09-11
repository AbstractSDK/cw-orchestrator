# Osmosis

Osmosis is a cutting-edge decentralized exchange built on the Cosmos network, designed for seamless asset swaps across various interconnected blockchains. As a trailblazer in AMM protocols, Osmosis empowers users with fast, secure, and efficient cross-chain liquidity solutions. The platform's innovative approach to DeFi positions it as a cornerstone in the Cosmos ecosystem, enabling anyone to effortlessly tap into the vast potential of inter-chain finance.

[Visit Osmosis's Website](https://osmosis.zone/)

## Usage

See how to setup your main function in the [main function](../single_contract/scripting.md#main-function) section. Update the network passed into the `Daemon` builder to be `networks::OSMO_5`.

```rust,ignore
{{#include ../../../cw-orch/src/daemon/networks/osmosis.rs:osmosis}}
```

## References

- [Osmosis Documentation](https://docs.osmosis.zone/)
- [Osmosis Discord](https://discord.com/invite/osmosis)
