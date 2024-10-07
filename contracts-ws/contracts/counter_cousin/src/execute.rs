use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{error::*, state::*};

pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "increment"))
}

pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("action", "reset"))
}

pub fn set_cousin(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cousin: String,
) -> Result<Response, ContractError> {
    let cousin_addr = deps.api.addr_validate(&cousin)?;
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }

    crate::CounterContract::save(deps.branch(), &env, "cousin", cousin_addr.clone());

    Ok(Response::new().add_attribute("action", "set_cousin"))
}
