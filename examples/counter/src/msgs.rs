use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub initial_value: Uint128,
}

#[cw_serde]
#[derive(QueryResponses)]
#[cfg_attr(feature = "interface", derive(QueryFns))]
pub enum QueryMsg {
    #[returns(CurrentCount)]
    GetCount,
}

#[cw_serde]
pub struct CurrentCount(pub Uint128);

#[cw_serde]
#[cfg_attr(feature = "interface", derive(ExecuteFns))]
pub enum ExecuteMsg {
    Increase,
    Decrase,
    IncreaseBy(Uint128),
}

#[cw_serde]
pub struct MigrateMsg<T> {
    pub conf: T,
    pub version: String,
}
