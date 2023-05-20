# Cw-orchestrator quick-start

Getting started with cw-orchestrator is very easy. The snippets in this document are backed by an actual contact which you can check out [here](https://github.com/AbstractSDK/cw-orchestrator/tree/main/contracts/counter).

## Add `cw-orch` as an optional dependency

To start using orchestrator, add `cw-orch` to your contract's toml.
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

Now that we have added `cw-orch` as an optional dependency we will want to enable it through a feature. This ensures that the code added by `cw-orch` is not included in the wasm artifact of the contract. To do this add an `interface_entry_point` feature to the `Cargo.toml` and enable `cw-orch` when it is enabled.

To do this include the following in the `Cargo.toml`:

```toml
[features]
interface = ["dep:cw-orch"]
```

> You can learn more about Rust features [here](https://doc.rust-lang.org/cargo/reference/features.html).

## Contract Interface

Now that we have the dependency set up you can add the `interface_entry_point` macro to your contract's endpoints. This macro will generate an interface to your contract that you will be able to use to interact with your contract. Get started by adding the feature-flagged interface macro to the contract's endpoints:

```rust,no_run,noplayground
// in contract.rs
{{#include ../../contracts/counter/src/contract.rs:interface_entry}}

// ... Do the same for the other entry points (query, migrate, reply, sudo)
```

By adding these lines we generate code whenever the `interface_entry_point` macro is enabled.
The code will generate a contract interface. The contract interface will be the PascalCase of the crate's name.

It's a good idea to re-expose the interface in the crate's root so that it is easy to import:

```rust,no_run,noplayground
// in lib.rs
{{#include ../../contracts/counter/src/lib.rs:interface_reexport}}
```

> The name of the crate is defined in the `Cargo.toml` file of your contract.

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
