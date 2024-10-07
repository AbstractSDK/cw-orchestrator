use cosmwasm_std::{to_json_binary, Deps, Env, QueryRequest, StdResult, WasmQuery};

use crate::interface::CounterContract;
use crate::msg::QueryMsgFns;
use crate::{
    msg::{GetCountResponse, QueryMsg},
    state::STATE,
};

pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(GetCountResponse { count: state.count })
}

pub fn cousin_count(deps: Deps, env: Env) -> StdResult<GetCountResponse> {
    CounterContract::load(deps, &env, "cousin")
        .get_count()
        .map_err(Into::into)
}

pub fn raw_cousin_count(deps: Deps) -> StdResult<GetCountResponse> {
    let state = STATE.load(deps.storage)?;
    let cousin_state = STATE.query(&deps.querier, state.cousin.unwrap())?;
    Ok(GetCountResponse {
        count: cousin_state.count,
    })
}
