# Quick-Start Guide <!-- omit in toc -->

Get ready to change the way you interact with contracts. The following steps will allow you to write clean code such as:

```rust,ignore
{{#include ../../contracts/counter/examples/deploy.rs:clean_example}}
```

In this quick-start guide, we will review the necessary steps in order to integrate `cw-orch` into a simple contract crate. [We review integration of rust-workspaces (multiple contracts) at the end of this page](#integration-in-a-workspace).

> **NOTE**: *Additional content*
>
>If you're moving quicker than everybody else, we suggest looking at <a href="https://github.com/AbstractSDK/cw-orch-counter-example/compare/e0a54b074ca1a894bb6e58276944cf2013d152f2..main" target="_blank">a before-after review of this example integration</a>. This will help you catch the additions you need to make to your contract to be able to interact with it using cw-orchestrator.

## Summary

- [Summary](#summary)
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
cargo add cw-orch
```

Alternatively, you can add it manually in your `Cargo.toml` file as shown below:

```toml
[dependencies]
cw-orch = {version = "0.21.0" } # Latest version at time of writing
```

> **NOTE**: Even if you include `cw-orch` in your dependencies here, it won't be included in your `wasm` contract. Learn more about this behavior in the section about [Wasm Compilation](contracts/wasm-compilation.md)

### Creating an Interface

When using a single contract, we advise creating an `interface.rs` file inside your contract's directory. You then need to add this module to your `lib.rs` file. This file should not be included inside you final wasm. In order to do that, you need to add `#[cfg(not(target_arch = "wasm32"))]` when importing the file.

```rust,ignore
{{#include ../../contracts/counter/src/lib.rs:custom_interface}}
```

Then, inside that `interface.rs` file, you can define the interface for your contract:

```rust,ignore
{{#include ../../contracts/counter/src/interface.rs:custom_interface}}

```

Learn more about the content of the interface creation specifics on [the interface page](./contracts/interfaces.md#creating-an-interface)

> **NOTE**: It can be useful to re-export this struct to simplify usage (in `lib.rs`):
>
>    ```rust,ignore
>    #[cfg(not(target_arch = "wasm32"))]
>    pub use crate::interface::CounterContract;
>    ```

### Interaction helpers

cw-orchestrator provides a additional macros that simplify contract calls and queries. The macro implements functions on the interface for each variant of the contract's `ExecuteMsg` and `QueryMsg`.

Enabling this functionality is very straightforward. Find your `ExecuteMsg` and `QueryMsg` definitions (in `msg.rs` in our example) and add the `ExecuteFns` and `QueryFns` derive macros to them like below:

```rust,ignore
{{#include ../../contracts/counter/src/msg.rs:exec_msg}}

{{#include ../../contracts/counter/src/msg.rs:query_msg}}
```

Make sure to derive the `#[derive(cosmwasm_schema::QueryResponses)]` macro on your query messages !

Find out more about the interaction helpers on [the interface page](./contracts/interfaces.md#entry-point-function-generation)

> **NOTE**: Again, it can be useful to re-export these generated traits to simplify usage (in `lib.rs`):
>
>    ```rust,ignore
>    pub use crate::msg::{ExecuteMsgFns as CounterExecuteMsgFns, QueryMsgFns as CounterQueryMsgFns};
>    ```

### Using the integration

Now that all the setup is done, you can use your contract in tests, integration-tests or scripts.

Start by importing your crate, in your `[dev-dependencies]` for instance:

```toml
counter-contract = { path = "../counter-contract"}
```

You can now use:

```rust,ignore
{{#include ../../contracts/counter/examples/deploy.rs:full_counter_example}}
```

## Integration in a workspace

In this paragraph, we will use the `cw-plus` repository as an example. You can review:

- <a href="https://github.com/AbstractSDK/cw-plus" target="_blank">The full integration code</a> with `cw-orch` added
- <a href="https://github.com/cosmwasm/cw-plus/compare/main...abstractsdk:main" target="_blank">The complete diff</a> that shows you all integration spots (if you want to go fast)

### Handling dependencies and features

When using workspaces, you need to add `cw-orch` as a dependency on all crates that include `ExecuteMsg` and `QueryMsg` used in your contracts.
You then add the `#[derive(ExecuteFns)]` and `#[derive(QueryFns)]` macros to those messages. 

Refer above to [Adding `cw-orch` to your `Cargo.toml` file](#adding-cw-orch-to-your-cargotoml-file) for more details on how to do that.

For instance, for the `cw20_base` contract, you need to execute those 2 steps on the `cw20-base` contract (where the `QueryMsg` are defined) as well as on the `cw20` package (where the `ExecuteMsg` are defined).

### Creating an interface crate

When using workspace, we advise you to create a new crate inside your workspace for defining your contract's interfaces. In order to do that, use:

```shell
cargo new interface --lib
cargo add cw-orch --package interface 
```

Add the interface package to your workspace `Cargo.toml` file

```toml
[workspace]
members = ["packages/*", "contracts/*", "interface"]
```

Inside this `interface` crate, we advise to integrate all your contracts 1 by 1 in separate files. Here is the structure of the `cw-plus` integration for reference:

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

When importing your crates to get the messages types, you can use the following command in the interface folder.

```shell
cargo add cw20-base --path ../contracts/cw20-base/
cargo add cw20 --path ../packages/cw20
```

### Integrating single contracts

Now that you workspace is setup, you can [integrate with single contracts](#single-contract-integration) using the above section

## More examples and scripts

You can find more example interactions on the `counter-contract` example directly in the `cw-orchestrator` repo:  

- Some examples <a href="https://github.com/AbstractSDK/cw-orchestrator/blob/main/contracts/counter/examples/deploy.rs" target="_blank">showcase interacting with live chains</a>.
- Some other examples show <a href="https://github.com/AbstractSDK/cw-orchestrator/tree/main/contracts/counter/tests" target="_blank">how to use the library for testing your contracts</a>.

> **FINAL ADVICE**: Continue to explore those docs to learn more about `cw-orch`.
> Why not go directly to [environment variables](./contracts/env-variable.md)?
