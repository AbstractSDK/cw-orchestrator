use cosmwasm_std::coins;
use counter_contract::{
    contract::CounterContract,
    msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg},
};
use cw_orch::prelude::OsmosisTestTube;
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
