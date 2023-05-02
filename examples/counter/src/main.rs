pub mod error;
pub mod msgs;
pub mod state;

use error::ContractError;
use msgs::CurrentCount;
use state::{Count, COUNT};

use std::env;

use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use cw2::set_contract_version;

use cw_orch::{
    networks, Addr, Contract, CwOrcError, CwOrcExecute, CwOrcInstantiate, CwOrcQuery, CwOrcUpload,
    Daemon, Mock, TxHandler, TxResponse,
};

use dotenvy::dotenv;
use tokio::runtime::Runtime;

pub const CONTRACT_NAME: &str = "myDev:my-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

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

// #[cw_orch::interface]
// pub fn migrate(
//     deps: DepsMut,
//     env: Env,
//     msg: cw20_base::msg::MigrateMsg,
// ) -> Result<Response, ContractError> {
//     Ok(cw20_base::contract::migrate(deps, env, msg)?)
// }

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

// Prepare our trait for multiple scenarios
trait MyContractWrapper<T: TxHandler> {
    fn new() -> Self;
    fn upload(&self) -> Result<TxResponse<T>, CwOrcError>;
}

// Prepare our struct
struct MyContract<T> {
    pub inner: T,
    pub sender: Addr,
}

// Implement Mock scenario
impl MyContractWrapper<Mock> for MyContract<QuickStart<Mock>> {
    fn new() -> Self {
        let daemon_mock = Mock::new(&Addr::unchecked(
            "juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y",
        ))
        .unwrap();

        let sender = daemon_mock.sender();

        let contract = QuickStart(Contract::new(
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
impl MyContractWrapper<Daemon> for MyContract<QuickStart<Daemon>> {
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

        let contract = QuickStart(Contract::new(
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
    dotenv().ok();

    let args = std::env::args();

    println!("{:#?}", args.last());

    let my_contract = MyContract::<QuickStart<Mock>>::new();

    let upload_res = my_contract.upload().unwrap();
    println!("{:#?}", upload_res);

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
    println!("{:#?}", init_res);

    let exec_res = my_contract
        .inner
        .execute(&msgs::ExecuteMsg::Increase, None)
        .unwrap();
    println!("{:#?}", exec_res);

    let query_res = my_contract
        .inner
        .query::<msgs::CurrentCount>(&msgs::QueryMsg::GetCount)
        .unwrap();

    println!("{:#?}", query_res);
}
