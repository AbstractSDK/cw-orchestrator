# Counter Contract 

We chose the counter contract to showcase the power of cw-orchestrator. This is the first example most CosmWasm devs were confronted to and is very easy to showcase the powers of cw-orchestrator.

You can find an overview tutorial on how to adapt a contract for use with cw-orch without the technical details in [our documentation here](https://orchestrator.abstract.money/single_contract/interfaces.html). This README is intended to give more detailed instructions and be a summary of this tutorial.

In this tutorial, we will discuss the installation of cw-orch in depth. 


In order to be able to interact with smart contracts using cw-orchestrator, you need to add a few lines to some precise points in your contract. There are two steps that you need to do 

## Struct Creation + Endpoint registration

### Contract Macros

The simplest way of integrating cw-orchestrator into your contracts is to use the `interface_entry_point` macro. By declaring this macro on all your endpoints, you will create a Contract struct compatible with cw-orchestrator and register all the necessary variables to it. Take a look at the [contract.rs](src/contract.rs) file, it's that easy ! 
The supported endpoints are : 
- Instantiate
- Query
- Execute
- Migrate
- Reply
- Sudo

You can see in the [contract.rs](src/contract.rs) file that we feature flag that macro in order to not include cw-orch into your contract. You have to realize that including this macro with a feature flag **won't change anything** to the actual wasm file that you are uploading on-chain, if you don't make this feature a default. 
A good practice is to put this macro declaration right after the `cosmwasm_std::entry_point` macro definition. 

> **_NOTE_** In order to be compatible with this method of integration, there are a few prerequisites. If your contract doesn't match **one** of those criteria, you should try the [manual way](#manual-integration). It's not much more effort, you can do it ! Here are the pre-requisites : 
> - You need to have all the endpoints definitions all in the same file. This is needed because we define traits for an the contract object
> - Your artifacts should be contained in the artifacts folder of the crate OR the workspace. 
>  >   If you define your contract with this method, cw-orchestrator will look for artifacts files that match your crate name (with `-` converted to `_`) in an `artifacts` folder. It starts looking in the root of the current crate and goes up until it finds such a folder or fails.
>  >  For instance, here, the artifacts are located at the [workspace root](../../artifacts/). The file named [counter_contract.wasm](../../artifacts/counter_contract.wasm) will be used by this struct to upload the contract on-chain. 

Integrating via this method will create a struct which name is the name of the current crate converted to PascalCase. This structure has a `new` constructor that you can use [following the steps in our documentation](https://orchestrator.abstract.money/single_contract/interfaces.html#constructor).

For instance here, it will create the CounterContract struct inside the contract file.


### Manual Integration

If for any reason you don't want to use the beautiful automatic version we described above, you can still do things manually, we have [a nice tutorial here](https://orchestrator.abstract.money/single_contract/interfaces.html#customizable-interface-macro). This will allow you to customize the entry-points of your contract as well as the location of the artifacts directory for your wasm files

## Endpoint Function Generation

<table>
<tr>
<th> Tired of having to use endless schemas ?</th>
<th> Tired of having to redeclare your field names every time you want to declare an struct ?</th>
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


With orchestrator, you know your types **and** you get to use the Rust syntax you are used to ! Your code can now look like this : 
```rust,ignore
    dex.swap(CwAsset::Native("ujuno"), CwAsset::Native("uluna"), 3465u128.into())
```

In order to be able to interact in this manner with your code, you just need to add : 
- `#[derive(cw_orch::ExecuteFns)]` in front of your `ExecuteMsg` definitions [You can see an example here](./src/msg.rs)
- `#[derive(cw_orch::QueryFns)]` in front of your `QueryMsg` definitions [You can see an example here](./src/msg.rs)

And you are all set ! 

### Details
Actually, the query macro does need additional definitions. If you haven't already, you need to define the `cosmwasm_schema::QueryResponses` macro. This is actually used by cw-orchestrator to determine the return type of your queries. This is absolutely marvelous and your code now changes to : 

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

#### Payable
Let's see an example for executing a message (from a money market for instance)

```rust,ignore
    money_market.deposit_stable()?;
```

This money market however only knows how much you deposit into it by looking at the funds you send along with the transaction. Cw-orchestrator prevents that by default. However, to allow depositing funds with a transaction, you can add the `#[payable]` attribute on your enum variant like so : 

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

Be defining this attribute, you can now use : 
```rust,ignore
    money_market.deposit_stable(coins(456, "ujunox"))?;
```

#### fn_name
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


#### impl_into
For more details about this attribute, [see the full documentation](https://orchestrator.abstract.money/single_contract/interfaces.html#impl_into-attribute)




## Other cw-orch traits

### Deploy

cw-orchestrator defines the [Deploy](../../cw-orch/src/deploy.rs) trait that allows developers to create an interface that allows themselves or other developers to : 
1. Simply deploy their codebase on a test chain or elsewhere. [See it live here](https://github.com/AbstractSDK/abstract/blob/main/framework/scripts/src/bin/full_deploy.rs#L29).
2. Provide bindings (addresses, code_ids), for other developers to integrate with the platform. [See it live here](https://github.com/AbstractSDK/abstract/blob/main/modules/contracts/apps/croncat/examples/deploy.rs#L32).

This is even more powerful than simply exposing the single contract interfaces, because it allows full customization of the deployment procedure. It also allows shipping other custom methods for simplifying the life of the integrator.

Imagine you are a developer that wants to integrate with [DA0-DA0](https://github.com/DA0-DA0). With this deploy trait, you could test your application like so : 

```rust,ignore
use daodao::core::DaoDao;
use cw_orch::daemon::networks::JUNO_1;

pub fn test(){
    let chain = Daemon::builder()
        .chain(JUNO_1) 
        .handle(Runtime::new().unwrap().handle()) 
        .build()
        .unwrap();
        
    let dao_id = 56u64;
    let daodao = DaoDao::load_from(chain.clone())?;
    daodao.set_account_id(dao_id)?;

    let my_custom_gov_admin = GovAdmin::new("gov-admin", chain)?;
    my_custom_gov_admin.upload()?;
    my_custom_gov_admin.instantiate(&InstantiateMsg{account_id: dao_id});

    daodao.nominate_admin(my_custom_gov_admin.address().to_string())?;
}
```
In our example, all the daodao core addresses are shipped with the daodao crate directly. Projects need to verify certain conditions in order to ship this state with the deploy bundle.

#### Conditions for shipping addresses in a bundle that implements the Deploy trait

Inside the deploy trait functions, you can define multiple methods. The two principle methods are : 
- `Deploy::store_on` : for storing all contracts in the bundle on chains (Upload)
- `Deploy::deploy_on` : One stop function for deploying your bundle on a chain. This usually calls the `store_on`` method before instantiating all contracts sequentially

After that comes some more complexity. For users to be able to retrieve addresses, they need to come from somewhere. In order for bundle maintainers to allow that, they should implement the `Deploy::load_from`, just like we do at abstract : 
```rust,ignore
    
    fn deployed_state_file_path(&self) -> Option<String> {
        let crate_path = env!("CARGO_MANIFEST_DIR");

        Some(
            PathBuf::from(crate_path)   
                .join("daemon_state.json")
                .display()
                .to_string(),
        )
    }

    fn load_from(chain: Chain) -> Result<Self, Self::Error> {
        let mut abstr = Self::new(chain);
        // We register all the contracts default state
        abstr.set_contracts_state();
        Ok(abstr)
    }
```

The `Deploy::set_contracts_state` doesn't need to be re-implemented and it allows to override the daemon state file and use the deployed addresses and code_ids instead. Doing this decouples the local state of the users from the deployed state of the maintainers/project. 

You can customize the `Deploy::deployed_state_file_path` and `Deploy::load_from` methods, be we recommend doing something similar to simplify exporting the state correctly. 


> **_NOTE_** You should check out the [Abstract bundle implementation](https://github.com/AbstractSDK/abstract/blob/main/framework/packages/abstract-interface/src/deployment.rs) to understand how to ship a bundle. In abstract, we have a file named `daemon_state.json` in the crate root with all the abstract state. We refer to it by referring to the absolute crate path and adding `"daemon_state.json"` to it. 
> 
> We recommend defining this location from the absolute crate path `env!("CARGO_MANIFEST_DIR")` for it to be accessible even when imported from a crate. 


#### Limitations


## Learn More
Don't hesitate to check out the [examples](./examples) and [tests](./tests) of this crate to learn more and get inspiration from code directly