# Interfaces

Interfaces are virtual wrappers around CosmWasm contracts. They allow you to interact with your contracts in a type-safe way, and provide a convenient way to reason about contract interactions. Interfaces are the core reason why we built cw-orchestrator and we hope that you'll find them as useful as we do.

You can find the code for this example in the [cw-orch counter-contract folder](https://github.com/AbstractSDK/cw-orchestrator/tree/main/contracts/counter).

## Setup

Before we can create an interface we need to add cw-orch to the contract's `Cargo.toml` file. In `counter` run:

```shell
$ cargo add --optional cw-orch
> Adding cw-orch v0.13.3 to optional dependencies.
```

or add it manually to the `counter/Cargo.toml` file:

```toml
[dependencies]
cw-orch = {version = "0.13.3", optional = true } # Latest version at time of writing
```

We add `cw-orch` as an optional dependency to ensure that it is not included in the wasm artifact of the contract. This way there are no trust assumptions made about the code added by `cw-orch`, making it safe to use for production contracts.

However, we will need a way to enable the dependency when we want to use it. To do this add an `interface` feature to the `Cargo.toml` and enable `cw-orch` when it is enabled.

You can do this by including the following in the `counter/Cargo.toml`:

```toml
[features]
interface = ["dep:cw-orch"] # Adds the dependency when the feature is enabled
```

Features (aka. feature flags) are a way to enable or disable parts of your code. In this case, we are including `cw-orch` as a dependency when the `interface` feature is enabled. This is a common pattern for feature flags.

## Creating an Interface

Now that we have our dependency set up we can create the interface. `cw-orch` provides two methods to easily create an interface for your contract.

The first is the `interface_entry_point` macro. This macro will generate an interface for your contract by calling it at the entry points of your contract. We'll cover this macro first as it's the easiest to use.

Alternatively you can also use the `interface` macro. This macro is more flexible and allows you to create an interface for your contract without having to call it at the entry points, as well as the ability to specify the contract's source more easily. We'll cover this macro in the end of this section.

### Entry Point Macro

As mentioned this macro is the easiest to use. It will generate an interface for your contract by calling it at the entry points of your contract. Here's an example of how to use it.

In `counter/src/contract.rs`:

```rust,ignore
{{#include ../../../contracts/counter/src/contract.rs:interface_entry}}
```

Most of this should look familiar but if you're wondering about the two lines that contain `#[...]` here's what they do:

1.
    ```rust,ignore
    {{#include ../../../contracts/counter/src/contract.rs:entry_point_line}}
    ```
   This is a CosmWasm macro. It enables the Wasm runtime to call into the function. You can read more about the macro in the [CosmWasm book](https://book.cosmwasm.com/basics/entry-points.html). We only enable this macro when the `export` feature is enabled. This prevents conflicts with other entry points when the contract is a dependency of another contract.

2.
    ```rust,ignore
    {{#include ../../../contracts/counter/src/contract.rs:interface_line}}
    ```
    This is the cw-orch provided macro. It will generate an interface for your contract by analyzing the messages passed to the entry points. This is possible because the entry point function definitions have strict parameter requirements. With this information the macro can generate a type safe interface for your contract. We only enable this macro when the `interface` feature is enabled.

### Customizable Interface Macro

The second method to create an interface is the `interface` macro. To use it, first create a new file in the `contract/src` directory called `interface.rs`. This is where we will expose our interface.

In `counter/src/lib.rs`:

```rust,ignore
{{#include ../../../contracts/counter/src/lib.rs:custom_interface}}
```

Then in `counter/src/interface.rs`:

```rust,ignore
{{#include ../../../contracts/counter/src/interface.rs:custom_interface}}
```

This use of the `interface` macro even allows you to have generic arguments in the message types. Any generics will be added to the interface under a `PhantomData` attribute.

## Constructor

Both macros implement a `new` function on the interface:

```rust,ignore
{{#include ../../../contracts/counter/tests/integration_tests.rs:constructor}}
```

The constructor takes two arguments:

1. `contract_id`: The unique identifier for this contract. This is used as the key when retrieving address and code_id information for the contract.
2. `chain`: The CosmWasm supported environment to use when calling the contract. Also includes the default sender information that will be used to call the contract.

## Custom Functions

Now you can start implementing custom functions for your interfaces with ensured type safety.

The environments that are currently supported are:

1. [cw-multi-test](https://crates.io/crates/cw-multi-test) by using [`Mock`](https://docs.rs/cw-orch/latest/cw_orch/prelude/struct.Mock.html) as the environment.
2. Blockchain daemons like [junod](https://github.com/CosmosContracts/juno), [osmosisd](https://github.com/osmosis-labs/osmosis), etc. These use the [`Daemon`](https://docs.rs/cw-orch/latest/cw_orch/prelude/struct.Daemon.html) environment.
3. Chain-backed mock `deps` for unit-testing. This uses the [`MockQuerier`](https://docs.rs/cw-orch/latest/cw_orch/live_mock/struct.MockQuerier.html) that resolves all queries on a real node over gRPC.

### Generic function

Generic functions can be executed over any environment. Setup functions are a good example of this.

```rust,ignore
{{#include ../../../contracts/counter/tests/integration_tests.rs:setup}}
```

### Daemon-only functions

```rust,ignore
{{#include ../../../contracts/counter/src/interface.rs:daemon}}
```

## Entry Point Function Generation

Contract execution and querying is so common that we felt the need to improve the method of calling them. To do this we created two macros: `ExecuteFns` and `QueryFns`. As their name implies they can be used to automatically generate functions for executing and querying your contract through the interface.

### Execution

To get started, find the `ExecuteMsg` definition for your contract. In our case it's located in `contracts/counter/src/msg.rs`. Then add the following line to your `ExecuteMsg` enum:

```rust,ignore
{{#include ../../../contracts/counter/src/msg.rs:exec_msg}}
```

Again we feature flag the function generation to prevent cw-orchestrator entering as a dependency when building your contract.

The functions are implemented as a trait named `ExecuteMsgFns` which is implemented on any interface that uses this `ExecuteMsg`.

Using the trait then becomes as simple as:

```rust,ignore
// in integration_tests.rs
{{#include ../../../contracts/counter/tests/integration_tests.rs:reset}}
```

### Query

Generating query functions is a similar process but has the added advantage of using the `cosmwasm-schema` return tags to detect the query's return type. This allows for type-safe query functions!

```rust,ignore
{{#include ../../../contracts/counter/src/msg.rs:query_msg}}
```

Using it is just as simple as the execution functions:

```rust,ignore
// in integration_tests.rs
{{#include ../../../contracts/counter/tests/integration_tests.rs:query}}
```

Just like the interface it can be beneficial to re-export the trait in your `lib.rs` or `interface.rs` file.

In the counter contract we re-export in `lib.rs`;

```rust,ignore
{{#include ../../../contracts/counter/src/lib.rs:fn_re_export}}
```

### `impl_into` Attribute

For nested messages (execute and query) you can add an `impl_into` attribute. This expects the enum to implement the `Into` trait for the provided type. This is extremely useful when working with generic messages:

```rust
use cw_orch::interface;
use cw_orch::prelude::*;

// An execute message that is generic.
#[cosmwasm_schema::cw_serde]
pub enum GenericExecuteMsg<T> {
    Generic(T),
}

// Now the following is possible:
type ExecuteMsg = GenericExecuteMsg<Foo>;

#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[impl_into(ExecuteMsg)]
pub enum Foo {
    Bar { a: String },
}

impl From<Foo> for ExecuteMsg {
    fn from(msg: Foo) -> Self {
        ExecuteMsg::Generic(msg)
    }
}

#[interface(Empty, ExecuteMsg, Empty, Empty)]
struct Example<Chain>;

impl<Chain: CwEnv> Example<Chain> {
    pub fn test_macro(&self) {
        // function `bar` is available because of the `impl_into` attribute!
        self.bar("hello".to_string()).unwrap();
    }
}
```

## Learn more

Got questions? Join the [Abstract Discord](https://discord.gg/vAQVnz3tzj) and ask in the `#cw-orchestrator` channel.
Learn more about Abstract at [abstract.money](https://abstract.money).

## References

- [cw-orchestrator](https://crates.io/crates/cw-orch)
- [cw-plus-orc](https://crates.io/crates/cw-plus-orc)
- [Abstract Contract Interfaces](https://crates.io/crates/abstract-cw-orch)
