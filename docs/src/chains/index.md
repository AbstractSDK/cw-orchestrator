# Supported Chains

cw-orchestrator currently explicitly supports the chains in this section:

- [Archway](./archway.md)
- [Injective](./injective.md)
- [Juno](./juno.md)
- [Kujira](./kujira.md)
- [Landslide](./landslide.md)
- [Migaloo](./migaloo.md)
- [Neutron](./neutron.md)
- [Nibiru](./nibiru.md)
- [Osmosis](./osmosis.md)
- [Rollkit](./rollkit.md)
- [Sei](./sei.md)
- [Terra](./terra.md)
- [Union](./union.md)
- [Xion](./xion.md)

## Support a new Cosmos Chain

Almost all Cosmos Chains can be added to cw-orchestrator. Depending on the on-chain modules (CosmWasm is not available on all chains for instance), action may be forbidden for some chains. However, connection should work out of the box for most of the chains. In order to add a new chain to your project, you can use the following objects:

```rust,ignore
{{#include ../../../cw-orch/tests/new_chain.rs:NEW_NETWORK_INFO}}
```

This chain info can then be used inside your project just like any other chain defined inside cw-orch.

Alternatively, we suggest using the <a href="https://docs.rs/cw-orch-daemon/latest/cw_orch_daemon/sync/struct.DaemonBuilder.html#method.grpc_url" target="blank">grpc_url</a> and <a href="https://docs.rs/cw-orch-daemon/latest/cw_orch_daemon/sync/struct.DaemonBuilder.html#method.gas" target="blank">gas</a> methods on the DaemonBuilder for quick and dirty fixes to the grpc url and the gas prices if needed.

If you would like to add explicit support for another chain, please feel free to [open a PR](https://github.com/AbstractSDK/cw-orchestrator/compare)!

## Config Overwrite

If you're running into issues with a dead gRPC URL, wrong gas price or similar issues then you can use a config file to overwrite those variables. To do so, make a new config file in `~/.cw-orchestrator/networks.toml`. 

```bash
touch ~/.cw-orchestrator/networks.toml
```

Then you can add the following optional content to the file, replacing the values with the ones you need. The chain-id is used to identify the chain you want to overwrite the values for.

```toml
{{#include ../../../networks.toml.example}}
```

So in the example above the configuration is applied for a `Daemon` built with the `JUNO_1` network.

## Issues

Each of the gRPC endpoints has been battle-tested for deployments. If you find any issues, please [open an issue](https://github.com/AbstractSDK/cw-orchestrator/issues/new)!
