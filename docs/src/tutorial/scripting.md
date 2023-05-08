# Writing and Executing Scripts

Now that we have the interface written for our contract, we can start writing scripts to deploy and interact with it.

## Setup

Like before, we're going to setup a new folder for our scripts. This time, we'll call it `scripts` and initialize it as a binary crate:

```bash
cargo init --bin scripts
```

> If your cargo project is a workspace, be sure to add `scripts` to the [workspace].members array at the workspace root.

Your scripts will have basically the same dependencies as your contract interfaces, but with a few additions:

```bash
cargo add --path ../packages/interfaces
```

and also add the `dotenv` crate:

```bash
cargo add anyhow dotenv log
```

and, we must enable the `daemon` feature on `cw_orch`

```bash
cargo add cw_orch --features daemon
```

## Main Function

Now that we have our dependencies setup, we can start writing our script. Either create a new file in the `src` directory of the `scripts/src` package, or use the `main.rs` file that was created by default.

This function is mostly just boilerplate, so you can copy and paste it into your new script file. It will just call your function and give you nicer error traces:

```rust
fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = deploy_contract() {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        // The backtrace is not always generated. Try to run this example
        // with `$env:RUST_BACKTRACE=1`.
        // log::debug!("backtrace: {:?}", err.backtrace());

        ::std::process::exit(1);
    }
}
```

## Deployment Function

First, we'll define a function that will deploy our contract to the chain. This function will setup the environment (connecting to the chain), deploy the contract, and return a `Result` with the contract address.

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
