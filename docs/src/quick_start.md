# Cw-Orchestrator Quick-Start Guide

This guide will show you how to use the `cw-orchestrator` with your smart contract. Follow the steps below to add `cw-orch` to your contract's TOML file, enable the interface feature, add the interface macro to your contract's endpoints, and use interaction helpers to simplify contract calls and queries.

## Adding `cw-orch` to Your Contract's TOML File

To use the `cw-orchestrator`, you need to add `cw-orch` to your contract's TOML file. Run the command below in your contract's directory:

```shell
$ cargo add --optional cw-orch
> Adding cw-orch v0.10.0 to optional dependencies.
```

Alternatively, you can add it manually in your `Cargo.toml` file as shown below:

```toml
[dependencies]
cw-orch = {version = "0.10.0", optional = true } # Latest version at time of writing
```

After adding `cw-orch` as an optional dependency, you should enable it through a feature. Doing so ensures that the code added by `cw-orch` is not included in the wasm artifact of the contract. You can do this by adding an `interface` feature to the `Cargo.toml` and enabling `cw-orch` when it is activated:

```toml
[features]
interface = ["dep:cw-orch"]
```

## Adding the Interface Macro to Your Contract's Endpoints

With the dependency set up, you can now add the `interface` macro to your contract's endpoints. This macro will generate an interface to your contract that you can use to interact with it. You can get started by adding the feature-flagged interface macro to the contract's endpoints as shown in the code snippet below:

```rust,no_run
# use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
# struct InstantiateMsg;
# struct ExecuteMsg;

// In `contract.rs`
#[cfg_attr(feature="interface", cw_orch::interface_entry_point)] // <--- Add this line
pub fn instantiate(
   deps: DepsMut,
   env: Env,
   info: MessageInfo,
   msg: InstantiateMsg,
 -> StdResult<Response> {
    // ...
    Ok(Response::new())
}

#[cfg_attr(feature="interface", cw_orch::interface_entry_point)] // <--- Add this line
pub fn execute(
   deps: DepsMut,
   env: Env,
   info: MessageInfo,
   msg: ExecuteMsg,
 -> StdResult<Response> {
    // ...
    Ok(Response::new())
}
// ... Do the same for the other entry points (query, migrate, reply, sudo)
```

By adding these lines, we generate code whenever the `interface` macro is enabled. The code generates a contract interface, the name of which will be the PascalCase of the crate's name.

## Example of Using Cw-Orchestrator

The following example provides a clear understanding of how to use `cw-orchestrator` with a smart contract. Here, we have a contract with a `Cargo.toml` file like the following:

```toml
# Cargo.toml
[package]
name = "example-contract"
# ...

[features]
# Features that are enabled by default
default = ["export"]
# Exports the WASM entry points, similar to the `library` feature
export = []
# Enables the contracts's interface
interface = ["dep:cw-orch"]

[dependencies]
cw-orch = {version = "0.10.0", optional = true }
# ...
```

Then our contract looks something like:

```rust
# use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};
# use cosmwasm_schema::{entry_point};
# struct InstantiateMsg;
# struct ExecuteMsg;
# struct QueryMsg;
# struct MigrateMsg;
// contract.rs
#[cfg_attr(feature = "export", entry_point)]
#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Instantiate contract
    Ok(Response::default())
}

#[cfg_attr(feature = "export", entry_point)]
#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // match statements
        _ => todo!()
    }
}

#[cfg_attr(feature = "export", entry_point)]
#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // match statements
        _ => todo!()
    }
}

#[cfg_attr(feature = "export", entry_point)]
#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    // ...
    Ok(Response::default())
}
```

The macros generate an `ExampleContract` struct that is now available in `contract.rs`.  
You can now create a test in `contract/tests` and start interacting with the contract as shown below:

<!-- ```rust
{{#include ../../contracts/mock_contract/src/lib.rs:2:10}}
``` -->

```rust
# struct InstantiateMsg {};
# enum ExecuteMsg {
    #   Increment {}
# };
# enum QueryMsg {
    #   Config {}
# };
# struct MigrateMsg {};
// contract/tests/example.rs
# use cosmwasm_std::{Addr};
use cw_orch::prelude::*;
// import the generated interface
use example_contract::contract::ExampleContract;
#[test]
fn example_test() {
    // init mock environment
    let sender = Addr::unchecked("sender");
    // Init the mock environment (cw-multi-test App)
    let mock = Mock::new(&sender);
    // `new()` function is available to construct the contract interface
    let example_contract = ExampleContract::new("example_contract", mock);
    // Now we can start scripting!

    // Upload the contract to the mock
    example_contract.upload()?;

    // Instantiate the contract
    example_contract.instantiate(&InstantiateMsg { }, None, None)?;

    // Execute the newly instantiated contract
    example_contract.execute(&ExecuteMsg::Increment { }, None)?;

    // Query
    let resp: String = example_contract.query(&QueryMsg::Config { })?;

    // Migrate
    example_contract.migrate(&MigrateMsg { }, None)?;
}
```

## Interaction helpers

cw-orchestrator provides an additional macro to simplify contract calls and queries. The macro generates functions on the interface for each variant of the contract's ExecuteMsg and QueryMsg.

Enabling this functionality is very straight-forward. Find your `ExecuteMsg` and `QueryMsg` definitions and add the `ExecuteFns` and `QueryFns` derive macros to them like below:

```rust,no_run
use cosmwasm_schema::QueryResponses;
use cw_orch::{ExecuteFns, QueryFns};

#[cfg_attr(feature = "interface", derive(ExecuteFns))]
pub enum ExecuteMsg {
    Increment {},
    // ...
}

#[cfg_attr(feature = "interface", derive(QueryFns))]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(String)]
    Config {}
    // ...
}
```

Any variant of the `ExecuteMsg` and `QueryMsg` that has a `#[derive(ExecuteFns)]` or `#[derive(QueryFns)]` will have a function generated on the interface through a trait. The function will have the same name as the variant and will take the same arguments as the variant.

You can access these functions by importing the generated traits form the message file. The generated traits are named `ExecuteMsgFns` and `QueryMsgFns`.

```rust,ignore
// Import the generated traits
# use cosmwasm_std::{Addr};
use example_contract::msg::{ExecuteMsgFns, QueryMsgFns};
use cw_orch::prelude::*;

fn example_test() {
    // init mock environment
    let sender = Addr::unchecked("sender");
    // Init the mock environment (cw-multi-test App)
    let mock = Mock::new(&sender);
    // `new()` function is available to construct the contract interface
    let example_contract = ExampleContract::new("example_contract", mock);

    // Upload the contract to the mock
    example_contract.upload()?;

    // Instantiate the contract
    example_contract.instantiate(&InstantiateMsg { }, None, None)?;

    // Execute the increment endpoint
    example_contract.increment()?;

    // Query the config
    // Return type optional!
    let resp: QueryResponse = example_contract.config()?;
}
# example_test();
```

> The function arguments are ordered alphabetically to prevent breaking changes when struct fields are moved.
