use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ContractError;
use cosmwasm_std::{Addr, Deps, MessageInfo};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state");

pub fn assert_owner(deps: Deps, info: &MessageInfo) -> Result<(), ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}
