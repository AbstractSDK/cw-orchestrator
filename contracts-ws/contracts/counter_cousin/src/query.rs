use cosmwasm_std::{Deps, Env, StdResult};

use crate::interface::CounterContract;
use crate::msg::QueryMsgFns;
use crate::{msg::GetCountResponse, state::STATE};
use cw_orch::prelude::*;

pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetCountResponse { count: state.count })
}

pub fn cousin_count(deps: Deps, env: Env) -> StdResult<GetCountResponse> {
    CounterContract::load(deps, &env, "cousin")
        .get_count()
        .map_err(Into::into)
}

pub fn raw_cousin_count(deps: Deps, env: &Env) -> StdResult<GetCountResponse> {
    let cousin_state = CounterContract::load(deps, env, "cousin").item_query(STATE)?;
    Ok(GetCountResponse {
        count: cousin_state.count,
    })
}
