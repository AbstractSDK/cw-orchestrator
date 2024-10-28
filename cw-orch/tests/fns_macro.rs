use mock_contract::{ExecuteMsgFns, InstantiateMsg, MockContract, QueryMsgFns};

use cw_orch::prelude::{CwOrchInstantiate, Mock};
use cw_orch::prelude::{CwOrchUpload, IndexResponse};

#[test]
fn test_execute() {
    let contract = MockContract::new("test:mock_contract", Mock::new("Ghazshag"));
    contract.upload().unwrap();

    contract.instantiate(&InstantiateMsg {}, None, &[]).unwrap();

    let response = contract.first_message().unwrap();
    assert!(response
        .event_attr_value("wasm", "_contract_address")
        .is_ok(),);
    assert_eq!(
        response.event_attr_value("wasm", "action").unwrap(),
        "first message passed"
    );

    contract.second_message("".to_string(), &[]).unwrap_err();
}

#[test]
fn test_query() {
    let contract = MockContract::new("test:mock_contract", Mock::new("Ghazshag"));
    contract.upload().unwrap();

    contract.instantiate(&InstantiateMsg {}, None, &[]).unwrap();

    let response = contract.first_query().unwrap();
    assert_eq!(response, "first query passed");

    contract.second_query("".to_string()).unwrap_err();
}
