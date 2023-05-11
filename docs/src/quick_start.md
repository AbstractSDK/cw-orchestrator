# Cw-orchestrator quick-start

Getting started with cw-orchestrator is very easy. The first step to using orchestrator is adding `cw-orch` to your contract's toml.

## Add `cw-orch`

You can do this by running the following in the directory of your contract:

```shell
$ cargo add --optional cw-orch
> Adding cw-orch v0.10.0 to optional dependencies.
```

Or you can add it manually in your `Cargo.toml`:

```toml
[dependencies]
cw-orch = {version = "0.10.0", optional = true } # Latest version at time of writing
```

Now that we have added `cw-orch` as an optional dependency we will want to enable it through a feature. This ensures that the code added by `cw-orch` is not included in the wasm artifact of the contract. To do this add an `interface` feature to the `Cargo.toml` and enable `cw-orch` when it is enabled.

To do this include the following in the `Cargo.toml`:

```toml
[features]
interface = ["dep:cw-orch"]
```

> You can learn more about Rust features [here](https://doc.rust-lang.org/cargo/reference/features.html).

## Contract Interface

Now that we have the dependency set up you can add the `interface` macro to your contract's endpoints. This macro will generate an interface to your contract that you will be able to use to interact with your contract. Get started by adding the feature-flagged interface macro to the contract's endpoints:

```rust
// In `contract.rs`
#[cfg_attr(feature="interface", cw_orch::interface_entry_point)] // <--- Add this line
pub fn instantiate(
   deps: DepsMut,
   env: Env,
   info: MessageInfo,
   msg: InstantiateMsg,
 -> StdResult<Response> {
    // ...
}

#[cfg_attr(feature="interface", cw_orch::interface_entry_point)] // <--- Add this line
pub fn execute(
   deps: DepsMut,
   env: Env,
   info: MessageInfo,
   msg: ExecuteMsg,
 -> StdResult<Response> {
    // ...
}
// ... Do the same for the other entry points (query, migrate, reply, sudo)
```

By adding these lines we generate code whenever the `interface` macro is enabled.
The code will generate a contract interface. The contract interface will be the PascalCase of the crate's name.

> The name of the crate is defined in the `Cargo.toml` file of your contract.

## Example

Let's look at an example to solidify your understanding.
We have a contract with a `Cargo.toml` file roughly be like the following:

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

This macro generates a `ExampleContract` struct that is now available in `contract.rs`.

If we now create a test in `contract/tests` we can start interacting with it!

```rust
// contract/tests/example.rs

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
    example_contract.instantiate(&InstantiateMsg { ... }, None, None)?;

    // Execute the newly instantiated contract
    example_contract.execute(&ExecuteMsg::Increment { ... }, None)?;

    // Query
    let resp: QueryResponse = example_contract.query(&QueryMsg::Config { ... })?;

    // Migrate
    example_contract.migrate(&MigrateMsg { ... }, None)?;
}
```

## Interaction helpers

cw-orchestrator provides an additional macro to simplify contract calls and queries. The macro generates functions on the interface for each variant of the contract's ExecuteMsg and QueryMsg.

Enabling this functionality is very straight-forward. Find your `ExecuteMsg` and `QueryMsg` definitions and add the `ExecuteFns` and `QueryFns` derive macros to them like below:

```rust

#[cfg_attr(feature = "interface", derive(ExecuteFns))]
pub enum ExecuteMsg {
    Increment {},
    ...
}

#[cfg_attr(feature = "interface", derive(QueryFns))]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {}
    ...
}
```

Any variant of the `ExecuteMsg` and `QueryMsg` that has a `#[derive(ExecuteFns)]` or `#[derive(QueryFns)]` will have a function generated on the interface through a trait. The function will have the same name as the variant and will take the same arguments as the variant.

You can access these functions by importing the generated traits form the message file. The generated traits are named `ExecuteMsgFns` and `QueryMsgFns`.

```rust

// Import the generated traits
use example_contract::msg::{ExecuteMsgFns, QueryMsgFns};

#[test]
fn example_test() {
    // init mock environment
    let sender = Addr::unchecked("sender");
    // Init the mock environment (cw-multi-test App)
    let mock = Mock::new(&sender);
    // `new()` function is available to construct the contract interface
    let example_contract = ExampleContract::new("example_contract", mock);

    // ... upload and instantiate like before

    // Execute the increment endpoint
    example_contract.increment()?;

    // Query the config
    // Return type optional!
    let resp: QueryResponse = example_contract.config()?;
}

```

> The function arguments are ordered alphabetically to prevent breaking changes when struct fields are moved.
