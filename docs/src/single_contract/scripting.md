# Writing and Executing Scripts

Now that we have the interface written for our contract, we can start writing scripts to deploy and interact with it on a real blockchain. We'll do this by adding a `bin` folder in our contract and add our deploy script there.

## Setup

Before we get going we need to add the `bin` folder and tell cargo that it contains scripts. We can do this by creating a folder named bin in `counter` and creating a file in it called `deploy.rs`

```bash
mkdir counter/bin
touch counter/bin/deploy.rs
```

Then we want to add a new feature to our crate. We will call the feature `deploy` and it will enable interface feature as well as setting the `daemon` feature on `cw-orch`.

```toml
[features]
# ...
deploy = ["interface", "cw-orch/daemon", "dotenv", "env_logger"]


[dependencies]
# ...
# Deps for deployment
dotenv = { version = "0.15.0", optional = true } # Enables loading of .env files
env_logger = { version = "0.10.0", optional = true } # Enables logging to stdout
```

Finally, we need to add the bin to our `Cargo.toml` file. Add put a feature requirement on it:

```toml
[[bin]]
name = "deploy"
path = "bin/deploy.rs"
required-features = ["deploy"]
```

Now we're ready to start writing our script.

## Main Function

With the setup done, we can start writing our script. Our initial plan is to deploy the counter contract to the chain. We'll start by writing a main function that will call our deploy function.

```rust,ignore
{{#include ../../contracts/counter/bin/deploy.rs}}
```

## Deployment Function

Our `main` function will deploy our contract to Juno testnet. This function will setup the environment (connecting to the chain), deploy the contract, and return a `Result` with the contract address.

```rust
// scripts/src/my_contract.rs
use anyhow::Result;
use cw_orch::networks;
use cw_orch::{Addr, instantiate_daemon_env, NetworkInfo, DaemonOptionsBuilder};
// Traits for contract deployment
use cw_orch::interface::*;
use interfaces::my_contract::MyContract;

// Select the chain to deploy to
const NETWORK: NetworkInfo = networks::juno::UNI_6;
const CONTRACT_NAME: &str = "my-contract";

pub fn deploy_contract() -> anyhow::Result<Addr> {
    // Create a runtime for the asynchronous actions
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());

    // Specify the options for the blockchain daemon
    let options = DaemonOptionsBuilder::default()
        // or provide `chain_data`
        .network(NETWORK)
        .deployment_id("my_deployment_version")
        .build()?;

    // Setup the environment
    let (_sender, chain) = instantiate_daemon_env(&rt, options)?;

    // Create a new instance of your contract interface
    let mut contract = MyContract::new(CONTRACT_NAME, chain);
    // Upload your contract
    contract.upload()?;

    // Instantiate your contract
    let init_msg = InstantiateMsg {
        // ...
    };
    // The second argument is the admin, the third is any coins to send with the init message
    contract.instantiate(&init_msg, None, None)?;

    // Load and return the contract address
    let contract_addr = contract.address()?;
    Ok(contract_addr)
}
```

### Additional Scripts

So you have your contract deployed, but what now? You can write additional scripts to interact with your contract. For example, you can write a script to query the contract state, or to execute a contract method.

Here's an example of a script that queries the contract state:

```rust
// scripts/src/my_contract.rs
// use ...
use my_contract::{QueryMsg};
// ...

pub fn query_contract() -> anyhow::Result<()> {
    // Setup the environment
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let options = DaemonOptionsBuilder::default()
        .network(NETWORK)
        .build()?;
    let (_sender, chain) = instantiate_daemon_env(&rt, options)?;

    // Create a new instance of your contract interface
    let contract = MyContract::new(CONTRACT_NAME, chain);
    // Load the contract address (this will use the address set from the previous deploy script)
    let contract_addr = contract.address();
    // Query the contract
    let res = contract.query(&QueryMsg::Balance {
      address: contract_addr,
    })?;
    // Print the result
    println!("{:?}", res);
    Ok(())
}
```

 And one that executes a contract method:

```rust
// scripts/src/my_contract.rs
use cw_orch::*;
use my_contract::{ExecuteMsg, ExecuteMsgFnsDerive};
// ...

pub fn execute_contract() -> anyhow::Result<()> {
    // Setup the environment
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());
    let options = DaemonOptionsBuilder::default()
        .network(NETWORK)
        .build()?;
    let (_sender, chain) = instantiate_daemon_env(&rt, options)?;

    // Create a new instance of your contract interface
    let contract = MyContract::new(CONTRACT_NAME, chain);
    // Load the contract address (this will use the address set from the previous deploy script)
    let contract_addr = contract.address();
    // Execute a contract method
    let res = contract.execute(&ExecuteMsg::UpdateBalance {
      address: contract_addr,
      balance: Uint128::from(1000000u128),
    }, None)?;
    // OR, if you're usincg the `ExecuteMsgFnsDerive` derive macro
    let res = contract.update_balance(contract_addr, Uint128::from(1000000u128))?;
    // Print the result
    println!("{:?}", res);
    Ok(())
}
```
