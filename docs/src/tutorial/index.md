# Quickstart

In order to use cw-orchestrator you need access to the entry point message types (`InstantiateMsg`,`ExecuteMsg`,...) of the contracts you want to interact with.

If you want to perform on-chain transaction you also need access to the gRPC endpoint of a node. These are most-often available on port 9090.

> If you're unsure as to what the gRPC endpoint of your chain is, check the [Cosmos Directory](https://cosmos.directory) and there should be some listed. cw-orchestrator ships with a set of urls that should contain at least one working endpoint. Feel free to add urls to the list and PR them!

The following sections detail setting up a library package for the contract interfaces and a binary package for executable scripts.

Following this example, the project's structure should eventually look like:

```path
.
├── Cargo.toml
├── artifacts
│   └── my_contract.wasm (binary file)
├── my-contract
│   ├── Cargo.toml
│   └── src
│       ├── contract.rs (execute, instantiate, query, ...)
│       └── ..
├── packages
│   ├── my-project
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs
│   │       └── my-contract.rs (msgs)
│   └── interfaces
│       ├── Cargo.toml
│       └── src
│            └── lib.rs
│            └── my-contract.rs (interface_entry_point)
└── scripts
    ├── .env
    ├── Cargo.toml
    └── src
        ├── deploy.rs
        └── test_my_contract.rs
```

## Sections

- **[Interfaces](./interfaces.md)**
  - Define interfaces for your contracts.
- **[Environment File](./env-variable.md)**
  - Configure your mnemonics and log settings.
- **[Scripting](./scripting.md)**
  - Write runnable scripts with your interfaces.
- **[Integration](./integration.md)**
  - Export a deployment of your application for use in integration testing.
