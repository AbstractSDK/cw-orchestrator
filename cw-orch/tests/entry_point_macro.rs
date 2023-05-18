use contract_counter::{
    contract::ContractCounter,
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
};

use cosmwasm_std::Event;

use cw_orch::prelude::{
    ContractInstance, CwOrcExecute, CwOrcInstantiate, CwOrcMigrate, CwOrcQuery, CwOrcUpload, Mock,
};

mod common;
use cosmwasm_std::Addr;

#[test]
fn test_instantiate() {
    let contract = ContractCounter::new(
        "test:contract_counter",
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
        "test:contract_counter",
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
        "test:contract_counter",
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
    let contract = ContractCounter::new("test:contract_counter", Mock::new(&admin).unwrap());
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
    use crate::common::Id;
    use cw_orch::environment::TxHandler;
    use cw_orch::prelude::networks;
    use cw_orch::prelude::Daemon;

    let runtime = tokio::runtime::Runtime::new().unwrap();

    let daemon = Daemon::builder()
        .chain(networks::LOCAL_JUNO)
        .handle(runtime.handle())
        .build()
        .unwrap();

    let contract = contract_counter::contract::ContractCounter::new(
        format!("test:contract_counter:{}", Id::new()),
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
