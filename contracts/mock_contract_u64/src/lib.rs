use mock_contract::{InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg};

use cosmwasm_std::to_binary;
use cosmwasm_std::Binary;
use cosmwasm_std::Deps;
use cosmwasm_std::DepsMut;
use cosmwasm_std::MessageInfo;
use cosmwasm_std::Response;
use cosmwasm_std::StdError;
use cosmwasm_std::StdResult;
use cosmwasm_std::{entry_point, Env};


#[cfg_attr(feature = "export", entry_point)]
#[cfg_attr(feature = "cw-orch", cw_orch::interface_entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
#[cfg_attr(feature = "cw-orch", cw_orch::interface_entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg<u64>,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::FirstMessage {} => {
            Ok(Response::new().add_attribute("action", "first message passed"))
        }
        ExecuteMsg::SecondMessage { t: _ } => Err(StdError::generic_err("Second Message Failed")),
        ExecuteMsg::ThirdMessage { .. } => Ok(Response::new().add_attribute("action", "third message passed"))
    }
}

#[entry_point]
#[cfg_attr(feature = "cw-orch", cw_orch::interface_entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::FirstQuery {} => to_binary("first query passed"),
        QueryMsg::SecondQuery { .. } => Err(StdError::generic_err("Query not available")),
    }
}

#[entry_point]
#[cfg_attr(feature = "cw-orch", cw_orch::interface_entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> StdResult<Response> {
    if msg.t.eq("success") {
        Ok(Response::new())
    } else {
        Err(StdError::generic_err(
            "migrate endpoint reached but no test implementation",
        ))
    }
}
