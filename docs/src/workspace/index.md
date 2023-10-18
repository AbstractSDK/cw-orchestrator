# Workspace Tutorial

In this tutorial, you will learn how to setup cw-orchestrator inside a workspace project. We present here the best practices for setting up an application composed of multiple contracts with the `cw-orch` crate. If you want a short version on how to integrate your contracts, we advise runing to our [Quick Start guide](../quick_start.md).

> **NOTE**: [A rust workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html) is a simple way to have multiple contracts inside the same repository and are especially suited for applications in which multiple contracts communicate with each other.

This tutorial has multiple components:

- [Project Setup](./setup.md)
  - This tutorial helps you setup your project to have a rational workspace structure and all the dependencies needed for interacting with your contracts.
- [Project Wrapper](./deploy.md)
  - This tutorial allows you to simplify the way you interact with all the contracts included in your project. It presents best practices and usual project structure to keep your project organized.
- [Collaborating](./collaboration.md)
  - This tutorial shows you how you can distribute your contract code, you deployment variables (code_ids, addresses...) for other projects to integrate with. This is the ultimate way to collaborate with other projects.

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
