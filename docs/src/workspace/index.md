# Workspace Tutorial

**WIP**

Following this example, the project's structure should look like:

```path
.
├── Cargo.toml
├── artifacts
│   ├── other_contract.wasm
│   └── my_contract.wasm
├── contracts
│   ├── my-contract
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── contract.rs (execute, instantiate, query, ...)
│   │       └── ..
│   └── other-contract
│       └── ..
├── packages
│   ├── my-project (messages)
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       ├── my-contract.rs
│   │       ├── other-contract.rs
│   │       └── ..
│   └── my-project-interface (interface collection)
│       ├── Cargo.toml
│       └── src
│           ├── lib.rs
│           ├── my-project.rs
│           └── ..
└── scripts (executables)
    ├── .env
    ├── Cargo.toml
    └── src
        ├── deploy.rs
        └── test_project.rs
```

<!-- ## Sections

- **[Interfaces](./interfaces.md)**
  - Define interfaces for your contracts.
- **[Scripting](./scripting.md)**
  - Write runnable scripts with your interfaces.
- **[Integration](./integration.md)**
  - Export a deployment of your application for use in integration testing. -->
