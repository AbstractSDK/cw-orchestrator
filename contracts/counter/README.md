# Counter Contract

This guide features the counter contract as a prime example to demonstrate the capabilities of cw-orchestrator. Often serving as the introductory example for new CosmWasm developers, the counter contract is an ideal candidate for showcasing the functionalities of cw-orchestrator.

For a high-level overview tutorial that explains how to adapt a contract for compatibility with cw-orchestrator, please refer to [our documentation.](https://orchestrator.abstract.money/single_contract/interfaces.html) This README aims to provide a more detailed set of instructions, serving as an extended summary of that tutorial.

## Struct Creation and Endpoint Registration

### Contract Macros

The most straightforward way to integrate cw-orchestrator into your contracts is by using the `interface_entry_point` macro. Applying this macro to each of your contract's entry-points will generate a struct that is compatible with cw-orchestrator, while also registering the necessary variables.

Take a look at the [contract.rs](./src/contract.rs) file, it's that easy!
The supported endpoints are:

- Instantiate
- Query
- Execute
- Migrate
- Reply
- Sudo

You can see in the [contract.rs](./src/contract.rs) file that we feature flag that macro in order to not include cw-orch into the contract's build artifact. You have to realize that including this macro with a feature flag **won't change anything** to the actual wasm file that you are uploading on-chain if you don't enable this feature by default.
A good practice is to put this macro declaration right after the `cosmwasm_std::entry_point` macro definition.

> **_NOTE_** In order to be compatible with this method of integration, there are a few prerequisites. If your contract doesn't match **one** of those criteria, you should opt for the [manual way](#manual-integration) approach. It's not much more effort, you can do it! Here are the pre-requisites:
> - All the entry-points must be defined in the same file. This is required because of the way traits are implemented for the generated struct.
> - Your artifacts (*.wasm files) should be contained in the artifacts folder of the crate OR the workspace. i.e. there should be a folder named `artifacts` in the root of your crate or workspace.
>  >   If you define your contract with this method, cw-orchestrator will look for artifacts files that match your crate name (with `-` converted to `_`) in an `artifacts` folder. It starts looking in the root of the current crate and goes up until it finds a directory named `artifacts` or fails.
>  >  For instance, here, the artifacts are located at the [workspace root](../../artifacts/). The file named [counter_contract.wasm](../../artifacts/counter_contract.wasm) will be used by this struct to upload the contract on-chain. 

**Integrating via this method will create a struct which is named after the current crate's name converted to PascalCase.** This structure has a `new` constructor that you can use [following the steps in our documentation](https://orchestrator.abstract.money/single_contract/interfaces.html#constructor).

For example in the `counter` contract, it will define an `CounterContract` struct inside the `contract.rs` file because the package name is `counter-contract`, defined [here](../counter/Cargo.toml).

### Manual Integration

If for any reason you don't want to use the beautiful automatic version we described above, you can still do things manually, we have [a nice tutorial here](https://orchestrator.abstract.money/single_contract/interfaces.html#customizable-interface-macro). This will allow you to customize the entry-points of your contract as well as the location of the artifacts directory for your wasm files.

## Endpoint Function Generation

<table>
<tr>
<th> Tired of having to use endless schemas?</th>
<th> Tired of having to redeclare your field names every time you want to declare an struct?</th>
</tr>
<tr style="vertical-align: top;">
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

With orchestrator, you know your types **and** you get to use the Rust syntax you are used to! Your code can now look like this:
```rust,ignore
    dex.swap(CwAsset::Native("ujuno"), CwAsset::Native("uluna"), 3465u128.into())
```

In order to be able to interact in this manner with your code, you just need to add : 
- `#[derive(cw_orch::ExecuteFns)]` above of your `ExecuteMsg` definitions [You can see an example here](./src/msg.rs)
- `#[derive(cw_orch::QueryFns)]` above of your `QueryMsg` definitions [You can see an example here](./src/msg.rs)

And you are all set!

### Details

Actually, the query macro does need additional definitions. If you haven't already, you need to define the `cosmwasm_schema::QueryResponses` macro. This is used by cw-orchestrator to determine the return type of your queries. This is absolutely marvelous and your code can be transformed:

```rust,ignore
    // Previous
    let balance: BalanceResponse = cosmwasm_smart_query("my_cw20_address", cw20::QueryMsg::Balance{
        address: "my_address".to_string(),
    })?;

    // Now
    let balance = my_cw20.balances("my_address".to_string())?;
```

Better right ?

### Additional attributes

#### `payable`

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
    money_market.deposit_stable(coins(456, "ujunox"))?;
```

#### `fn_name`

Be careful what you name your enum variants, you could stumble upon one of the reserved keywords of cw-orch. If this happens, you can use the fn_name attribute to rename a generated function.

```rust,ignore

    ExecuteMsg{
        #[fn_name("proxy_execute")]
        Execute{
            msg: CosmoMsg
        }
    }

    // Will error because the execute function is reserved for contract execution. Will not even compile actually
    money_market.execute(message_to_execute_via_a_proxy)?;
```

```rust,ignore
    ExecuteMsg{
        #[fn_name("proxy_execute")]
        Execute{
            msg: CosmoMsg
        }
    }
    // This works smoothly !
    money_market.proxy_execute(message_to_execute_via_a_proxy)?;
```

This is also true for query functions.


#### `impl_into`

For more details about this attribute, [see the full documentation](https://orchestrator.abstract.money/single_contract/interfaces.html#impl_into-attribute)

## Other cw-orch traits

### Deploy

cw-orchestrator defines the [Deploy](../../cw-orch/src/deploy.rs) trait that allows developers to create an interface that allows themselves or other developers to:

1. Easily deploy their codebase on a test chain or elsewhere. [See it live here](https://github.com/AbstractSDK/abstract/blob/main/framework/scripts/src/bin/full_deploy.rs#L29).
2. Provide bindings (addresses, code_ids), for other developers to integrate with the platform. [See it live here](https://github.com/AbstractSDK/abstract/blob/main/modules/contracts/apps/croncat/examples/deploy.rs#L32).

This is more powerful than simply exposing the single contract interfaces, because it allows full customization of the deployment procedure. It also allows shipping other custom methods for simplifying the life of the integrator.

Imagine you are a developer that wants to integrate with [Abstract](https://github.com/AbstractSDK/). With this deploy trait, you could test your application like so:

```rust,ignore
use abstract_interface::Abstract;
use cw_orch::daemon::networks::JUNO_1;

pub fn test() -> anyhow::Result<()>{
    let chain = Daemon::builder()
        .chain(JUNO_1) 
        .handle(Runtime::new().unwrap().handle()) 
        .build()?;
        
    // Here you load all the contracts from the addresses and code_ids that abstract ships along with the Deploy trait they implement
    let abstract_deploy = Abstract::load_from(chain.clone())?;
    
    // Then you can call abstract specific commands without having to specify any addresses yourself. 
    // You just need to import the crate and you can interact with it directly
    let new_account = abstract_deploy.account_factory.create_new_account(
        AccountDetails{
            name: "New account name, input something funny ?".to_string(),
            description: Some("Account description".to_string()),
            link: None,
            namespace: None,
            base_asset: None,
            install_modules: vec![],
        },
        GovernanceDetails::Monarchy{
            monarch: "<monarch-address>"
        }
    )?;

    println!("Created new abstract account with manager address : {}", new_account.manager.address()?);
    Ok(())
}
```

In our example, all the abstract core addresses, which we require to interact with the contracts, are included in the `abstract_interface` crate directly. By using this Deploy trait, Abstract includes the addresses of their deployment and code ids in their published software, they allows other developers to easily interact and integrate with their platform.

To do this for your project you need to verify certain conditions.

#### Conditions for shipping addresses in a crate that implements the Deploy trait

Inside the deploy trait functions, you can define multiple methods. The two principle methods are:

- `Deploy::store_on` : for storing all contracts in the bundle on chains (Upload)
- `Deploy::deploy_on` : One stop function for deploying your bundle on a chain. This usually calls the `store_on` method before instantiating all contracts sequentially

After those functions are implemented you can link up the deployment data with your objects. For users to be able to retrieve addresses, they need to come from somewhere. In order for bundle maintainers to allow that, they should implement the `Deploy::load_from`, just like we do at abstract:

```rust,ignore
    fn deployed_state_file_path(&self) -> Option<String> {
        let crate_path = env!("CARGO_MANIFEST_DIR");

        Some(
            PathBuf::from(crate_path)
                // State file of your deployment
                .join("state.json")
                .display()
                .to_string(),
        )
    }

    fn load_from(chain: Chain) -> Result<Self, Self::Error> {
        let mut abstr = Self::new(chain);
        // We set all the contract's default state (addresses, code_ids)
        abstr.set_contracts_state();
        Ok(abstr)
    }
```

The `Deploy::set_contracts_state` doesn't need to be re-implemented and it allows to override the daemon state file and use the deployed addresses and code_ids instead. Doing this decouples the local state of the users from the deployed state of the maintainers/project. 

You can customize the `Deploy::deployed_state_file_path` and `Deploy::load_from` methods, be we recommend doing something similar to simplify exporting the state correctly. 


> **_NOTE_** You should check out the [Abstract bundle implementation](https://github.com/AbstractSDK/abstract/blob/main/framework/packages/abstract-interface/src/deployment.rs) to understand how to ship a bundle. In abstract, we have a file named `state.json` in the crate root with all the abstract state. We refer to it by referring to the absolute crate path and adding `"state.json"` to it. 
> 
> We recommend defining this location from the absolute crate path `env!("CARGO_MANIFEST_DIR")` for it to be accessible even when imported from a crate. 

Our abstract workspace structure looks like this : 

```
.
├── artifacts
├── contracts
│   ├── contract1
│   │   └── src
│   │       ├── contract.rs
│   │       └── ...
│   └── contract2
│       └── src
│           ├── contract.rs
│           └── ...
├── packages
|   └── interface 
|       ├── src
│       │   ├── deploy.rs   // <-- Definition of the deploy struct and implementation of the Deploy trait. 
|       │   |               // <--   Leverages contract1 and contract2 structures
│       │   └── ...
│       └── state.json      /// <-- Usually a symlink to the state.json file you use for deployment (by default in ~/.cw-orchestrator)
├── scripts
|   └── src
|       └── bin             // <-- Your deployment script can be located here
└── .env 					// <-- Place your .env file at the root of your workspace
```

In the Deploy trait implementation (here in`deploy.rs` file), use this to indicate that `packages/interface/state.json` has your state : 
```rust
    fn deployed_state_file_path(&self) -> Option<String> {
        let crate_path = env!("CARGO_MANIFEST_DIR");

        Some(
            PathBuf::from(crate_path)
                .join("state.json")
                .display()
                .to_string(),
        )
    }
```

Make sure you are adding the `set_contract_state`, helper function in the `load_from` function of the `Deploy` trait to make sure your deployment leverages the saved contract addresses. 

```rust

    fn load_from(chain: Chain) -> Result<Self, Self::Error> {
        let mut abstr = Self::new(chain);
        // We register all the contracts default state
        abstr.set_contracts_state();
        Ok(abstr)
    }
```


Those 2 steps will allow users to access your state from their script when importing you `interface` crate.

#### Limitations


## Learn More
Don't hesitate to check out the [examples](./examples) and [tests](./tests) of this crate to learn more and get inspiration from code directly