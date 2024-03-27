# Rollkit

Rollkit is the open modular framework for sovereign rollups. Their mission is to empower developers to quickly innovate and create entire new classes of rollups with minimal tradeoffs.

[Visit Rollkit's Website](https://rollkit.dev/)

```rust,ignore
{{#include ../../../packages/cw-orch-networks/src/networks/rollkit.rs:rollkit}}
```

## Usage

See how to setup your main function in the [main function](../contracts/scripting.md#main-function) section. Update the network passed into the `Daemon` builder to be `networks::ROLLKIT_1`.

## References

- [Rollkit Documentation](https://rollkit.dev/learn/intro)
- [Rollkit Setup with CosmWasm](https://rollkit.dev/tutorials/cosmwasm)
