use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Count(pub Uint128);

pub const COUNT: Item<Count> = Item::new("count");
