# Migaloo

Migaloo is a permission-less, open source network for decentralized interoperable applications running the latest tech.

[Visit Migaloo's Docs](https://docs.migaloo.zone/)

## Usage

See how to setup your main function in the [main function](../single_contract/scripting.md#main-function) section. Update the network passed into the `Daemon` builder to be `networks::MIGALOO_1`.

```rust,ignore
{{#include ../../../cw-orch/src/daemon/networks/migaloo.rs:migaloo}}
```

## References

- [Migaloo Documentation](https://docs.migaloo.zone/intro)
- [White Whale Discord](https://discord.gg/pc5EXCBfEwa)
