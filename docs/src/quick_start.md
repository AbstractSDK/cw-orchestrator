# Quick-Start Guide

This guide will show you how to use the `cw-orchestrator` with your smart contract. Follow the steps below to add `cw-orch` to your contract's TOML file, enable the interface feature, add the interface macro to your contract's endpoints, and use interaction helpers to simplify contract calls and queries.

## Adding `cw-orch` to Your `Cargo.toml` File

To use the `cw-orchestrator`, you need to add `cw-orch` to your contract's TOML file. Run the command below in your contract's directory:

```shell
$ cargo add --optional cw-orch
> Adding cw-orch v0.16.0 to optional dependencies.
```

When using workspaces, you need to add `cw-orch` in the crate where the messages are defined. For instance for the cw20-base contract on the [cw-plus repository](https://github.com/CosmWasm/cw-plus/): 
```shell
$ cargo add cw-orch --optional --package cw20-base
> Adding cw-orch v0.16.0 to optional dependencies.
$ cargo add cw-orch --optional --package cw20
> Adding cw-orch v0.16.0 to optional dependencies.
```

Alternatively, you can add it manually in your `Cargo.toml` file as shown below:

```toml
[dependencies]
cw-orch = {version = "0.16.0", optional = true } # Latest version at time of writing
```

> **NOTE**: In the rest of this guide we will assume your project is a rust workspace. We also provide an example setup on the [cw-plus](https://github.com/AbstractSDK/cw-plus/tree/main) repository for you to follow along or get help if you're lost. 

Now that we have added `cw-orch` as an optional dependency we will want to enable it through a feature. This ensures that the code added by `cw-orch` is not included in the wasm artifact of the contract. To do this add an `interface` feature to the `Cargo.toml` and enable `cw-orch` when it is enabled.

To do this include the following in the `Cargo.toml` of the packages where you included `cw-orch` as an optional dependency:

```toml
[features]
interface = ["dep:cw-orch"] # Adds the dependency when the feature is enabled
```

> **NOTE**: If you are using `rust-analyzer`, you can add the following two lines in your `settings.json` to make sure the features get taken into account when checking the project : 
>
>    ```json 
>     "rust-analyzer.cargo.features": "all",
>     "rust-analyzer.check.features": "all",
>    ```

## Creating an Interface

When using workspaces, we advise you to create a new crate inside your workspace for defining your contract interfaces. In order to do that, use : 
```shell
cargo new interface --lib
cargo add cw-orch --package interface 
```

And add the interface package to your workspace `Cargo.toml` file
```toml
[workspace]
members = ["packages/*", "contracts/*", "interface"]
```

In the interface package, you can define all the interfaces to your contracts. For instance, for the `cw20-base` contract : 
```rust
use cw_orch::interface;
use cw_orch::prelude::*;
use cw20_base::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg};

/// Creates a `Cw20Base` struct representing a contract and link it to the different messages that can be sent to it
#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Cw20Base;

/// Allow your structure to be uploaded (on a test or an actual environment)
impl<Chain: CwEnv> Uploadable for Cw20Base<Chain> {
    // Use for your struct to be able to access the associated wasm file. This is used for scripting mostly. 
    // You don't need to implement this function when testing with the `Mock` environment
    fn wasm(&self) -> WasmPath {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let wasm_path = format!("{}/../artifacts/{}", crate_path, "cw20_base.wasm");

        WasmPath::new(wasm_path).unwrap()
    }
    // This needs to be implemented ONLY if you are testing with the `Mock` environment
    fn wrapper(&self) -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                cw20_base::contract::execute,
                cw20_base::contract::instantiate,
                cw20_base::contract::query,
            )
            .with_migrate(cw20_base::contract::migrate),
        )
    }
}

```
When importing your crates to get the messages types, you can use the following command in the interface folder. Don't forget to activate the interface feature to be able to use the cw_orch functionalities. 

```shell
cargo add cw20-base --path ../contracts/cw20-base/ --features=interface
cargo add cw20 --path ../packages/cw20 --features=interface
```

## Interaction helpers

cw-orchestrator provides an additional macro to simplify contract calls and queries. The macro generates functions on the interface for each variant of the contract's `ExecuteMsg` and `QueryMsg`.

Enabling this functionality is very straightforward. Find your `ExecuteMsg` and `QueryMsg` definitions and add the `ExecuteFns` and `QueryFns` derive macros to them like below. In our example, you have to define it for the `QueryMsg` in the `cw20-base` crate and for the `ExecuteMsg` in the `cw20` crate.

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
    /// Returns the current balance of the given address, 0 if unset.
    #[returns(cw20::BalanceResponse)]
    Balance { address: String },
    // ...
}

# fn main() {}
```

Any variant of the `ExecuteMsg` and `QueryMsg` that has a `#[derive(ExecuteFns)]` or `#[derive(QueryFns)]` will have a function implemented on the interface (e.g. `Cw20Base`) through a trait. The function will have the snake_case name of the variant and will take the same arguments as the variant. The arguments are ordered in alphabetical order to prevent attribute ordering from changing the function signature. If coins need to be sent along with the message you can add `#[payable]` to the variant and the function will take a `Vec<Coin>` as the last argument.

You can access these functions by importing the generated traits from the message file. The generated traits are named `ExecuteMsgFns` and `QueryMsgFns`. Again it's helpful to re-export these traits in the crate's root so that they are easy to import:

```rust,ignore
// in lib.rs
#[cfg(feature = "interface")]
pub use crate::msg::{ExecuteMsgFns as MyContractExecuteFns, QueryMsgFns as MyContractQueryFns};
```

## Example Cw20Base Contract interaction

To show all this functionality in action, we will use an example counter contract. The example counter contract is a simple contract that allows you to increment and decrement a counter. The contract also allows you to query the current value of the counter. The contract is available [here](https://github.com/AbstractSDK/cw-orchestrator/tree/main/contracts/counter).

We have already added the `interface_entry_point` macro to the contract's endpoints. We can now create a test in `contract/tests` to interact with the contract. The test will use the `Mock` struct from `cw-orchestrator` to mock the environment and the `CounterContract` struct generated by the `interface_entry_point` macro to interact with the contract.

```rust,ignore
{{#include ../../contracts/counter/tests/integration_tests.rs:integration_test}}
```
