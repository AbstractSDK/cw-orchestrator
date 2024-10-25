use cosmwasm_std::coins;
use counter_contract::{
    msg::InstantiateMsg, CounterContract, CounterExecuteMsgFns, CounterQueryMsgFns,
};

use cw_orch::prelude::CallAs;
use cw_orch::prelude::*;
use cw_orch_osmosis_test_tube::OsmosisTestTube;

pub fn main() {
    // ANCHOR: osmosis_test_tube_creation
    let mut chain = OsmosisTestTube::new(coins(1_000_000_000_000, "uosmo"));
    // ANCHOR_END: osmosis_test_tube_creation

    // ANCHOR: osmosis_test_tube_usage
    let contract_counter = CounterContract::new(chain.clone());

    let upload_res = contract_counter.upload();
    assert!(upload_res.is_ok());

    let init_res = contract_counter.instantiate(&InstantiateMsg { count: 0 }, None, &[]);
    assert!(init_res.is_ok());
    // ANCHOR_END: osmosis_test_tube_usage

    let exec_res = contract_counter.increment();
    assert!(exec_res.is_ok());

    let sender = chain
        .init_account(coins(1_000_000_000_000, "uosmo"))
        .unwrap();

    let exec_call_as = contract_counter.call_as(&sender).increment();
    assert!(exec_call_as.is_ok());

    let query_res = contract_counter.get_count();
    assert!(query_res.is_ok());
}

// This is used for documentation only
// This is actually only used to avoid having the `mut` keyword inside the mock_usage anchor (only necessary for set_sender)
pub fn customize() {
    let mut chain = OsmosisTestTube::new(coins(1_000_000_000_000, "uosmo"));

    let mut contract_counter = CounterContract::new(chain.clone());

    // ANCHOR: osmosis_test_tube_customization
    let new_sender = chain.init_account(coins(100_000, "ujunox")).unwrap();

    // Reuploads as the new sender
    contract_counter.call_as(&new_sender).upload().unwrap();

    // Here the contract_counter sender is again `sender`

    // Sets the new_sender as the definite sender
    contract_counter.set_sender(&new_sender);

    // From now on the contract_counter sender is `new_sender`
    // ANCHOR_END: osmosis_test_tube_customization

    // ANCHOR: deep_osmosis_test_tube_customization
    chain.app.borrow_mut().increase_time(150);
    // ANCHOR_END: deep_osmosis_test_tube_customization
}
