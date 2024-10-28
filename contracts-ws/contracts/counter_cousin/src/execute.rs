use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw_orch::prelude::*;

use crate::CounterContract;
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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cousin: String,
) -> Result<Response, ContractError> {
    assert_owner(deps.as_ref(), &info)?;

    let cousin_addr = deps.api.addr_validate(&cousin)?;

    CounterContract::load(deps, &env, "cousin").set_address(&cousin_addr);

    Ok(Response::new().add_attribute("action", "set_cousin"))
}
