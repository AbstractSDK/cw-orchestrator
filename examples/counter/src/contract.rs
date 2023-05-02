// Dependencies
use std::{env, path::Path};
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use tokio::runtime::Runtime;

// cw-orchestrator Dependencies
use cw_orch::{
    networks, Addr, Contract, CwOrcError, CwOrcExecute, CwOrcInstantiate, CwOrcQuery, CwOrcUpload,
    Daemon, Mock, TxHandler, TxResponse,
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
// STATE_FILE= "./my-contract-state.json"
// LOCAL_MNEMONIC= "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose"

// this two are used only within this example
// CHAIN= "testing"
// DEPLOYMENT_ID= "my-contract-counter"

// After that is configured we can continue to our next step which is start coding!

// Using cw_orch::interface macro we can define our entry points.
// this also generates a struct using our contract cargo name using PascalCase
// in this example the name is CounterContract
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
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    msg: msgs::MigrateMsg<msgs::InstantiateMsg>,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, msg.version)?;
    COUNT.save(deps.storage, &Count(msg.conf.initial_value))?;
    Ok(Response::default().add_attribute("action", "migrate"))
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

// Now that we have setup our contract entry points above, We can continue to the next step.
// In this case we will prepare a trait for our two scenarios Mock and Daemon
// Daemon is our production scenario, deploying to a blockchain, be it a local testnet, a tesnet our a mainnet
// and Mock is our development scenario, used for unit testing and fine tuning our contract
trait CounterWrapper<T: TxHandler> {
    fn new() -> Self;
    fn upload(&self) -> Result<TxResponse<T>, CwOrcError>;
}

// Prepare our contract struct
struct Counter<T> {
    pub inner: T,
    pub sender: Addr,
}

// Implement Mock scenario
impl CounterWrapper<Mock> for Counter<CounterContract<Mock>> {
    fn new() -> Self {
        let daemon_mock = Mock::new(&Addr::unchecked(
            "juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y",
        ))
        .unwrap();

        let sender = daemon_mock.sender();

        let contract = CounterContract(Contract::new(
            &env::var("DEPLOYMENT_ID").unwrap(),
            daemon_mock.clone(),
        ));

        Self {
            inner: contract,
            sender,
        }
    }

    fn upload(&self) -> Result<TxResponse<Mock>, CwOrcError> {
        self.inner.upload()
    }
}

// Implement Daemon or real scenario
impl CounterWrapper<Daemon> for Counter<CounterContract<Daemon>> {
    fn new() -> Self {
        let runtime = Runtime::new().unwrap();

        let res = Daemon::builder()
            .chain(networks::parse_network(&env::var("CHAIN").unwrap()))
            .handle(runtime.handle())
            .mnemonic(env::var("LOCAL_MNEMONIC").unwrap())
            .build();

        let Some(daemon) = res.as_ref().ok() else {
            panic!("Error: {}", res.err().unwrap().to_string());
        };

        let sender = daemon.sender.address().unwrap();

        let contract = CounterContract(Contract::new(
            &env::var("DEPLOYMENT_ID").unwrap(),
            daemon.clone(),
        ));

        Self {
            inner: contract,
            sender,
        }
    }

    fn upload(&self) -> Result<TxResponse<Daemon>, CwOrcError> {
        self.inner.upload()
    }
}

fn main() {
    pretty_env_logger::init();

    let _ = dotenvy::from_path(&Path::new(&format!("{}/.env", env!("CARGO_MANIFEST_DIR"))));

    let args = std::env::args();

    println!("{:#?}", args.last());

    let my_contract = Counter::<CounterContract<Mock>>::new();

    let upload_res = my_contract.upload().unwrap();
    println!("upload_res: {:#?}", upload_res);

    let init_res = my_contract
        .inner
        .instantiate(
            &msgs::InstantiateMsg {
                initial_value: 0u128.into(),
            },
            Some(&my_contract.sender),
            None,
        )
        .unwrap();
    println!("init_res: {:#?}", init_res);

    let exec_res = my_contract
        .inner
        .execute(&msgs::ExecuteMsg::Increase, None)
        .unwrap();
    println!("exec_res: {:#?}", exec_res);

    let query_res = my_contract
        .inner
        .query::<msgs::CurrentCount>(&msgs::QueryMsg::GetCount)
        .unwrap();

    println!("query_res: {:#?}", query_res);
}
