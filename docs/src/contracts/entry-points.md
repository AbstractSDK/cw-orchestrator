
# Entry Point Function Generation

Contract execution and querying is so common that we felt the need to improve the method of calling them. To do this we created two macros: `ExecuteFns` and `QueryFns`. As their name implies they can be used to automatically generate functions for executing and querying your contract through the interface.

## Execution

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

## Query

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

## Additional Remarks on `QueryFns` and `ExecuteFns`

The `QueryFns` and `ExecuteFns` derive macros generate traits that are implemented on any Contract structure (defined by the [`interface` macro](./interfaces.md#creating-an-interface)) that have the matching execute and query types. Because of the nature of rust traits, you need to import the traits in your application to use the simplifying syntax. Those traits are named `ExecuteMsgFns` and `QueryMsgFns`.

Any variant of the `ExecuteMsg` and `QueryMsg` that has a `#[derive(ExecuteFns)]` or `#[derive(QueryFns)]` will have a function implemented on the interface (e.g. `CounterContract`) through a trait. Here are the main things you need to know about the behavior of those macros:

- The function created will have the snake_case name of the variant and will take the same arguments as the variant.
- The arguments are ordered in alphabetical order to prevent attribute ordering from changing the function signature.
- If coins need to be sent along with the message you can add `#[payable]` to the variant and the function will take a `Vec<Coin>` as the last argument.
- The `cw_orch::QueryFns` macro needs your `QueryMsg` struct to have the [`cosmwasm_schema::QueryResponses`](https://docs.rs/cosmwasm-schema/1.4.1/cosmwasm_schema/trait.QueryResponses.html) macro implemented (this is good practice even outside of use with `cw-orch`).

## Additional configuration

### `payable` Attribute

Let's see an example for executing a message (from a money market for instance).

```rust,ignore
    money_market.deposit_stable()?;
```

There's a problem with the above function. The money market only knows how much you deposit into it by looking at the funds you send along with the transaction. Cw-orchestrator doesn't ask for funds by default. However, to allow attaching funds to a transaction, you can add the `#[payable]` attribute on your enum variant like so:

```rust,ignore
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

### `fn_name` Attribute

```rust
#[derive(cw_orch::ExecuteFns)] 
pub enum ExecuteMsg{
    Execute{
        msg: CosmoMsg
    }
}
```

The following command will error because the `execute` function is reserved for contract execution. This will not even compile actually.

```rust,ignore
// Doesn't compile
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

### `impl_into` Attribute

For nested messages (execute and query) you can add an `impl_into` attribute. This expects the enum to implement the `Into` trait for the provided type. This is extremely useful when working with generic messages:

```rust
{{#include ../../../cw-orch/tests/impl_into.rs:impl_into}}
```

### `disable_fields_sorting` Attribute

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
