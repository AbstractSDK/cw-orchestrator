# CW Multi Test

Cw Multi Test is a rust-based test framework that allows developers to test  for multi-contract interactions without having to dispatch messages, storage and other variables themselves. With cw-orchestrator, most of the `cw-multi-test` logic is abstracted away, but you can still [learn more about the framework here](https://github.com/CosmWasm/cw-multi-test).

## Quick Start

The cw-multi-test integration comes at no extra cost for the developer. Creating a test environement in cw-orchestrator that leverages cw-multi-test goes along the lines of : 

```rust,ignore
    use cw_orch::prelude::*;
    use cosmwasm_std::Addr;
{{#include ../../../cw-orch/examples/mock_test.rs:mock_creation}}
```

> **_NOTE:_** When using `cw-multi-test`, the addresses ARE NOT validated like on a live chain. Therefore, you can use any string format for designating addresses. For instance,`Addr::unchecked("my-first-sender")` is a valid cw-multi-test address.

> **_NOTE:_** When using cw-multi-test, NO gas fees are charged to the sender address.

## Interacting with contracts

You can then use the resulting `Mock` variable to interact with your [contracts](../single_contract/index.md): 

```rust,ignore
{{#include ../../../cw-orch/examples/mock_test.rs:mock_usage}}
```

When executing contracts in a `cw-multi-test` environment, the messages and sub-messages sent along the Response of an endpoint, will be executed as well.
This environment mocks the actual on-chain execution exactly 

> This environment uses the actual functions of your contract **without** having to compile them into WASM. When you are calling `upload` with this environment, no wasm files are included in the test environment. This allows for better debugging of your contract code.

> If you are using the customizable Interface Macro, you will need to have implemented the `wrapper` function for interacting the the `Mock` environment. This function wil allow you to "connect" your contract endpoints to your `Contract` struct [See the dedicated page for more details](../single_contract/interfaces.md#customizable-interface-macro).


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