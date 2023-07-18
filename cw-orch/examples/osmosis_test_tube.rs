// use cosmwasm_std::coins;
// use counter_contract::{
//     contract::CounterContract,
//     msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg},
// };
// use cw_orch::prelude::OsmosisTestTube;
// use cw_orch::prelude::{CwOrchExecute, CwOrchInstantiate, CwOrchQuery, CwOrchUpload};

pub fn main() {
    // let chain = OsmosisTestTube::new(coins(1_000_000_000_000, "uosmo"));

    // let contract_counter = CounterContract::new("mock:contract_counter", chain);

    // let upload_res = contract_counter.upload();
    // assert!(upload_res.is_ok());

    // let init_res = contract_counter.instantiate(&InstantiateMsg { count: 0 }, None, None);
    // assert!(init_res.is_ok());

    // let exec_res = contract_counter.execute(&ExecuteMsg::Increment {}, None);
    // assert!(exec_res.is_ok());

    // let query_res = contract_counter.query::<GetCountResponse>(&QueryMsg::GetCount {});
    // assert!(query_res.is_ok());
}
