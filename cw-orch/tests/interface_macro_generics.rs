use cw_orch::contract::Contract;
use cw_orch::daemon::sync::core::Daemon;
use cw_orch::environment::CwEnv;
use cw_orch::interface;
use cw_orch::prelude::ContractWrapper;
use cw_orch::prelude::TxHandler;
use cw_orch::prelude::Uploadable;
use cw_orch::prelude::WasmPath;
use mock_contract::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use cosmwasm_std::Event;
use cw_orch::prelude::ContractInstance;
use cw_orch::prelude::CwOrcExecute;
use cw_orch::prelude::CwOrcMigrate;
use cw_orch::prelude::CwOrcQuery;

use cw_orch::prelude::CwOrcUpload;
mod common;
use cosmwasm_std::Addr;
use cw_orch::prelude::{CwOrcInstantiate, Mock};

const MOCK_CONTRACT_WASM: &str = "../artifacts/mock_contract.wasm";

#[interface(InstantiateMsg, ExecuteMsg<T>, QueryMsg, MigrateMsg)]
pub struct MockContract;

impl<Chain: CwEnv, T> MockContract<Chain, T> {
    pub fn new(id: &str, chain: Chain) -> Self {
        Self(
            Contract::new(id, chain),
            std::marker::PhantomData::default(),
        )
    }
}

impl<Chain: CwEnv, T> Uploadable for MockContract<Chain, T> {
    fn wasm(&self) -> <Daemon as TxHandler>::ContractSource {
        // create contract base configuration
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let wasm_path = format!("{}/{}", crate_path, MOCK_CONTRACT_WASM);
        log::info!("Using wasm path {}", wasm_path);
        WasmPath::new(wasm_path).unwrap()
    }
    fn wrapper(&self) -> <Mock as TxHandler>::ContractSource {
        Box::new(
            ContractWrapper::new_with_empty(
                mock_contract_u64::execute,
                mock_contract_u64::instantiate,
                mock_contract_u64::query,
            )
            .with_migrate(mock_contract::migrate),
        )
    }
}

#[test]
fn test_instantiate() {
    let contract = MockContract::<_, u64>::new(
        "test:mock_contract",
        Mock::new(&Addr::unchecked("Ghazshag")).unwrap(),
    );
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg {}, None, None)
        .unwrap();
}

#[test]
fn test_execute() {
    let contract = MockContract::new(
        "test:mock_contract",
        Mock::new(&Addr::unchecked("Ghazshag")).unwrap(),
    );
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg {}, None, None)
        .unwrap();

    let response = contract
        .execute(&ExecuteMsg::FirstMessage {}, None)
        .unwrap();

    response.has_event(
        &Event::new("wasm")
            .add_attribute("_contract_addr", "contract0")
            .add_attribute("action", "first message passed"),
    );

    contract
        .execute(&ExecuteMsg::SecondMessage { t: 46u64 }, None)
        .unwrap_err();

    // This call should not error, the types are good now
    contract
        .execute(&ExecuteMsg::ThirdMessage { t: 67u64 }, None)
        .unwrap();
}

#[test]
fn test_query() {
    let contract = MockContract::<_, u64>::new(
        "test:mock_contract",
        Mock::new(&Addr::unchecked("Ghazshag")).unwrap(),
    );
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg {}, None, None)
        .unwrap();

    let response: String = contract.query(&QueryMsg::FirstQuery {}).unwrap();
    assert_eq!(response, "first query passed");

    contract
        .query::<String>(&QueryMsg::SecondQuery { t: "".to_string() })
        .unwrap_err();
}

#[test]
fn test_migrate() {
    let admin = Addr::unchecked("Ghazshag");
    let contract = MockContract::<_, u64>::new("test:mock_contract", Mock::new(&admin).unwrap());
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg {}, Some(&admin), None)
        .unwrap();

    contract
        .migrate(
            &MigrateMsg {
                t: "error".to_string(),
            },
            contract.code_id().unwrap(),
        )
        .unwrap_err();
    let response = contract
        .migrate(
            &MigrateMsg {
                t: "success".to_string(),
            },
            contract.code_id().unwrap(),
        )
        .unwrap();
    assert_eq!(response.events.len(), 1);
}
