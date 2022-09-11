use std::str::FromStr;

use cosmrs::Denom;

use crate::CosmScriptError;

pub(crate) mod client_types;
pub(crate) mod core_types;
// pub(crate) mod _daemon_state;
pub(crate) mod daemon_state;
pub(crate) mod tx_resp;

pub fn parse_cw_coins(coins: &[cosmwasm_std::Coin]) -> Result<Vec<cosmrs::Coin>, CosmScriptError> {
    coins
        .into_iter()
        .map(|cosmwasm_std::Coin { amount, denom }| {
            let parsed_amount = cosmwasm_std::Decimal::from_atomics(amount.clone(), 6)
                .map_err(|e| CosmScriptError::StdErr(e.to_string()))?;
            let in_go_decimal = cosmrs::Decimal::from_str(&parsed_amount.to_string())?;
            Ok(cosmrs::Coin {
                amount: in_go_decimal,
                denom: Denom::from_str(denom)?,
            })
        })
        .collect::<Result<Vec<_>, CosmScriptError>>()
}
