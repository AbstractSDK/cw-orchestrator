use cw_orch::{interface, prelude::*};
use mock_contract::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use cw_orch::prelude::Mock;

#[interface(InstantiateMsg, ExecuteMsg<T>, QueryMsg, MigrateMsg, id = "test:mock_contract")]
pub struct MockContract;

impl<Chain, T> Uploadable for MockContract<Chain, T> {
    fn wrapper() -> <Mock as TxHandler>::ContractSource {
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
    let contract = MockContract::<_, u64>::new(Mock::new("Ghazshag"));

    contract.upload().unwrap();

    contract.instantiate(&InstantiateMsg {}, None, &[]).unwrap();
}

#[test]
fn test_execute() {
    let contract = MockContract::new(Mock::new("Ghazshag"));

    contract.upload().unwrap();

    contract.instantiate(&InstantiateMsg {}, None, &[]).unwrap();

    let response = contract.execute(&ExecuteMsg::FirstMessage {}, &[]).unwrap();
    assert!(response
        .event_attr_value("wasm", "_contract_address")
        .is_ok(),);
    assert_eq!(
        response.event_attr_value("wasm", "action").unwrap(),
        "first message passed"
    );

    contract
        .execute(&ExecuteMsg::SecondMessage { t: 46u64 }, &[])
        .unwrap_err();

    // This call should not error, the types are good now
    contract
        .execute(&ExecuteMsg::ThirdMessage { t: 67u64 }, &[])
        .unwrap();
}

#[test]
fn test_query() {
    let contract = MockContract::<_, u64>::new(Mock::new("Ghazshag"));

    contract.upload().unwrap();

    contract.instantiate(&InstantiateMsg {}, None, &[]).unwrap();

    let response: String = contract.query(&QueryMsg::FirstQuery {}).unwrap();
    assert_eq!(response, "first query passed");

    contract
        .query::<String>(&QueryMsg::SecondQuery { t: "".to_string() })
        .unwrap_err();
}

#[test]
fn test_migrate() {
    let chain = Mock::new("Ghazshag");
    let contract = MockContract::<_, u64>::new(chain.clone());
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg {}, Some(&chain.sender_addr()), &[])
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
