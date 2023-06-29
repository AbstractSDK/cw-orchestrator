mod custom_resp;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))]
pub enum ExecuteMsg<T = String> {
    FirstMessage {},
    #[cfg_attr(feature = "interface", payable)]
    SecondMessage {
        /// test doc-comment
        t: T,
    },
    /// test doc-comment
    ThirdMessage {
        /// test doc-comment
        t: T,
    },
}

#[cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(String)]
    /// test-doc-comment
    FirstQuery {},
    #[returns(String)]
    SecondQuery {
        /// test doc-comment
        t: String,
    },
}

#[cw_serde]
pub struct MigrateMsg {
    pub t: String,
}

#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::FirstMessage {} => {
            Ok(Response::new().add_attribute("action", "first message passed"))
        }
        ExecuteMsg::SecondMessage { t: _ } => Err(StdError::generic_err("Second Message Failed")),
        ExecuteMsg::ThirdMessage { .. } => {
            Ok(Response::new().add_attribute("action", "third message passed"))
        }
    }
}

#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::FirstQuery {} => to_binary("first query passed"),
        QueryMsg::SecondQuery { .. } => Err(StdError::generic_err("Query not available")),
    }
}

#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> StdResult<Response> {
    if msg.t.eq("success") {
        Ok(Response::new())
    } else {
        Err(StdError::generic_err(
            "migrate endpoint reached but no test implementation",
        ))
    }
}
