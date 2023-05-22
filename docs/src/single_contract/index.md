# Tutorial

This tutorial will guide you through setting up a single contract for use with cw-orchestrator. By the end of this tutorial you should be able to:

- Write deployment scripts for your contract.
- Write integration tests for your contract.
- Write executables for interacting with your contract.

In order to ensure that the code snippets shown here are correct we'll be using the counter contract provided in the repository as the source for our code-snippets. You can find the contract [here](https://github.com/AbstractSDK/cw-orchestrator/tree/main/contracts/counter).

> If you're working within a cargo workspace environment you can follow along and read the [Workspace](../workspace/index.md) docs after this tutorial.

## Prerequisites

- Contract Entry Point Messages: In order to use cw-orchestrator you need access to the entry point message types (`InstantiateMsg`,`ExecuteMsg`,...) of the contracts you want to interact with. Having them locally will enable you to generate helper functions for interacting with the contracts.

- A gRPC endpoint (optional): If you want to perform on-chain transaction you will need access to the gRPC endpoint of a node. These are most-often available on port 9090. Look in the documentation of the chain you want to connect to if our defaults aren't sufficient, or use the [Cosmos Directory](https://cosmos.directory) implementation to query one.

- A desire to learn: This tutorial will cover the basics of using cw-orchestrator but it won't cover everything. If you want to learn more about the features of cw-orchestrator you can check out the [API Documentation](https://docs.rs/cw-orch/latest/cw_orch/).

The following sections detail setting up a contract, tests for the contract, and scripts for interacting with the contract on a blockchain network.

Following this example, the directory structure should eventually look like:

```path
.
├── Cargo.toml
├── artifacts
│   └── counter.wasm (binary file)
└── counter
    ├── Cargo.toml
    ├── bin
    │   └── deploy.rs
    └── src
        ├── contract.rs (execute, instantiate, query, ...)
        └── msg.rs
        └── ..

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
