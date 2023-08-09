use cosmwasm_std::coins;
use counter_contract::{
    contract::CounterContract,
    msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg},
};
use cw_orch::prelude::{CallAs, OsmosisTestTube};
use cw_orch::prelude::{CwOrchExecute, CwOrchInstantiate, CwOrchQuery, CwOrchUpload};

pub fn main() {
    // ANCHOR: osmosis_test_tube_creation
    let chain = OsmosisTestTube::new(coins(1_000_000_000_000, "uosmo"));
    // ANCHOR_END: osmosis_test_tube_creation

    // ANCHOR: osmosis_test_tube_usage
    let contract_counter = CounterContract::new("osmosis_test_tube:contract_counter", chain);

    let upload_res = contract_counter.upload();
    assert!(upload_res.is_ok());

    let init_res = contract_counter.instantiate(&InstantiateMsg { count: 0 }, None, None);
    assert!(init_res.is_ok());
    // ANCHOR_END: osmosis_test_tube_usage

    let exec_res = contract_counter.execute(&ExecuteMsg::Increment {}, None);
    assert!(exec_res.is_ok());

    let query_res = contract_counter.query::<GetCountResponse>(&QueryMsg::GetCount {});
    assert!(query_res.is_ok());
}

// This is used for documentation only
// This is actually only used to avoid having the `mut` keyword inside the mock_usage anchor (only necessary for set_sender)
pub fn customize() {
    let chain = OsmosisTestTube::new(coins(1_000_000_000_000, "uosmo"));

    let mut contract_counter = CounterContract::new("mock:contract_counter", chain.clone());

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
