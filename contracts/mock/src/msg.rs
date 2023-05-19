use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
#[cfg_attr(feature = "cw-orch", derive(cw_orch::ExecuteFns))]
pub enum ExecuteMsg<T = i32> {
    Increment {},
    Reset { count: T },
}

#[cw_serde]
#[cfg_attr(feature = "cw-orch", derive(cw_orch::QueryFns))]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}

#[cw_serde]
pub struct MigrateMsg {
    pub t: String,
}
