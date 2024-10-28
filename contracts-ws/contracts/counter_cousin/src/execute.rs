use crate::msg::ExecuteMsgFns;
use crate::CounterContract;
use crate::{error::*, state::*};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("action", "increment"))
}

pub fn increment_cousin(deps: DepsMut, env: &Env) -> Result<Response, ContractError> {
    let increment_msg = CounterContract::load(deps, env, "cousin").increment()?;

    Ok(Response::new().add_message(increment_msg))
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

    CounterContract::load(deps, &env, "cousin").set_raw_address(&cousin)?;

    Ok(Response::new().add_attribute("action", "set_cousin"))
}
