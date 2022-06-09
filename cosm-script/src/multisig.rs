use crate::core_types::Coin;
use crate::error::CosmScriptError;
use base64::encode;
use cosmrs::cosmwasm::MsgExecuteContract;
use cosmrs::{AccountId, Coin as CosmCoin};
use serde::Serialize;
use serde_json::json;

pub struct Multisig;

impl Multisig {
    pub fn create_proposal<E: Serialize>(
        msg: &E,
        _deployment_name: &str,
        contract_addr: &str,
        multisig_addr: &str,
        sender_addr: AccountId,
        coins: &[CosmCoin],
    ) -> Result<MsgExecuteContract, CosmScriptError> {
        let encoded = encode(serde_json::to_string(&msg)?);
        let msg = json!({
          "propose": {
            "msgs": [
              {
                "wasm": {
                  "execute": {
                    "msg": encoded,
                    "funds": coins.iter().map(Coin::from).collect::<Vec<Coin>>(),
                    "contract_addr": contract_addr
                  }
                }
              }
            ],
            "title": "",
            "description": ""
          }
        });

        log::debug!("{}", msg);
        log::info!(
            "Proposed msgs: {}",
            json!([
              {
                "wasm": {
                  "execute": {
                    "msg": encoded,
                    "funds": coins.iter().map(Coin::from).collect::<Vec<Coin>>(),
                    "contract_addr": contract_addr
                  }
                }
              }
            ])
        );

        // let coins: Vec<CosmCoin> = coins
        //     .iter()
        //     .map(|coin| CosmCoin {
        //         denom: coin.denom.parse().unwrap(),
        //         amount: coin.amount.to_string().replace(".", "").parse().unwrap(),
        //     })
        //     .collect();

        Ok(cosmrs::cosmwasm::MsgExecuteContract {
            sender: sender_addr,
            contract: multisig_addr.to_string().parse::<AccountId>()?,
            msg: serde_json::to_string(&msg)?.into_bytes(),
            funds: coins.to_vec(),
        })
    }
}
