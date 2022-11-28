# Interfaces

To get started with BOOT, create a new folder in your project's package directory and add it to the workspace members.

```bast
cd packages
cargo init --lib interfaces
cd interfaces
```

Following this example, the project's structure should eventually look like:

```path
.
├── Cargo.toml
├── my-contract
│   ├── Cargo.toml
│   └── src
│       ├── contract.rs (execute, instantiate, query, ...)
│       └── ..
├── packages
│   ├── my-project
│   │   └── my-contract.rs (msgs)
│   └── interfaces
│       └── my-contract.rs (interface)
└── scripts
    ├── Cargo.toml
    └── src
        └── bin
            ├── deploy.rs
            └── test_my_contract.rs
```

Now add `boot-core` to `Cargo.toml` along with the package that contains the contract's endpoint messages.

```toml
[dependencies]
boot-core = "0.1.4" # latest version as of writing this article
my-project = { path = "../my-project"}
```

## Defining Contract Interfaces

The contract interface is a struct that provides accessible methods to deploy and interact with an instance of your contract. Let's see how to use it.

First, create a new file in the src directory of the interfaces package, and add it to the library declaration

```rust
touch src/my-contract.rs
echo 'pub mod my-contract;' >> src/lib.rs
```

In your new contract file, define a struct for your contract interface and provide the [`Instantiate`|`Execute`|`Query`|`Migrate`] messages to the `boot_contract` macro, which will generate fully-typed instantiate, execute, query, and migrate methods for this struct.

```rust
use boot_core::prelude::*;
use my_project::my_contract::{InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg};

#[boot_contract(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct MyContract<Chain>;
```

The generic "\<Chain\>" argument allows you to write functions for your interface that will be executable in different environments.

> *If your entry point Msgs have any generic arguments, pull them out into newtypes before passing into the macro.*

## Constructor

Next, you'll want to define the constructor for the interface we just defined:

```rust
impl<Chain: BootEnvironment> MyContract<Chain> {
    /// Construct a new instance of MyContract
    /// * `contract_id` - what your contract should be called in local state (*not* on-chain)
    /// * `chain` - the environment to deploy to
    pub fn new(contract_id: &str, chain: &Chain) -> Self {
        // Use an absolute path
        let wasm_path = "../../target/wasm32-unknown-unknown/release/my-contract.wasm";
       // OR give the contract name and set ARTIFACTS_DIR environment variable to the artifacts folder. 
       let wasm_path = "my-contract";
        Self(
            Contract::new(contract_id, chain)
            .with_mock(Box::new(
                  ContractWrapper::new_with_empty(
                    my_contract::contract::execute,
                    my_contract::contract::instantiate,
                    my_contract::contract::query,
                )))
                .with_wasm_path(wasm_path),
        )
    }
}
```

Notice that we build the `Contract` instance and point it to the contract code using `with_wasm_path()`, where we provide the contract name `my-contract`. This contract name will be used to search the artifacts directory (set by ARTIFACTS_DIR env variable) for a `my-contract.wasm`. Alternatively you can specify a path to the wasm artifact after running `RUSTFLAGS='-C link-arg=-s' cargo wasm` in the contract's directory. See the CosmWasm documentation on compiling your contract for more information.

## Functions

Now we can start writing executable functions for our contracts with ensured type safety.
We can define functions that are generic or that can only be used called in a specific environment.
The environments that are currently supported are:

