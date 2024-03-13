#![allow(unused)]
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    to_json_binary, Binary, CustomMsg, CustomQuery, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, Uint128,
};

#[cw_serde]
pub struct A;

impl CustomMsg for A {}
impl CustomQuery for A {}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    FirstMessage {},
}

#[cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum QueryMsg {
    #[returns(String)]
    FirstQuery {},
    #[returns(String)]
    SecondQuery { t: String },
}

#[cw_serde]
pub struct MigrateMsg {
    pub t: String,
}

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response<A>> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}

pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response<A>> {
    match msg {
        ExecuteMsg::FirstMessage {} => {
            Ok(Response::new().add_attribute("action", "first message passed"))
        }
    }
}

pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::FirstQuery {} => to_json_binary("first query passed"),
        QueryMsg::SecondQuery { .. } => Err(StdError::generic_err("Query not available")),
    }
}

pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> StdResult<Response<A>> {
    if msg.t.eq("success") {
        Ok(Response::new())
    } else {
        Err(StdError::generic_err(
            "migrate endpoint reached but no test implementation",
        ))
    }
}
