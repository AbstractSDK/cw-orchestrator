use mock_contract::{ExecuteMsgFns, InstantiateMsg, MockContract, QueryMsgFns};

use cosmwasm_std::Event;

use cw_orch::prelude::CwOrcUpload;
mod common;
use cosmwasm_std::Addr;
use cw_orch::prelude::{CwOrcInstantiate, Mock};

#[test]
fn test_execute() {
    let contract = MockContract::new(
        "test:mock_contract",
        Mock::new(&Addr::unchecked("Ghazshag")),
    );
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg {}, None, None)
        .unwrap();

    let response = contract.first_message().unwrap();

    response.has_event(
        &Event::new("wasm")
            .add_attribute("_contract_addr", "contract0")
            .add_attribute("action", "first message passed"),
    );

    contract.second_message("".to_string(), &[]).unwrap_err();
}

#[test]
fn test_query() {
    let contract = MockContract::new(
        "test:mock_contract",
        Mock::new(&Addr::unchecked("Ghazshag")),
    );
    contract.upload().unwrap();

    contract
        .instantiate(&InstantiateMsg {}, None, None)
        .unwrap();

    let response = contract.first_query().unwrap();
    assert_eq!(response, "first query passed");

    contract.second_query("".to_string()).unwrap_err();
}
