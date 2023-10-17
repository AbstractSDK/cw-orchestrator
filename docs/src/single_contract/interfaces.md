# Interfaces

Interfaces are virtual wrappers around CosmWasm contracts. They allow you to interact with your contracts in a type-safe way, and provide a convenient way to reason about contract interactions. Interfaces are the core reason why we built cw-orchestrator and we hope that you'll find them as useful as we do.

> **Reminder**: You can find the code for this example in the [cw-orch counter-contract folder](https://github.com/AbstractSDK/cw-orchestrator/tree/main/contracts/counter).
>
> If you are a fast or visual learner, you can find a [**Before**-**After**](https://github.com/AbstractSDK/cw-orch-counter-example/compare/e0a54b074ca1a894bb6e58276944cf2013d152f2..64623d2141c04e4ba42dc6f9ef1a1daccc932d4a) view of the `cw-orch` integration process in the sample contract. 

## Setup

Before we can create an interface we need to add cw-orch to the contract's `Cargo.toml` file. In `counter` run:

```shell
$ cargo add --optional cw-orch
> Adding cw-orch v0.16.0 to optional dependencies.
```

or add it manually to the `counter/Cargo.toml` file:

```toml
[dependencies]
cw-orch = {version = "0.16.0", optional = true } # Latest version at time of writing
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

Now that we have our dependency set up we can create the interface. `cw-orch` provides a powerful `interface` macro that allows you to create an interface for your contract without having to call it at the entry points, as well as the ability to specify the contract's source more easily.

To use this macro, first create a new file in the `contract/src` directory called `interface.rs`. This is where we will expose our interface.

In `counter/src/lib.rs`:

```rust,ignore
{{#include ../../../contracts/counter/src/lib.rs:custom_interface}}
```

This allows importing the `interface.rs` file only when the `interface` feature is enabled on the contract. Again, this is done because we don't want cw-orch dependencies to end up in your resulting `Wasm` contract

Then in `counter/src/interface.rs`:

```rust,ignore
{{#include ../../../contracts/counter/src/interface.rs:custom_interface}}
```

This use of the `interface` macro even allows you to have generic arguments in the message types. Any generics will be added to the interface under a `PhantomData` attribute.

It can be beneficial to re-export the structure in our `lib.rs` file.

In the counter contract we re-export in `lib.rs`;

```rust,ignore
{{#include ../../../contracts/counter/src/lib.rs:interface_reexport}}
```

> **NOTE**: You can see that we have used the `artifacts_dir_from_workspace` macro inside the `wasm` trait function. This macro helps you locate the workspace `artifacts` folder. It actually looks for any directory named `artifacts` from the root of the current crate going up. For instance if the project is located in `/path1/path2/counter`, it will look for the artifacts folder inside the following directories in order and return as soon as it finds such a folder: 
> - `/path1/path2/counter`
> - `/path1/path2`
> - `/path1/`
> - ...
> 
> This works for single contracts as well as workspace setups. 
> If you have a specific setup, you can still specify the path yourself. If you do so, we advise indicating the wasm location from the current crate directory, using something like: 
>    ```rust 
>     let crate_path = env!("CARGO_MANIFEST_DIR");
>     let wasm_path = format!("{}/../../artifacts/counter_contract.wasm", crate_path);
>     WasmPath::new(wasm_path).unwrap()
>     ```
    


## Constructor

The `interface` macro implements a `new` function on the interface:

```rust,ignore
{{#include ../../../contracts/counter/tests/integration_tests.rs:constructor}}
```

The constructor takes two arguments:

1. `contract_id`: The unique identifier for this contract. This is used as the key when retrieving address and code_id information for the contract. This argument is a `&str`.
2. `chain`: The CosmWasm supported environment to use when calling the contract. Also includes the default sender information that will be used to call the contract. You can find more information later in the [Integrations](../integrations/index.md) section for how to create this `chain` variable

## Interacting with your contracts

Now, you are able to interact directly with your contracts with ensured type safety.

The environments that are currently supported are:

1. [cw-multi-test](https://crates.io/crates/cw-multi-test) by using [`Mock`](../integrations/cw-multi-test.md) as the `chain` variable.
2. Actual Cosmos SDK nodes for interacting with lives chains (`mainnet`, `testnet`, `local`). Use [`Daemon`](../integrations/daemon.md) as the `chain` variable.
3. [osmosis-test-tube](https://github.com/osmosis-labs/test-tube) or testing against actual chain binaries. This allows for fast testing with actual on-chain modules. This is particularly useful when testing against chain-specific modules. Use [`OsmosisTestTube`](../integrations/osmosis-test-tube.md) as the `chain` variable.

### Generic functions

Generic functions can be executed over any environment. Setup functions are a good example of this.

```rust,ignore
{{#include ../../../contracts/counter/tests/integration_tests.rs:setup}}
```

### Entry Point Function Generation

Contract execution and querying is so common that we felt the need to improve the method of calling them. To do this we created two macros: `ExecuteFns` and `QueryFns`. As their name implies they can be used to automatically generate functions for executing and querying your contract through the interface.

#### Execution

To get started, find the `ExecuteMsg` definition for your contract. In our case it's located in `counter/src/msg.rs`. Then add the following line to your `ExecuteMsg` enum:

```rust,ignore
{{#include ../../../contracts/counter/src/msg.rs:exec_msg}}
```

Again we feature flag the function generation to prevent cw-orchestrator entering as a dependency when building your contract.

The functions are implemented as a trait named `ExecuteMsgFns` which is implemented on any interface that uses this `ExecuteMsg` as an entrypoint message.

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


### Additional Remarks on `QueryFns` and `ExecuteFns`

The `QueryFns` and `ExecuteFns` derive macros generate traits that are implemented on any Contract structure (defined by the [`interface` macro](#creating-an-interface)) that have the matching execute and query types. Because of the nature of rust traits, you need to import the traits in your application to use the simplifying syntax. Those traits are named `ExecuteMsgFns` and `QueryMsgFns`.

Any variant of the `ExecuteMsg` and `QueryMsg` that has a `#[derive(ExecuteFns)]` or `#[derive(QueryFns)]` will have a function implemented on the interface (e.g. `CounterContract`) through a trait. Here are the main things you need to know about the behavior of those macros: 

- The function created will have the snake_case name of the variant and will take the same arguments as the variant. 
- The arguments are ordered in alphabetical order to prevent attribute ordering from changing the function signature. 
- If coins need to be sent along with the message you can add `#[payable]` to the variant and the function will take a `Vec<Coin>` as the last argument.
- The `cw_orch::QueryFns` macro needs your `QueryMsg` struct to have the [`cosmwasm_schema::QueryResponses`](https://docs.rs/cosmwasm-schema/1.4.1/cosmwasm_schema/trait.QueryResponses.html) macro implemented (this is good practice).

### Additional configuration

#### `payable` Attribute

Let's see an example for executing a message (from a money market for instance).

```rust,ignore
    money_market.deposit_stable()?;
```

There's a problem with the above function. The money market only knows how much you deposit into it by looking at the funds you send along with the transaction. Cw-orchestrator doesn't ask for funds by default. However, to allow attaching funds to a transaction, you can add the `#[payable]` attribute on your enum variant like so:

```rust, ignore
    #[derive(ExecuteFns)]
    enum ExecuteMsg{
        UpdateConfig{
            config_field: String
        },
        #[payable]
        DepositStable{}
        ...
    }
```

Be defining this attribute, you can now use:
```rust,ignore
    use cosmwasm_std::coins;
    money_market.deposit_stable(&coins(456, "ujunox"))?;
```

#### `fn_name` Attribute

```rust
#[derive(cw_orch::ExecuteFns)] 
pub enum ExecuteMsg{
    Execute{
        msg: CosmoMsg
    }
}
```
The following command will error because the `execute` function is reserved for contract execution. This will not even compile actually.

```rust
money_market.execute(message_to_execute_via_a_proxy)?;
```

This can happen in numerous cases actually, when using reserved keywords of cw-orch (or even rust). If this happens, you can use the `fn_name` attribute to rename a generated function.

```rust
#[derive(cw_orch::ExecuteFns)] 
pub enum ExecuteMsg{
    #[fn_name("proxy_execute")]
    Execute{
        msg: CosmoMsg
    }
}
// This works smoothly !
money_market.proxy_execute(message_to_execute_via_a_proxy)?;
```

This is also true for query functions.

#### `impl_into` Attribute

For nested messages (execute and query) you can add an `impl_into` attribute. This expects the enum to implement the `Into` trait for the provided type. This is extremely useful when working with generic messages:

```rust
{{#include ../../../cw-orch/tests/impl_into.rs:impl_into}}
```

#### `disable_fields_sorting` Attribute

By default the `ExecuteFns` and `QueryFns` derived traits will sort the fields of each enum member. For instance, 

```rust 
{{#include ../../../contracts/mock_contract/src/msg_tests.rs:ordered_msg_def}}
```
 will generate 
 ```rust
 pub fn bar(a: String, b: u64) -> ...{
    ...
 } 
 ```
You see in this example that the fields of the bar function are sorted lexicographically. We decided to put this behavior as default to prevent potential errors when rearranging the order of enum fields. If you don't want this behavior, you can disable it by using the `disable_fields_sorting` attribute. This is the resulting behavior: 

```rust 
{{#include ../../../contracts/mock_contract/src/msg_tests.rs:unordered_msg_def}}

 
 pub fn bar(b: u64, a: String) -> ...{
    ...
 } 
 ```
 
 > **NOTE**: This behavior CAN be dangerous if your struct members have the same type. In that case, if you want to rearrange the order of the members inside the struct definition, you will have to be careful that you respect the orders in which you want to pass them.




## Learn more

Got questions? Join the [Abstract Discord](https://discord.gg/vAQVnz3tzj) and ask in the `#cw-orchestrator` channel.
Learn more about Abstract at [abstract.money](https://abstract.money).

## References

- [cw-orchestrator](https://crates.io/crates/cw-orch)
- [cw-plus-orch](https://github.com/AbstractSDK/cw-plus)
- [Abstract Contract Interfaces](https://crates.io/crates/abstract-interface)
