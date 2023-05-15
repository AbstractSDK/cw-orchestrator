use mock_contract::{ExecuteMsg, InstantiateMsg, MigrateMsg, MockContract, QueryMsg};

use cosmwasm_std::Event;
use cw_orch::daemon::Daemon;
use cw_orch::prelude::ContractInstance;
use cw_orch::prelude::CwOrcExecute;
use cw_orch::prelude::CwOrcMigrate;
use cw_orch::prelude::CwOrcQuery;
use tokio::runtime::Runtime;

use cw_orch::prelude::CwOrcUpload;
mod common;
use cosmwasm_std::Addr;
use cw_orch::prelude::{CwOrcInstantiate, Mock};

#[test]
fn test_instantiate() {
    let contract = MockContract::new(
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
        .execute(&ExecuteMsg::SecondMessage { t: "".to_string() }, None)
        .unwrap_err();
}

#[test]
fn test_query() {
    let contract = MockContract::new(
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
    let contract = MockContract::new("test:mock_contract", Mock::new(&admin).unwrap());
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

#[test]
#[cfg(feature = "node-tests")]
fn daemon_test() {
    use cw_orch::prelude::networks;

    let runtime = tokio::runtime::Runtime::new().unwrap();

    let daemon = Daemon::builder()
        .chain(networks::LOCAL_JUNO)
        .handle(runtime.handle())
        .build()
        .unwrap();

    let contract = mock_contract::MockContract::new("test:mock_contract", daemon.clone());
    contract.upload().unwrap();

    contract
        .instantiate(
            &InstantiateMsg {},
            Some(&daemon.sender.address().unwrap()),
            None,
        )
        .unwrap();

    let response = contract
        .execute(&ExecuteMsg::FirstMessage {}, None)
        .unwrap();
    assert_eq!(
        response.get_events("wasm")[0].get_first_attribute_value("action"),
        Some("first message passed".to_string())
    );

    contract
        .execute(&ExecuteMsg::SecondMessage { t: "".to_string() }, None)
        .unwrap_err();

    let response: String = contract.query(&QueryMsg::FirstQuery {}).unwrap();
    assert_eq!(response, "first query passed");

    contract
        .query::<String>(&QueryMsg::SecondQuery { t: "".to_string() })
        .unwrap_err();

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
