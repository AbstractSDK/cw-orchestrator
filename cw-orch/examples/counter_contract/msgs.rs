use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
use cw_orch_fns_derive::{ExecuteFns, QueryFns};

// This is the message that we are going to use to start our contract
#[cw_serde]
pub struct InstantiateMsg {
    // our value to set in our state
    pub initial_value: Uint128,
}

// ExecuteMsg enum is where we hold our exacutable variants or our contract actions
#[cw_serde]
#[derive(ExecuteFns)]
pub enum ExecuteMsg {
    Increase,
    Decrase,
    IncreaseBy(Uint128),
}

// The QueryMsg enum holds our variants that we are going to use to get information out of our contract
#[cw_serde]
#[derive(QueryResponses, QueryFns)]
pub enum QueryMsg {
    #[returns(CurrentCount)]
    GetCount {},
}

// This is our response to our get_count query
#[cw_serde]
pub struct CurrentCount(pub Uint128);

// And last we have our MigrateMsg that is used to migrate our contract
#[cw_serde]
pub struct MigrateMsg {
    pub version: String,
}