1. [cw-multi-test](https://crates.io/crates/cw-multi-test)
2. Blockchain daemons [junod](https://github.com/CosmosContracts/juno), [osmosisd](https://github.com/osmosis-labs/osmosis),...

### Generic function

Generic functions can be executed over any environment.

```rust
impl<Chain: BootEnvironment> MyContract<Chain> {
    pub fn deploy(&self, instantiate_msg: &InstantiateMsg) -> Self {
        let sender = &self.chain.sender();
        self.upload()?;
        let resp = self.instantiate(&instantiate_msg, Some(sender), None)?;
        let my_contract_address = resp.instantiated_contract_address()?;
        log::info!("deployed my-contract to {}", my_contract_address);
    }
}
```

### Daemon-only functions

```rust
impl MyContract<Daemon> {
    pub fn send_ibc_transaction(&self, msg: &ExecuteMsg) -> Self {
        let resp = self.execute(&msg,None)?;
        let destination_port = resp.event_attr_value("ibc_transfer", "destination_port");?;
    }
}
```

### Mock-only functions

```rust
impl MyContract<Mock> {
    pub fn call_cw_20(&self, msg: &ExecuteMsg) -> Self {
        let cw_20 = 
        let destination_port = resp.event_attr_value("ibc_transfer", "destination_port");?;
    }
}
```

Script
Now that we have the interface written for our contract, we can start writing scripts to deploy and interact with it.
Setup
Like before, we're going to setup a new folder for our scripts. This time, we'll call it scripts and initialize it as a binary crate:
cargo init --bin scripts
If your cargo project is a workspace, be sure to add scripts to the [workspace].members array at the workspace root.
Your scripts will have basically the same dependencies as your contract interfaces, but with a few additions:
cargo add --path ../interfaces
and also add the anyhow and dotenv crates:
cargo add anyhow dotenv log
Env Configuration
The dotenv crate will allow us to load environment variables from a .env file. This is useful for setting up the chain configuration for your scripts.

# .env

# info, debug, trace

RUST_LOG=info

# where the contract wasms are located

ARTIFACTS_DIR="../artifacts"

# where to store the output state data

DAEMON_STATE_PATH="./daemon_state.json"

# Mnemonics of the account that will be used to sign transactions

LOCAL_MNEMONIC=""
TEST_MNEMONIC=""
MAIN_MNEMONIC=""
IMPORTANT: Make sure to exclude the .env file in your gitignore.
Main Function
Now that we have our dependencies setup, we can start writing our script. Either create a new file in the src directory of the scripts/src package, or use the main.rs file that was created by default.
This function is mostly just boilerplate, so you can copy and paste it into your new script file. It will just call your function and give you nicer error traces:
fn main() {
    dotenv().ok();
    env_logger::init();
    use dotenv::dotenv;
    if let Err(ref err) = deploy_contract() {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));
        ::std::process::exit(1);
    }
}
Deployment Function
First, we'll define a function that will deploy our contract to the chain. This function will setup the environment (connecting to the chain), deploy the contract, and return a Result with the contract address.
// scripts/src/my_contract.rs
use anyhow::Result;
use boot_core::networks;
use boot_core::prelude::{instantiate_daemon_env, NetworkInfo};
// Traits for contract deployment
use boot_core::interface::*;
use interfaces::my_contract::MyContract;
// Select the chain to deploy to
const NETWORK: NetworkInfo = networks::juno::UNI_5;
const CONTRACT_NAME: &str = "my-contract";
pub fn deploy_contract() -> anyhow::Result<String> {
    // Setup the environment
    let (_, _sender, chain) = instantiate_daemon_env(network)?;
    // Create a new instance of your contract interface
    let contract = MyContract::new(CONTRACT_NAME, &chain);
    // Upload your contract
    contract.upload()?;
    // Instantiate your contract
    let init_msg = InstantiateMsg {
        // ...
    };
    // The second argument is the admin, the third is any coins to send with the init message
    contract.instantiate(init_msg, None, None)?;
    // Load and return the contract address
    let contract_addr = contract.address()?;
    Ok(contract_addr)
}
Additional Scripts
So you have your contract deployed, but what now? You can write additional scripts to interact with your contract. For example, you can write a script to query the contract state, or to execute a contract method.
Here's an example of a script that queries the contract state:
// scripts/src/my_contract.rs
// use ...
use my_contract::{QueryMsg};
// ...
pub fn query_contract() -> Result<()> {
    // Setup the environment
    let (_, _sender, chain) = instantiate_daemon_env(NETWORK)?;
    // Create a new instance of your contract interface
    let contract = MyContract::new(CONTRACT_NAME, &chain);
    // Load the contract address (this will use the address set from the previous deploy script)
    let contract_addr = contract.address();
    // Query the contract
    let res = contract.query(QueryMsg::Balance {
      address: contract_addr,
    })?;
    // Print the result
    println!("{:?}", res);
    Ok(())
}
And one that executes a contract method:
// scripts/src/my_contract.rs
// use ...
use my_contract::{ExecuteMsg};
// ...
pub fn execute_contract() -> Result<()> {
    // Setup the environment
    let (_, _sender, chain) = instantiate_daemon_env(NETWORK)?;
    // Create a new instance of your contract interface
    let contract = MyContract::new(CONTRACT_NAME, &chain);
    // Load the contract address (this will use the address set from the previous deploy script)
    let contract_addr = contract.address();
    // Execute a contract method
    let res = contract.execute(ExecuteMsg::UpdateBalance {
      address: contract_addr,
      balance: Uint128::from(1000000u128),
    })?;
    // Print the result
    println!("{:?}", res);
    Ok(())
}
Refinement
You can also refine your contract interface to make it easier to use. For example, you can create a function that will execute a specific contract method and return the result, instead of having to call contract.execute and contract.query separately.
// interfaces/src/my_contract.rs
// Import the boot traits
use boot_core::interface::*;
// ...
impl<Chain: BootEnvironment> MyContract<Chain> {
    pub fn new(contract_id: &str, chain: &Chain) -> Self {
      // ...
    }
    /// Query the balance of an address
    /// `address` - the address to query
    pub fn balance(&self, address: Addr) -> Result<BalanceResponse> {
        let balance_query = QueryMsg::Balance { address };
        self.query(balance_query)
    }
    /// Update the balance of an address
    /// `address` - the address to update
    /// `balance` - the new balance
    pub fn update_balance(&self, address: Addr, balance: Uint128) -> Result<ExecuteResult> {
        let update_balance_msg = ExecuteMsg::UpdateBalance {
            address,
            balance,
        };
        self.execute(update_balance_msg)
    }
}
References
Boot Core
Boot Cw-plus
Abstract Contract Interfaces
