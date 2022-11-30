# Quickstart

In order to use BOOT you need access to the entry point message types (`InstantiateMsg`,`ExecuteMsg`,...) of the contracts you want to interact with.

If you want to perform on-chain transaction you also need access to the gRPC endpoint of a node. These are most-often available on port 9090.

> If you're unsure as to what the gRPC endpoint of your chain is, check the [Cosmos Directory](https://cosmos.directory) and there should be some listed.

The following sections detail setting up a library package for the contract interfaces and a binary package for executable scripts.˜

Following this example, the project's structure should eventually look like:

```path
.
├── Cargo.toml
├── my-contract
│   ├── Cargo.toml
│   └── src
│       ├── contract.rs (execute, instantiate, query, ...)
│       └── ..
├── packages
│   ├── my-project
│   │   └── my-contract.rs (msgs)
│   └── interfaces
│       └── my-contract.rs (interface)
└── scripts
    ├── Cargo.toml
    └── src
        └── bin
            ├── deploy.rs
            └── test_my_contract.rs
```

## Sections
- **[Interfaces](./interfaces.md)**
    * Define interfaces for your contracts.
- **[Environment File](./env-variable.md)**
    * Configure your mnemonics and log settings.
- **[Scripting](./scripting.md)**
    * Write runnable scripts with your interfaces.
- **[CI/CD](./ci-cd.md)**
	* Deploying your contracts in using continuous integration and deployment tools.
