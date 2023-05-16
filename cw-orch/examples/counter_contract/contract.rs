// Dependencies
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use std::{env, path::Path};
use tokio::runtime::Runtime;

// cw-orchestrator Dependencies
use cw_orch::{
    networks, Addr, CwOrcExecute, CwOrcInstantiate, CwOrcQuery, CwOrcUpload, Daemon, Mock,
    TxHandler,
};

use super::error::ContractError;
use super::msgs::{CurrentCount, QueryMsg, InstantiateMsg, ExecuteMsg, MigrateMsg};
use super::state::{Count, COUNT};

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

// Using the Rust macro cw_orch::interface_entry_point provided by cw-orchestrator we can define our contract entry points.
// This also generates a struct using our contract cargo name using PascalCase.
// In this example the name is CounterContract.
// This macro helps us with basic logic, keeps our contracts DRY and more important, it helps us speed our development process up
#[cw_orch::interface_entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg:InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    COUNT.save(deps.storage, &Count(msg.initial_value))?;

    Ok(Response::default().add_attribute("initial_value", msg.initial_value.to_string()))
}

#[cw_orch::interface_entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount => Ok(to_binary(&CurrentCount(COUNT.load(deps.storage)?.0))?),
    }
}

#[cw_orch::interface_entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let response = match msg {
        ExecuteMsg::Increase => {
            let mut value = COUNT.load(deps.storage)?.0;
            value = value.checked_add(1u128.into())?;
            COUNT.save(deps.storage, &Count(value))?;
            Response::default().add_attribute("action", "increase")
        }
        ExecuteMsg::Decrase => {
            let mut value = COUNT.load(deps.storage)?.0;
            value = value.checked_sub(1u128.into())?;
            COUNT.save(deps.storage, &Count(value))?;
            Response::default().add_attribute("action", "decrease")
        }
        ExecuteMsg::IncreaseBy(amount) => {
            let mut value = COUNT.load(deps.storage)?.0;
            value = value.checked_add(amount)?;
            COUNT.save(deps.storage, &Count(value))?;
            Response::default().add_attribute("action", "increase_by")
        }
    };

    Ok(response)
}

#[cw_orch::interface_entry_point]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    msg: MigrateMsg,
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

// our strategy for mock testing of the contract
fn dev(contract_id: String) {
    let sender = Addr::unchecked("juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y");
    // let chain = networks::parse_network(&env::var("CHAIN").unwrap());

    let mock = Mock::new(&sender).unwrap();

    let contract_counter = CounterContract::<Mock>::new(contract_id, mock);

    let upload_res = contract_counter.upload().unwrap();
    println!("upload_res: {:#?}", upload_res);

    let init_res = contract_counter
        .instantiate(
            &msgs::InstantiateMsg {
                initial_value: 0u128.into(),
            },
            Some(&sender),
            None,
        )
        .unwrap();
    println!("init_res: {:#?}", init_res);

    let exec_res = contract_counter
        .execute(&msgs::ExecuteMsg::Increase, None)
        .unwrap();
    println!("exec_res: {:#?}", exec_res);

    let query_res = contract_counter
        .query::<msgs::CurrentCount>(&msgs::QueryMsg::GetCount)
        .unwrap();
    println!("query_res: {:#?}", query_res);
}

// this is our strategy for local deployment
fn local(contract_id: String) {
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
        panic!("Error: {}", res.err().unwrap());
    };

    let contract_counter = CounterContract::<Daemon>::new(contract_id, daemon.clone());

    let upload_res = contract_counter.upload().unwrap();
    println!("upload_res: {:#?}", upload_res);

    let init_res = contract_counter
        .instantiate(
            &msgs::InstantiateMsg {
                initial_value: 0u128.into(),
            },
            Some(&contract_counter.0.get_chain().sender()),
            None,
        )
        .unwrap();
    println!("init_res: {:#?}", init_res);

    let exec_res = contract_counter
        .execute(&msgs::ExecuteMsg::Increase, None)
        .unwrap();
    println!("exec_res: {:#?}", exec_res);

    let query_res = contract_counter
        .query::<msgs::CurrentCount>(&msgs::QueryMsg::GetCount)
        .unwrap();
    println!("query_res: {:#?}", query_res);
}

fn main() {
    pretty_env_logger::init();

    let _ = dotenvy::from_path(Path::new(&format!("{}/.env", env!("CARGO_MANIFEST_DIR"))));

    let args = std::env::args();

    let contract_id = env::var("DEPLOYMENT_ID").unwrap();

    match args.last().unwrap().as_str() {
        "local" => local(contract_id),
        "dev" => dev(contract_id),
        _ => dev(contract_id),
    };
}

#[test]
fn test_contract() {
    let _ = dotenvy::from_path(Path::new(&format!("{}/.env", env!("CARGO_MANIFEST_DIR"))));
    let contract_id = env::var("DEPLOYMENT_ID").unwrap();
    let sender = Addr::unchecked("juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y");
    let mock = Mock::new(&sender).unwrap();
    let contract_counter = CounterContract::<Mock>::new(contract_id, mock);
}
