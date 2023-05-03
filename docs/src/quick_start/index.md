# Quick start

Welcome to cw-orchestrator quick start guide.
Here we plan to quickly set you up with what you need to get going with using this amazing tool and start developing smart contracts really fast!

## Requirements

You will need this setup and configured to follow this quick start guide and in life in general! 😉

* [git]((https://git-scm.com/book/en/v2/Getting-Started-Installing-Git))
* [Rust (of course!)](https://www.rust-lang.org/learn/get-started)
* [Rust WASM target](https://www.hellorust.com/setup/wasm-target/)
* [Docker](https://docs.docker.com/engine/install/)

## Setup and configuration

### Dependencies

Now that we have that properly working (hopefully) we can run the following command
```bash
cargo init counter-contract && cd $_
```
This will create our crate and enter the folder. ($_ real helpful bash trick to get the last string in a command)

Now we need to add the following dependecies for our crate:
```bash
cargo add cosmwasm-std cw-multi-test cw-storage-plus cosmwasm-schema serde serde_json tokio dotenvy thiserror --no-default-features -F serde/derive -F tokio/rt
```

After we have added those we need to add the important crates, cw-orchestrator crates!
To add cw-orch run the following command:
```bash
cargo add --git https://github.com/AbstractSDK/cw-orchestrator.git cw-orch -F daemon
```

Now this is the last dependency we need:

```bash
cargo add --git https://github.com/AbstractSDK/cw-plus.git cw2
```

### Setting up a local Juno container

This little script can help you setup a local Juno container in Docker without too much trouble:
```bash
#!/bin/bash
docker run -d --name juno_node_1 -p 1317:1317 -p 26656:26656 -p 9090:9090 -e STAKE_TOKEN=ujunox -e UNSAFE_CORS=true ghcr.io/cosmoscontracts/juno:v14.0.0 ./setup_and_run.sh juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y
```

**NOTE**

You can also run it manually (without the `#!/bin/bash` part!), but remember that you might find yourself in need of removing the container, if you want to clear the state of it.
For example for re-uploading your contract!, if you dont have migration implemented in your contract already!

### Structure

This is the structure that we are going to be using for our contract:

```
├── Cargo.lock
├── Cargo.toml
└── src
    ├── contract.rs
    ├── error.rs
    ├── main.rs
    ├── msgs.rs
    └── state.rs
```

So we need to run the next command in our root crate directory

```bash
touch src/error.rs src/msgs.rs src/state.rs src/contract.rs .env
```

After we are done with that we need to edit our `Cargo.toml` and add the following above our `[dependencies]` entry:

```toml
[[bin]]
name = "contract"
path = "src/contract.rs"
```

### Environment variables

Now we need to edit our .env file to contain the next values:

```bash
# this is for cw-orchestrator, it's used to store our contract state information
STATE_FILE="./counter-contract-state.json"
# the mnemonic for the Juno genesis wallet that we are going to be using
LOCAL_MNEMONIC="clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose"
# the name of the Juno local test network
CHAIN="testing"
# the identificator for our contract deployment
DEPLOYMENT_ID="counter-contract-id"
```

## Coding the contract

lets start with the `src/state.rs` this file will hold our state variables.

```rust
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw_storage_plus::Item;

// we use the cw_serde macro to add different traits related to our struct
// that will help us work with it in general
#[cw_serde]
pub struct Count(pub Uint128);

// this will hold our data!
pub const COUNT: Item<Count> = Item::new("count");
```

now our `src/msgs.rs` file were we hold all our msgs to interact with our contract.

```rust
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

// This is the message that we are going to use to start our contract
#[cw_serde]
pub struct InstantiateMsg {
    // our value to set in our state
    pub initial_value: Uint128,
}

// The QueryMsg enum holds our varaints that we are going to use to get information out of our contract
#[cw_serde]
#[derive(QueryResponses)]
#[cfg_attr(feature = "interface", derive(QueryFns))]
pub enum QueryMsg {
    #[returns(CurrentCount)]
    GetCount,
}

// This is our response to our get_count query
#[cw_serde]
pub struct CurrentCount(pub Uint128);

// ExecuteMsg enum is where we hold our exacutable variants or our contract actions
#[cw_serde]
#[cfg_attr(feature = "interface", derive(ExecuteFns))]
pub enum ExecuteMsg {
    Increase,
    Decrase,
    IncreaseBy(Uint128),
}

// And last we have our MigrateMsg that is used to migrate our contract
#[cw_serde]
pub struct MigrateMsg<T> {
    pub conf: T,
    pub version: String,
}
```

now lets do the `src/error.rs`

```rust
use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

// in this enum we setup our handlers to default errors
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error(transparent)]
    ContractOverflow(#[from] OverflowError),
}
```

and now to the juicy part `src/contract.rs`

```rust
// Dependencies
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use std::{env, path::Path};
use tokio::runtime::Runtime;

// cw-orchestrator Dependencies
use cw_orch::{
    networks, Addr, Contract, CwOrcError, CwOrcExecute, CwOrcInstantiate, CwOrcQuery, CwOrcUpload,
    Daemon, Mock, TxHandler, TxResponse, Uploadable,
};

// We define our contract dependencies
pub mod error;
pub mod msgs;
pub mod state;

use error::ContractError;
use msgs::CurrentCount;
use state::{Count, COUNT};

// Contract version and name
pub const CONTRACT_NAME: &str = "mydev:CounterContract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Most of our contract will look the same to the average CosmWasm contract
// the main difference is the amount of code that we need to get started.

// In this example we are going to use Junos local testnet.

// We are going to need the following system environment variables set up for this example to work

// this first two are essential to any integration we do using cw-orchestrator
// STATE_FILE="./my-contract-state.json"
// LOCAL_MNEMONIC="clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose"

// this two are used only within this example
// CHAIN="testing"
// DEPLOYMENT_ID="my-contract-counter"

// After that is configured we can continue to our next step which is start coding!

// Using the Rust macro cw_orch::interface provided by cw-orchestrator we can define our contract entry points.
// This also generates a struct using our contract cargo name using PascalCase.
// In this example the name is CounterContract.
// This macro helps us with basic logic, keeps our contracts DRY and more important, it helps us speed our development process up
#[cw_orch::interface]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: msgs::InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    COUNT.save(deps.storage, &Count(msg.initial_value))?;

    Ok(Response::default().add_attribute("initial_value", msg.initial_value.to_string()))
}

#[cw_orch::interface]
pub fn query(deps: Deps, _env: Env, msg: msgs::QueryMsg) -> StdResult<Binary> {
    match msg {
        msgs::QueryMsg::GetCount => Ok(to_binary(&CurrentCount(COUNT.load(deps.storage)?.0))?),
    }
}

#[cw_orch::interface]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: msgs::ExecuteMsg,
) -> Result<Response, ContractError> {
    let response = match msg {
        msgs::ExecuteMsg::Increase => {
            let mut value = COUNT.load(deps.storage)?.0;
            value = value.checked_add(1u128.into())?;
            COUNT.save(deps.storage, &Count(value))?;
            Response::default().add_attribute("action", "increase")
        }
        msgs::ExecuteMsg::Decrase => {
            let mut value = COUNT.load(deps.storage)?.0;
            value = value.checked_sub(1u128.into())?;
            COUNT.save(deps.storage, &Count(value))?;
            Response::default().add_attribute("action", "decrease")
        }
        msgs::ExecuteMsg::IncreaseBy(amount) => {
            let mut value = COUNT.load(deps.storage)?.0;
            value = value.checked_add(amount.into())?;
            COUNT.save(deps.storage, &Count(value))?;
            Response::default().add_attribute("action", "increase_by")
        }
    };

    Ok(response)
}

#[cw_orch::interface]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    msg: msgs::MigrateMsg<msgs::InstantiateMsg>,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, msg.version)?;
    COUNT.save(deps.storage, &Count(msg.conf.initial_value))?;
    Ok(Response::default().add_attribute("action", "migrate"))
}

// Now that we have setup for our contract entry points, We can continue to the next step.
// This is where more of the magic of cw-orchestrator occurs
// In this case we will prepare a trait for our two scenarios Mock and Daemon
// Daemon is our production scenario, deploying to a real blockchain, be it a local testnet, a tesnet our a mainnet
// and Mock is our development scenario, used for unit testing and fine tuning our contract with speed
trait CounterWrapper<T: TxHandler> {
    fn new() -> Self;

    fn get_inner(&self) -> CounterContract<T>;

    fn upload(&self) -> Result<TxResponse<T>, CwOrcError>
    where
        T: TxHandler,
        CounterContract<T>: Uploadable<T>,
    {
        self.get_inner().upload()
    }
}

// Prepare our contract struct
struct Counter<T> {
    pub inner: T,
    pub sender: Addr,
}

// Implement Mock or development scenario
impl CounterWrapper<Mock> for Counter<CounterContract<Mock>> {
    fn new() -> Self {
        // We are going to use a genesis wallet from juno local
        // this is the way we setup our mock environment
        let mock = Mock::new(&Addr::unchecked(
            "juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y",
        ))
        .unwrap();

        // this is an example in how we can acquire the sender address configured to our operations
        // that is configured to our environment, be it a Mock or Daemon (see below)
        let sender = mock.sender();

        // We start our contract
        let contract = CounterContract(Contract::new(
            // this is used to identify our contract in the state file
            &env::var("DEPLOYMENT_ID").unwrap(),
            mock.clone(),
        ));

        Self {
            inner: contract,
            sender,
        }
    }

    fn get_inner(&self) -> CounterContract<Mock> {
        self.inner.clone()
    }
}

// Implement Daemon or deployment scenario
impl CounterWrapper<Daemon> for Counter<CounterContract<Daemon>> {
    fn new() -> Self {
        let runtime = Runtime::new().unwrap();

        // To generate a daemon we use Daemon::builder
        // which provides an easy to use interface
        // where step by step we can configure our daemon to our needs
        let res = Daemon::builder()
            // using the networks module we can provide a network
            // in this case we are using the helper parse_network that converts a string to a variant
            // but we can use networks::LOCAL_JUNO or networks::JUNO_1 for example
            .chain(networks::parse_network(&env::var("CHAIN").unwrap()))
            // here we provide the runtime to be used
            // if none is provided it will try to get one if its inside one
            .handle(runtime.handle())
            // we configure the mnemonic
            // if we dont provide an mnemonic here it will try to read it
            // from LOCAL_MNEMONIC environment variable
            // this is the one we are using in this scenario
            // but you can also use TEST_MNEMONIC and MAIN_MNEMONIC
            // depending to where you are deploying
            .mnemonic(env::var("LOCAL_MNEMONIC").unwrap())
            // and we build our daemon
            .build();

        let Some(daemon) = res.as_ref().ok() else {
            panic!("Error: {}", res.err().unwrap().to_string());
        };

        // once more here we see the sender method for adquiring our sender address configured now to our daemon
        let sender = daemon.sender();

        // We start our contract
        let contract = CounterContract(Contract::new(
            // this is used to identify our contract in the state file
            &env::var("DEPLOYMENT_ID").unwrap(),
            daemon.clone(),
        ));

        Self {
            inner: contract,
            sender,
        }
    }

    fn get_inner(&self) -> CounterContract<Daemon> {
        self.inner.clone()
    }
}

// our strategy for mock testing of the contract
fn dev() {
    let contract_counter = Counter::<CounterContract<Mock>>::new();

    let upload_res = contract_counter.upload().unwrap();
    println!("upload_res: {:#?}", upload_res);

    let init_res = contract_counter
        .inner
        .instantiate(
            &msgs::InstantiateMsg {
                initial_value: 0u128.into(),
            },
            Some(&contract_counter.sender),
            None,
        )
        .unwrap();
    println!("init_res: {:#?}", init_res);

    let exec_res = contract_counter
        .inner
        .execute(&msgs::ExecuteMsg::Increase, None)
        .unwrap();
    println!("exec_res: {:#?}", exec_res);

    let query_res = contract_counter
        .inner
        .query::<msgs::CurrentCount>(&msgs::QueryMsg::GetCount)
        .unwrap();
    println!("query_res: {:#?}", query_res);
}

// this is our strategy for local deployment
fn local() {
    let contract_counter = Counter::<CounterContract<Daemon>>::new();

    let upload_res = contract_counter.upload().unwrap();
    println!("upload_res: {:#?}", upload_res);

    let init_res = contract_counter
        .inner
        .instantiate(
            &msgs::InstantiateMsg {
                initial_value: 0u128.into(),
            },
            Some(&contract_counter.sender),
            None,
        )
        .unwrap();
    println!("init_res: {:#?}", init_res);

    let exec_res = contract_counter
        .inner
        .execute(&msgs::ExecuteMsg::Increase, None)
        .unwrap();
    println!("exec_res: {:#?}", exec_res);

    let query_res = contract_counter
        .inner
        .query::<msgs::CurrentCount>(&msgs::QueryMsg::GetCount)
        .unwrap();
    println!("query_res: {:#?}", query_res);
}

fn main() {
    pretty_env_logger::init();

    let _ = dotenvy::from_path(&Path::new(&format!("{}/.env", env!("CARGO_MANIFEST_DIR"))));

    let args = std::env::args();

    match args.last().unwrap().as_str() {
        "local" => local(),
        "dev" => dev(),
        _ => dev(),
    };
}
```

now to run our contract we can do the following command, this will run the contract in the Mock scenario

```bash
cargo run -- dev
```

And this command will deploy our command to the local juno network in our docker container

```bash
cargo run -- local
```

