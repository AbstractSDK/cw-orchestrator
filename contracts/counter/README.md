


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

    println!("Created new abstract account with manager address: {}", new_account.manager.address()?);
    Ok(())
}
```

In our example, all the abstract core addresses, which we require to interact with the contracts, are included in the `abstract_interface` crate directly. By using this Deploy trait, Abstract includes the addresses of their deployment and code ids in their published software, they allows other developers to easily interact and integrate with their platform.

To do this for your project you need to verify certain conditions.

#### Conditions for shipping addresses in a crate that implements the Deploy trait

Inside the deploy trait functions, you can define multiple methods. The two principle methods are:

- `Deploy::store_on`: for storing all contracts in the bundle on chains (Upload)
- `Deploy::deploy_on`: One stop function for deploying your bundle on a chain. This usually calls the `store_on` method before instantiating all contracts sequentially

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

Our abstract workspace structure looks like this: 

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

In the Deploy trait implementation (here in`deploy.rs` file), use this to indicate that `packages/interface/state.json` has your state:

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