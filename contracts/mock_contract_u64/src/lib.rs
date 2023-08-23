use mock_contract::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

#[cfg_attr(feature = "export", entry_point)]
#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[entry_point]
#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg<u64>,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::FirstMessage {} => {
            Ok(Response::new().add_attribute("action", "first message passed"))
        }
        ExecuteMsg::SecondMessage { t: _ } => Err(StdError::generic_err("Second Message Failed")),
        ExecuteMsg::ThirdMessage { .. } => {
            Ok(Response::new().add_attribute("action", "third message passed"))
        }
        ExecuteMsg::FourthMessage => {
            Ok(Response::new().add_attribute("action", "fourth message passed"))
        }
        ExecuteMsg::FifthMessage => {
            if info.funds.is_empty() {
                return Err(StdError::generic_err("Coins missing"));
            }
            Ok(Response::new().add_attribute("action", "fourth message passed"))
        }
    }
}

#[entry_point]
#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::FirstQuery {} => to_binary("first query passed"),
        QueryMsg::SecondQuery { .. } => Err(StdError::generic_err("Query not available")),
        QueryMsg::ThirdQuery => to_binary("third query passed"),
    }
}

#[entry_point]
#[cfg_attr(feature = "interface", cw_orch::interface_entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> StdResult<Response> {
    if msg.t.eq("success") {
        Ok(Response::new())
    } else {
        Err(StdError::generic_err(
            "migrate endpoint reached but no test implementation",
        ))
    }
}
