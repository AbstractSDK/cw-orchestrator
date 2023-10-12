# Quick-Start Guide <!-- omit in toc -->

Get ready to change the way you interact with contracts. The following steps will allow you to write such clean code :  
```rust
{{#include ../../contracts/counter/examples/deploy.rs:clean}}

```
In this quick-start guide, we will review the necessary steps in order to integrate `cw-orch` into a simple contract crate. [We review integration of rust-workspaces (multiple contracts) at the end of this page](#integration-in-a-workspace).


> **NOTE**: *Additional content*
>
>If you're moving quicker than everybody else, we suggest looking at [a before-after review of this example integration](https://github.com/AbstractSDK/cw-orch-counter-example/compare/e0a54b074ca1a894bb6e58276944cf2013d152f2..64623d2141c04e4ba42dc6f9ef1a1daccc932d4a). This will help you catch the additions you need to make to your contract to be able to interact with it using cw-orchestrator.


### Summary
- [Single Contract Integration](#single-contract-integration)
  - [Adding `cw-orch` to your `Cargo.toml` file](#adding-cw-orch-to-your-cargotoml-file)
  - [Creating an Interface](#creating-an-interface)
  - [Interaction helpers](#interaction-helpers)
  - [Using the integration](#using-the-integration)
- [Integration in a workspace](#integration-in-a-workspace)
  - [Handling dependencies and features](#handling-dependencies-and-features)
  - [Creating an interface crate](#creating-an-interface-crate)
  - [Integrating single contracts](#integrating-single-contracts)
- [More examples and scripts](#more-examples-and-scripts)


## Single Contract Integration

### Adding `cw-orch` to your `Cargo.toml` file

To use cw-orchestrator, you need to add `cw-orch` to your contract's TOML file. Run the command below in your contract's directory:

```shell
$ cargo add --optional cw-orch
```

Alternatively, you can add it manually in your `Cargo.toml` file as shown below:

```toml
[dependencies]
cw-orch = {version = "0.16.4", optional = true } # Latest version at time of writing
```

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

### Creating an Interface

When using a single contract, we advise you to create an `interface.rs` file along your existing contract files. You then need to add the module to your `lib.rs` file. Don't forget the *feature-flag* the file in order to be able to use `cw-orch` inside it
```rust
{{#include ../../contracts/counter/src/lib.rs:custom_interface}}
```

Then, going inside that `interface.rs` file, you can define the interface to your contract for use with `cw-orch` : 

```rust 
{{#include ../../contracts/counter/src/interface.rs:custom_interface}}

```

Find out more about the content of the interface creation specifics on [the interface page](./single_contract/interfaces.md#creating-an-interface)

> **NOTE**: It can be useful to re-export this struct to simplify usage (in `lib.rs`) : 
>
>    ```rust,ignore
>    #[cfg(feature = "interface")]
>    pub use crate::interface::CounterContract;
>    ```


### Interaction helpers

cw-orchestrator provides an additional macro to simplify contract calls and queries. The macro generates functions on the interface for each variant of the contract's `ExecuteMsg` and `QueryMsg`.

Enabling this functionality is very straightforward. Find your `ExecuteMsg` and `QueryMsg` definitions (in `msg.rs` in our example) and add the `ExecuteFns` and `QueryFns` derive macros to them like below.

```rust,no_run
{{#include ../../contracts/counter/src/msg.rs:exec_msg}}

{{#include ../../contracts/counter/src/msg.rs:query_msg}}
```

Find out more about the interaction helpers on [the interface page](./single_contract/interfaces.md#entry-point-function-generation)

> **NOTE**: Again, it can be useful to re-export these traits to simplify usage (in `lib.rs`) : 
>
>    ```rust,ignore
>    #[cfg(feature = "interface")]
>    pub use crate::msg::{ExecuteMsgFns as CounterExecuteMsgFns, QueryMsgFns as CounterQueryMsgFns};
>    ```

### Using the integration

Now that all the setup is done, you can use your contract in tests, integration-tests or scripts.

Start by importing your crate, with the `interface` feature enabled : 
```toml
counter-contract = { path = "../counter-contract", features = ["interface"] }
```

You can now use : 
```rust
{{#include ../../contracts/counter/examples/deploy.rs:full_counter_example}}
```

## Integration in a workspace

In this paragraph, we will use the `cw-plus` repository as an example. You can review : 
- [The full integration code](https://github.com/AbstractSDK/cw-plus) with `cw-orch` added
- [The complete diff](https://github.com/cosmwasm/cw-plus/compare/main...abstractsdk:main) that shows you all integration spots (if you want to go fast)

### Handling dependencies and features

When using workspaces, you need to do the 2 following actions on all crates that include `ExecuteMsg` and `QueryMsg` used in your contracts :
1. Add `cw-orch` as an optional dependency
2. Add an `interface` feature (allows to make sure `cw-orch` is not compiled into your `wasm` contract)

Refer above to [Adding `cw-orch` to your `Cargo.toml` file](#adding-cw-orch-to-your-cargotoml-file) for more details on how to do that.

For instance, for the `cw20_base` contract, you need to execute those 2 steps on the `cw20-base` contract (where the `QueryMsg` are defined) as well as on the `cw20` package (where the `ExecuteMsg` are defined).


### Creating an interface crate

When using workspace, we advise you to create a new crate inside your workspace for defining your contract interfaces. In order to do that, use : 
```shell
cargo new interface --lib
cargo add cw-orch --package interface 
```

Add the interface package to your workspace `Cargo.toml` file
```toml
[workspace]
members = ["packages/*", "contracts/*", "interface"]
```

Inside this `interface` crate, we advise to integrate all your contracts 1 by 1 in separate files. Here is the structure of the `cw-plus` integration for reference : 

```path
interface (interface collection)
├── Cargo.toml
└── src
    ├── cw1_subkeys.rs
    ├── cw1_whitelist.rs
    ├── cw20_base.rs
    ├── cw20_ics20.rs
    └── ..
```

When importing your crates to get the messages types, you can use the following command in the interface folder. Don't forget to activate the interface feature to be able to use the cw_orch functionalities. 

```shell
cargo add cw20-base --path ../contracts/cw20-base/ --features=interface
cargo add cw20 --path ../packages/cw20 --features=interface
```

### Integrating single contracts

Now that you workspace is setup, you can [integrate with single contracts](#single-contract-integration) using the above section

## More examples and scripts

You can find more example interactions on the `counter-contract` example directly in the `cw-orchestrator` repo :  

- Some examples [showcase interacting with live chains](https://github.com/AbstractSDK/cw-orchestrator/blob/main/contracts/counter/examples/deploy.rs).
- Some other examples show [how to use the library for testing your contracts](https://github.com/AbstractSDK/cw-orchestrator/tree/main/contracts/counter/tests).

> **FINAL ADVICE**: Continue to explore those docs to learn more about `cw-orch`. 
> Why not go directly to [environment variables](./single_contract/env-variable.md) ?