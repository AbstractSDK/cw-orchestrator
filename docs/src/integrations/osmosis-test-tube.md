# Osmosis Test Tube

Osmosis Test Tube is a rust-based test framework that allows developers to test for multi-contract interactions without having to dispatch messages, storage and other variables themselves. This environment is even close to the actual on-chain mechanisms than [Cw Multi Test](./cw-multi-test.md), because it runs tests directly on the actual chain binaries. With cw-orchestrator, most of the `osmosis-test-tube` logic is abstracted away, but you can still [learn more about the framework here](https://docs.rs/crate/osmosis-test-tube/latest).

## Prerequesites

In order to use `osmosis-test-tube`, the library needs to be able to compile and install the chain binaries. For that you will need to install a go compiler as well as the clang library. Run the following commands to install both of those libraries.

### Ubuntu
1. Install [go](https://go.dev/doc/install)
2. Install the clang library
```bash
    sudo apt install clang
``` 

### Arch Linux
1. Install go
```bash
    sudo pacman -Sy go
```
2. Instal the clang library
```bash
    sudo pacman -Sy clang
```

## Quick Start

Creating a test environement in cw-orchestrator that leverages cw-multi-test goes along the lines of : 

```rust,ignore
    use cw_orch::prelude::*;
    use cosmwasm_std::coins;
{{#include ../../../cw-orch/examples/osmosis_test_tube.rs:osmosis_test_tube_creation}}
```

This snippet will create a new address, provide it with initial balances and create the `osmosis-test-tube` environment.
The addresses are not handled like in the cw-multi-test environment and can't be decided upon manually. You will learn more later about [handling addresses in the OsmosisTestTube environement](#additional-customization). 

> **_NOTE:_** When using `osmosis-test-tube`, the addresses **are** validated like on a live chain.

> **_NOTE:_** When using `osmosis-test-tube`, gas fees are charged to the sender address. The gas fees don't represent the actual gas fees you will occur when interacting with the actual chain. That's why in the test snippet above, the amount of `uosmo` instantiated with the account is very high. 

## Interacting with contracts

You can then use the resulting `OsmosisTestTube` variable to interact with your [contracts](../single_contract/index.md): 

```rust,ignore
{{#include ../../../cw-orch/examples/osmosis_test_tube.rs:osmosis_test_tube_usage}}
```

When executing contracts in an `osmosis_test_tube` environment, the messages and sub-messages sent along the Response of an endpoint, will be executed as well. This environment mimics the actual on-chain execution by dispatching the messages inside the actual chain binaries.

> If you are using the customizable Interface Macro, you will need to have implemented the `wasm` function for interacting the the `Mock` environment. This function wil allow you to "connect" your contract endpoints to your `Contract` struct [See the dedicated page for more details](../single_contract/interfaces.md#customizable-interface-macro).


> **_NOTE:_** Keep in mind that cw-multi-test is based solely in rust and that a lot of actual blockchain modules are not mocked in the environment. The main cosmos modules are there (Bank, Staking), but some very useful ones (tokenfactory, ibc) as well as Stargate messages are not supported by the environment.

## Cloning

When cloning a cw_multi_test environment, you are not cloning the entire environment, but instead you are creating a new `Mock` typed variable with the same underlying `cw_multi_test::App` object reference. This is useful for objects that require to pass the chain as an object rather than by reference.
The underlying `cw_multi_test::App` object is however not clonable.

## Additional tools

The `Mock` test environment allows you to change application variables (such as the balance of an account) using wrappers around the underlying `cw_multi_test::App` object. Here are some examples of those wrappers in context : 


```rust,ignore
{{#include ../../../cw-orch/examples/mock_test.rs:mock_customization}}
```

## Additional customization

As we don't provide wrappers around each and every functionality that cw-multi-test provides, you can also customize the underyling `cw_multi_test::App`object to your specific needs. In the following example, we create a new validator in the test environment : 

```rust,ignore
{{#include ../../../cw-orch/examples/mock_test.rs:deep_mock_customization}}
````