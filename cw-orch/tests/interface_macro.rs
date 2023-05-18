mod common;

use cw_orch::contract::Contract;
use cw_orch::environment::CwEnv;
use cw_orch::environment::TxHandler;
use cw_orch::prelude::ContractWrapper;
use cw_orch::prelude::Uploadable;
use cw_orch::prelude::WasmPath;

use contract_counter::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use cosmwasm_std::Event;
use cw_orch::prelude::ContractInstance;
use cw_orch::prelude::CwOrcExecute;
use cw_orch::prelude::CwOrcMigrate;
use cw_orch::prelude::CwOrcQuery;
use cw_orch::prelude::CwOrcUpload;
use cw_orch::prelude::Daemon;
use cw_orch::prelude::{CwOrcInstantiate, Mock};

use cosmwasm_std::Addr;
use cw_orch::interface;
const MOCK_CONTRACT_WASM: &str = "../artifacts/mock_contract.wasm";

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct ContractCounter;

impl<Chain: CwEnv> ContractCounter<Chain> {
    pub fn new(id: &str, chain: Chain) -> Self {
        Self(Contract::new(id, chain))
    }
}

impl<Chain: CwEnv> Uploadable for ContractCounter<Chain> {
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
                contract_counter::contract::execute,
                contract_counter::contract::instantiate,
                contract_counter::contract::query,
            )
            .with_migrate(contract_counter::contract::migrate),
        )
    }
}

#[test]
fn test_instantiate() {
    let contract = ContractCounter::new(
        "test:mock_contract",
        Mock::new(&Addr::unchecked("Ghazshag")).unwrap(),
    );
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg { count: 0 }, None, None)
        .unwrap();
}

#[test]
fn test_execute() {
    let contract = ContractCounter::new(
        "test:mock_contract",
        Mock::new(&Addr::unchecked("Ghazshag")).unwrap(),
    );
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg { count: 0 }, None, None)
        .unwrap();

    let response = contract.execute(&ExecuteMsg::Increment {}, None).unwrap();
    response.has_event(
        &Event::new("wasm")
            .add_attribute("_contract_addr", "contract0")
            .add_attribute("action", "first message passed"),
    );

    contract
        .execute(&ExecuteMsg::Reset { count: 0 }, None)
        .unwrap_err();
}

#[test]
fn test_query() {
    let contract = ContractCounter::new(
        "test:mock_contract",
        Mock::new(&Addr::unchecked("Ghazshag")).unwrap(),
    );
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg { count: 0 }, None, None)
        .unwrap();

    let response: String = contract.query(&QueryMsg::GetCount {}).unwrap();
    assert_eq!(response, "first query passed");
}

#[test]
fn test_migrate() {
    let admin = Addr::unchecked("Ghazshag");
    let contract = ContractCounter::new("test:mock_contract", Mock::new(&admin).unwrap());
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg { count: 0 }, Some(&admin), None)
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

#[test]
#[cfg(feature = "node-tests")]
#[serial_test::serial]
fn daemon_test() {
    use cw_orch::prelude::networks;

    use crate::common::Id;

    let runtime = tokio::runtime::Runtime::new().unwrap();

    let daemon = Daemon::builder()
        .chain(networks::LOCAL_JUNO)
        .handle(runtime.handle())
        .build()
        .unwrap();

    let contract = contract_counter::contract::ContractCounter::new(
        format!("test:mock_contract:{}", Id::new()),
        daemon.clone(),
    );
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg { count: 0 }, Some(&daemon.sender()), None)
        .unwrap();

    let response = contract.execute(&ExecuteMsg::Increment {}, None).unwrap();
    assert_eq!(
        response.get_events("wasm")[0].get_first_attribute_value("action"),
        Some("first message passed".to_string())
    );

    contract
        .execute(&ExecuteMsg::Reset { count: 0 }, None)
        .unwrap_err();

    let response: String = contract.query(&QueryMsg::GetCount {}).unwrap();
    assert_eq!(response, "first query passed");

    contract
        .migrate(
            &MigrateMsg {
                t: "error".to_string(),
            },
            contract.code_id().unwrap(),
        )
        .unwrap_err();
    contract
        .migrate(
            &MigrateMsg {
                t: "success".to_string(),
            },
            contract.code_id().unwrap(),
        )
        .unwrap();
}
