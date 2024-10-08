# Interfaces

Interfaces are virtual wrappers around CosmWasm contracts. They allow you to interact with your contracts in a type-safe way, and provide a convenient way to reason about contract interactions. Interfaces are the core reason why we built cw-orchestrator and we hope that you'll find them as useful as we do.

> **Reminder**: You can find the code for this example in the <a href="https://github.com/AbstractSDK/cw-orchestrator/tree/main/contracts/counter" target="_blank">cw-orch counter-contract folder</a>.
>
> If you are a fast or visual learner, you can find a <a href="https://github.com/AbstractSDK/cw-orch-counter-example/compare/e0a54b074ca1a894bb6e58276944cf2013d152f2..17f505758734fd3cfe3b4f7cede341376c930e98" target="_blank">**Before**-**After**</a> view of the `cw-orch` integration process in the sample contract.

## Creating an Interface

Now that we have our filesystem and crate setup, we are able to create our contract interface using the `cw-orch::interface` macro. It allows you to create an interface for your contract without having to call it at the entry points, as well as the ability to specify the contract's source more easily.

```rust,ignore
{{#include ../../../contracts-ws/contracts/counter/src/interface.rs:custom_interface}}
```

The use of the `interface` macro even allows you to have generic arguments in the message types. Any generics will be added to the interface under a `PhantomData` attribute.

It can be beneficial to re-export the structure in our `lib.rs` file.

In the counter contract we re-export in `lib.rs`;

```rust,ignore
{{#include ../../../contracts-ws/contracts/counter/src/lib.rs:interface_reexport}}
```

> **NOTE**: You can see that we have used the `artifacts_dir_from_workspace` macro inside the `wasm` trait function. This macro helps you locate the workspace `artifacts` folder. It actually looks for any directory named `artifacts` from the root of the current crate going up. For instance if the project is located in `/path1/path2/counter`, it will look for the artifacts folder inside the following directories in order and return as soon as it finds such a folder:
>
> - `/path1/path2/counter`
> - `/path1/path2`
> - `/path1/`
> - ...
>
> This works for single contracts as well as workspace setups.
> If you have a specific setup, you can still specify the path yourself. If you do so, we advise indicating the wasm location from the current crate directory, using something like:
>
>    ```rust,ignore
>     let crate_path = env!("CARGO_MANIFEST_DIR");
>     let wasm_path = format!("{}/../../artifacts/counter_contract.wasm", crate_path);
>     WasmPath::new(wasm_path).unwrap()
>     ```

## Constructor

The `interface` macro implements a `new` function on the interface:

```rust,ignore
{{#include ../../../contracts-ws/contracts/counter/tests/integration_tests.rs:constructor}}
```

The constructor takes one argument:

- `chain`: The CosmWasm supported environment to use when calling the contract. Also includes the default sender information that will be used to call the contract. You can find more information later in the [Integrations](../integrations/index.md) section for how to create this `chain` variable

> **NOTE**: If you prefer working with different contract addresses for the same contract interface, you can remove the id argument in the `interface` macro:
>
> ```rust,ignore
>#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
>pub struct CounterContract;
>```
>
>The generated constructor will now take 2 arguments, the `contract_id` and the `chain`. This `contract_id` will allow you to specify which contract you want to interact with.
>
>```rust,ignore
>let contract = CounterContract::new("specific_counter_contract", chain.clone());
>```

## Interacting with your contracts

Now, you are able to interact directly with your contracts with ensured type safety.

The environments that are currently supported are:

1. <a href="https://crates.io/crates/cw-multi-test" target="_blank">cw-multi-test</a> by using [`Mock`](../integrations/cw-multi-test.md) as the `chain` variable.
2. Actual Cosmos SDK nodes for interacting with lives chains (`mainnet`, `testnet`, `local`). Use [`Daemon`](../integrations/daemon.md) as the `chain` variable.
3. <a href="https://github.com/osmosis-labs/test-tube" target="_blank">osmosis-test-tube</a> or testing against actual chain binaries. This allows for fast testing with actual on-chain modules. This is particularly useful when testing against chain-specific modules. Use [`OsmosisTestTube`](../integrations/osmosis-test-tube.md) as the `chain` variable.

### Generic functions

Generic functions can be executed over any environment. Setup functions are a good example of this.

```rust,ignore
{{#include ../../../contracts-ws/contracts/counter/tests/integration_tests.rs:setup}}
```

### Entry point function generation

<table>
<tr>
<td> Tired of having to use endless schemas?</td>
<td> Tired of having to redeclare your field names every time you want to declare an struct?</td>
</tr>
<tr>
    <td>

```json
{
    "swap": {
        "offer_asset": {
            "native":{
                "denom":"ujuno"
            }
        },
        "ask_asset": {
            "native":{
                "denom":"uluna"
            }
        },
        "amount": "3465"
    }
}
```

</td>
<td>

```rust,ignore
dex::core::swap::ExecuteMsg::Swap{
    offer_asset: CwAsset::Native("ujuno"),
    ask_asset: CwAsset::Native("uluna"),
    amount: 3465u128.into()
}
```

</td>
</tr>
</table>

Learn more in the [next section about entry-point functions](./entry-points.md) how to do just that!

## Learn more

Got questions? Join the <a href="https://discord.gg/vAQVnz3tzj" target="_blank">Abstract Discord</a> and ask in the `#cw-orchestrator` channel.
Learn more about Abstract at <a href="https://abstract.money" target="_blank">abstract.money</a>.

## References

- <a href="https://crates.io/crates/cw-orch" target="_blank">cw-orchestrator</a>
- <a href="https://github.com/AbstractSDK/cw-plus" target="_blank">cw-plus-orch</a>
- <a href="https://crates.io/crates/abstract-interface" target="_blank">Abstract Contract Interfaces</a>
