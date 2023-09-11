# Cw-Orchestrator

<div align="center">
  <img src="https://raw.githubusercontent.com/AbstractSDK/assets/mainline/orchestrator_bg2.png", width = "230px"/>  
  
<a href="https://docs.rs/cw-orch/latest" ><img alt="docs.rs" src="https://img.shields.io/docsrs/cw-orch"></a> <a href="https://crates.io/crates/cw-orch" ><img alt="Crates.io" src="https://img.shields.io/crates/d/cw-orch"></a> <a href="https://app.codecov.io/gh/AbstractSDK/cw-orchestrator" ><img alt="Codecov" src="https://img.shields.io/codecov/c/github/AbstractSDK/cw-orchestrator?token=CZZH6DJMRY"></a>

</div>

cw-orchestrator is an advanced testing and deployment tool for CosmWasm smart-contracts. It's designed to make it easy to test and deploy contracts in a variety of environments including cw-multi-test, local, testnet, and mainnet. It does this by providing the ability to write environment-generic code that interacts with CosmWasm contracts and by doing so removing the need to keep maintain deployment code for multiple environments. In short, cw-orchestrator should be your go-to tool for testing and deploying CosmWasm contracts.

## Features

- **Testing**: cw-orchestrator provides a testing framework that makes it easy to write tests for CosmWasm contracts. It does this by providing a testing environment that mimics the behavior of a CosmWasm blockchain. This allows you to write tests that interact with your contract in the same way that it would be interacted with on a real blockchain. These kinds of tests allow developers to more easily test contract-to-contract interactions without having to deal with the overhead of running a local node. The testing framework also provides a number of utilities that make it easy to write tests for CosmWasm contracts. These utilities include the ability to easily set and query balances, set block height/time and more. Additionally by creating a wrapper interface around a project's deployment developers can share their testing infrastructure with other developers, allowing them to easily test how their contracts interact with the project. The testing framework supported by cw-orchestrator include : 
  - **[Cw-Multi-Test](./integrations/cw-multi-test.md)**
  - **[Starship](./integrations/starship.md)**
  - **[Osmosis-test-tube](./integrations/osmosis-test-tube.md)**

- **Deployment**: cw-orchestrator also provides the ability to deploy to real networks. It does this by providing an easy to use interface to a blockchain node that can be used to submit transactions, query state and inspect transaction results.

- **Interface Generation**: Interacting with a smart-contract is often verbose, leading to a lot of boilerplate code. cw-orchestrator solves this problem by providing a macro that generates an interface to a contract. This interface can be used to easily interact with a contract and improves the readability of your tests and deployments. Making it easier to write and maintain tests and deployments. Additionally, because this library is written in Rust, any breaking changes to your contract's interface will cause a compile-time error, making it easy to keep your tests and deployments up to date.

## Getting Started

These docs contain a quick-start and a longer tutorial-style walkthrough.
