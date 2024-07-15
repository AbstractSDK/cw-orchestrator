# Cw-Orchestrator

<div align="center">
  <img src="https://raw.githubusercontent.com/AbstractSDK/assets/mainline/v1/orchestrator_bg2.png", width = "230px"/>  
  
<a href="https://docs.rs/cw-orch/latest" ><img alt="docs.rs" src="https://img.shields.io/docsrs/cw-orch"></a> <a href="https://crates.io/crates/cw-orch" ><img alt="Crates.io" src="https://img.shields.io/crates/d/cw-orch"></a> <a href="https://app.codecov.io/gh/AbstractSDK/cw-orchestrator" ><img alt="Codecov" src="https://img.shields.io/codecov/c/github/AbstractSDK/cw-orchestrator?token=CZZH6DJMRY"></a>

</div>

cw-orchestrator is an advanced testing and deployment tool for CosmWasm smart-contracts. It's designed to make it easy to test and deploy contracts in a variety of environments including cw-multi-test, local, testnet, and mainnet. It does this by providing the ability to write environment-generic code that interacts with CosmWasm contracts. In doing so it removes the need to maintain deployment code for multiple environments. In short, cw-orchestrator is the go-to tool for testing and deploying CosmWasm contracts.

## Features

Below we briefly outline the key features that cw-orchestrator has to offer.

### Testing

cw-orchestrator provides a testing framework that makes it easy to write tests for CosmWasm contracts. It does this by providing a testing environment that mimics the behavior of a CosmWasm blockchain. This allows you to write tests that interact with your contract in the same way that it would be interacted with on a real blockchain.  

This framework allow developers to easily test contract-to-contract interactions without having to deal with the overhead of running a node locally. The testing framework also provides a number of utilities simplify the syntax for write tests. These utilities include the ability to easily set and query balances, set block height/time and more.

Additionally developers can share their infrastructure with other developers by creating a wrapper around their project's deployment logic, allowing others to easily test how their contracts interact with the project.

The testing frameworks supported by cw-orchestrator includes:

- **[Cw-Multi-Test](./integrations/cw-multi-test.md)**
- **[Starship](./interchain/integrations/daemon.md#for-testing)**
- **[Osmosis-test-tube](./integrations/osmosis-test-tube.md)**

### Deployment + Scripting

cw-orchestrator also provides the ability to deploy to real networks. It does this by providing an easy to use interface to a blockchain node that can be used to submit transactions, query state and inspect transaction results. Any blockchain transaction can be broadcasted using cw-orchestrator.

### Interface Generation

Interacting with a smart-contract is often verbose, leading to a lot of boilerplate code. cw-orchestrator solves this problem by providing a macro that generates an interface to a contract. This interface can be used to easily interact with a contract and improves the readability of your tests and deployments. Making it easier to write and maintain tests and deployments. Additionally, because this library is written in Rust, any breaking changes to your contract's interface will cause a compile-time error, making it easy to keep your tests and deployments up to date.

## Getting Started

These docs contain a [quick-start](./quick_start.md) and a longer [tutorial-style walkthrough](./contracts/index.md).
