# Quick-Start Guide

This guide will show you how to use the `cw-orchestrator` with your smart contract. Follow the steps below to add `cw-orch` to your contract's TOML file, enable the interface feature, add the interface macro to your contract's endpoints, and use interaction helpers to simplify contract calls and queries.

## Adding `cw-orch` to Your `Cargo.toml` File

To use the `cw-orchestrator`, you need to add `cw-orch` to your contract's TOML file. Run the command below in your contract's directory:

```shell
$ cargo add --optional cw-orch
> Adding cw-orch v0.13.3 to optional dependencies.
```

Alternatively, you can add it manually in your `Cargo.toml` file as shown below:

```toml
[dependencies]
cw-orch = {version = "0.13.3", optional = true } # Latest version at time of writing
```

Now that we have added `cw-orch` as an optional dependency we will want to enable it through a feature. This ensures that the code added by `cw-orch` is not included in the wasm artifact of the contract. To do this add an `interface` feature to the `Cargo.toml` and enable `cw-orch` when it is enabled.

To do this include the following in the `Cargo.toml`:

```toml
[features]
interface = ["dep:cw-orch"] # Adds the dependency when the feature is enabled
```

## Creating an Interface

Now that we have the dependency set up you can add the `interface_entry_point` macro to your contract's entry points. This macro will generate an interface to your contract that you will be able to use to interact with your contract. Get started by adding the feature-flagged interface macro to the contract's entry points:

```rust,no_run,noplayground
# use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
# pub struct InstantiateMsg;
# pub struct ExecuteMsg;
#
// In `contract.rs`
#[cfg_attr(feature="interface", cw_orch::interface_entry_point)] // <--- Add this line
pub fn instantiate(
   deps: DepsMut,
   env: Env,
   info: MessageInfo,
   msg: InstantiateMsg,
) -> StdResult<Response> {
    // ...
    Ok(Response::new())
}

#[cfg_attr(feature="interface", cw_orch::interface_entry_point)] // <--- Add this line
pub fn execute(
   deps: DepsMut,
   env: Env,
   info: MessageInfo,
   msg: ExecuteMsg,
) -> StdResult<Response> {
    // ...
    Ok(Response::new())
}
#
# fn main() {}
// ... Do the same for the other entry points (query, migrate, reply, sudo)
```

By adding these lines, we generate code whenever the `interface` feature is enabled. The code generates a contract interface, the name of which will be the PascalCase of the crate's name.

When uploading to a blockchain the marco will search for an `artifacts` directory in the project's root. If this is not what you want you can specify the paths yourself using the `interface` macro covered in [interfaces](./tutorial/interfaces.md#defining-contract-interfaces).

> The name of the crate is defined in the `Cargo.toml` file of your contract.

It can be helpful to re-expose the interface in the crate's root so that it is easy to import:

```rust,ignore
// in lib.rs
#[cfg(feature = "interface")]
pub use crate::contract::MyContract
```

You can now create a test in `contract/tests` or an executable in `contract/bin` and start interacting with the contract.

## Interaction helpers

cw-orchestrator provides an additional macro to simplify contract calls and queries. The macro generates functions on the interface for each variant of the contract's `ExecuteMsg` and `QueryMsg`.

Enabling this functionality is very straight-forward. Find your `ExecuteMsg` and `QueryMsg` definitions and add the `ExecuteFns` and `QueryFns` derive macros to them like below:

```rust,no_run
use cosmwasm_schema::{QueryResponses, cw_serde};

#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))]
#[cw_serde]
pub enum ExecuteMsg {
    Increment {},
    // ...
}

#[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))]
#[derive(QueryResponses)]
#[cw_serde]
pub enum QueryMsg {
    #[returns(String)]
    Config {}
    // ...
}

# fn main() {}
```

Any variant of the `ExecuteMsg` and `QueryMsg` that has a `#[derive(ExecuteFns)]` or `#[derive(QueryFns)]` will have a function implemented on the interface through a trait. The function will have the snake_case name of the variant and will take the same arguments as the variant. The arguments are ordered in alphabetical order to prevent attribute ordering from changing the function signature. If coins need to be sent along with the message you can add `#[payable]` to the variant and the function will take a `Vec<Coin>` as the last argument.

You can access these functions by importing the generated traits form the message file. The generated traits are named `ExecuteMsgFns` and `QueryMsgFns`. Again it's helpful to re-export these traits in the crate's root so that they are easy to import:

```rust,ignore
// in lib.rs
#[cfg(feature = "interface")]
pub use crate::msg::{ExecuteMsgFns as MyContractExecuteFns, QueryMsgFns as MyContractQueryFns};
```

## Example Counter Contract

To show all this functionality in action, we will use an example counter contract. The example counter contract is a simple contract that allows you to increment and decrement a counter. The contract also allows you to query the current value of the counter. The contract is available [here](https://github.com/AbstractSDK/cw-orchestrator/tree/main/contracts/counter).

We have already added the `interface_entry_point` macro to the contract's endpoints. We can now create a test in `contract/tests` to interact with the contract. The test will use the `Mock` struct from `cw-orchestrator` to mock the environment and the `CounterContract` struct generated by the `interface_entry_point` macro to interact with the contract.

```rust,ignore
{{#include ../../contracts/counter/tests/integration_tests.rs:integration_test}}
```
