use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw_storage_plus::Item;

// we use the cw_serde macro to add different traits related to our struct
// that will help us work with it in general
#[cw_serde]
pub struct Count(pub Uint128);

// this will hold our data!
pub const COUNT: Item<Count> = Item::new("count");
