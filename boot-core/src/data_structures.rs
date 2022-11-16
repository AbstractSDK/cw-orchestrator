use std::str::FromStr;

use cosmrs::Denom;

use crate::BootError;

pub(crate) mod client_types;
pub(crate) mod core_types;
// pub(crate) mod _daemon_state;
pub(crate) mod daemon_state;
pub mod mock_state;
pub(crate) mod tx_resp;

pub fn parse_cw_coins(coins: &[cosmwasm_std::Coin]) -> Result<Vec<cosmrs::Coin>, BootError> {
    coins
        .iter()
        .map(|cosmwasm_std::Coin { amount, denom }| {
            Ok(cosmrs::Coin {
                amount: amount.u128(),
                denom: Denom::from_str(denom)?,
            })
        })
        .collect::<Result<Vec<_>, BootError>>()
}

pub fn parse_rs_coins(coins: &[cosmrs::Coin]) -> Result<Vec<cosmwasm_std::Coin>, BootError> {
    coins
        .iter()
        .map(|cosmrs::Coin { amount, denom }| {
            Ok(cosmwasm_std::Coin {
                amount: amount.clone().into(),
                denom: denom.to_string(),
            })
        })
        .collect::<Result<Vec<_>, BootError>>()
}
