# Collaboration

The structure you defined in the [Deploy Wrapper section](./deploy.md) is very useful for testing and scripting with your platform. However, in most cases you also want other developers and smart-contracts builders to be able to test their application against your application logic. This can be done by publishing your deployment structures to [`crates.io`](https://crates.io)

We'll continue with the [Abstract](https://abstract.money) platform as an example. This platform can be used by developers to create smart contacts enhancing the wallet experience of their users. They create applications (apps) on top of the framework and need the framework to be present on their test chain to be able to test or script against it.

On actual blockchains (Mainnets and Testnets), users won't redeploy the full Abstract platform. Instead, we added the Abstract addresses and code_ids alongside the abstract interface so that users can access those contracts without having to specify all those variables by hand (or find them in some documentation scattered across the web).

This type of information can also be useful for users working with Dexes (which have pool, factory, router addresses).

We'll present in this guide how projects can provide this kind of information to their users directly through Rust dependencies.

## State File

In order for your users to get access to the state of your application (addresses and cods_ids), you need to implement the `deployed_state_file` method on the `Deploy` trait :

```rust
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
```

In this function, you indicate where the deployment file is located. To be able to have those addresses accessible to users importing the crate, we advise you need to specify the path using the `crate_path` variable that is automatically set by Cargo.

> **NOTE**: At Abstract for instance, we have a `scripts` crate which is different from the `interface` crate. In order to keep up with the deployment file location, we have symbolic link from `interface/state.json` to `scripts/state.json`. This symbolic link is turned automatically into an actual file when it's published the crate to [crates.io](https://crates.io).
>
> [Check out the Abstract setup here](https://github.com/AbstractSDK/abstract/).  

Then, in the `load_from` function, we advise using the following implementation:
```rust
    fn load_from(chain: Chain) -> Result<Self, Self::Error> {
        let mut abstr = Self::new(chain);
        // We set all the contract's default state (addresses, code_ids)
        abstr.set_contracts_state();
        Ok(abstr)
    }
```

where `Self::new` simply creates the contract structures. You don't need to reimplement the `set_contracts_state` function. It will use the state indicated in `Self::deployed_state_file_path` as long as contracts are not re-uploaded / instantiated. 

You can customize the `Deploy::deployed_state_file_path` and `Deploy::load_from` methods, be we recommend doing something similar to what we show above to avoid mistakes and errors.

For visual learners, the workspace looks something like this : 

```path
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
│       └── state.json      // <-- Usually a symlink to the state.json file you use for deployment (by default in ~/.cw-orchestrator)
├── scripts
|   └── src
|       └── bin             // <-- Your deployment script can be located here
└── .env                    // <-- Place your .env file at the root of your workspace
```
