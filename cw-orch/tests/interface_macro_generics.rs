use cw_orch::{environment::CwEnv, interface, prelude::*};
use mock_contract::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use cosmwasm_std::Event;
use cw_orch::prelude::Mock;

#[interface(InstantiateMsg, ExecuteMsg<T>, QueryMsg, MigrateMsg, id = "test:mock_contract")]
pub struct MockContract;

impl<Chain, T> Uploadable for MockContract<Chain, T> {
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
    let contract = MockContract::<_, u64>::new(Mock::new("Ghazshag"));

    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg {}, None, None)
        .unwrap();
}

#[test]
fn test_execute() {
    let contract = MockContract::new(Mock::new("Ghazshag"));

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
    let contract = MockContract::<_, u64>::new(Mock::new("Ghazshag"));

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
    let chain = Mock::new("Ghazshag");
    let contract = MockContract::<_, u64>::new(chain.clone());
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg {}, Some(&chain.sender()), None)
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
