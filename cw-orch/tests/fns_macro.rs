use contract_counter::contract::ContractCounter;
use contract_counter::msg::InstantiateMsg;
use contract_counter::msg::{ExecuteMsgFns, QueryMsgFns};
use cw_orch::prelude::CwOrcUpload;
mod common;
use cosmwasm_std::Addr;
use cw_orch::prelude::{CwOrcInstantiate, Mock};

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

    let response = contract.increment().unwrap();
    assert_eq!(response.events.len(), 2)
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

    let response = contract.get_count().unwrap().count;
    assert_eq!(response, 0);
}
