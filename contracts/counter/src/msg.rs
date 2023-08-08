use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_orch::cli;

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

// ANCHOR: exec_msg
#[cw_serde]
#[cli]
#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))] // Function generation
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
}
// ANCHOR_END: exec_msg

// ANCHOR: query_msg
#[cw_serde]
#[cli]
#[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))] // Function generation
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// Custom response for the query
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}
// ANCHOR_END: query_msg

#[cw_serde]
pub struct MigrateMsg {
    pub t: String,
}
