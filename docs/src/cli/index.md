# Orchestrator Command Line Interface (CLI)

Currently, each chain has its own CLI based on wasmd, which are incompatible with each other. With this in mind, we created cw-orch-cli. cw-orchestrator allows for easy chain switching, which is essential for cross-chain solutions.

## Prerequisites

- Rust
- OpenSSL

## Setup

```bash
cargo install cw-orch-cli
```

## Features

Supported features of cw-orch-cli:

- **[Keys management](./keys.md)**
  - Add, show or remove key for the CLI

Feel free to request new features by [opening an issue](https://github.com/AbstractSDK/cw-orchestrator/issues/new)!
