use crate::{error::TerraRustScriptError, core_types::Coin, chain::Chain};
use base64::encode;
use cosmrs::{tx::Body, AccountId,Coin as CosmCoin ,crypto::PublicKey};
use serde_json::{json, Value};
use cosmrs::cosmwasm::MsgExecuteContract;

pub struct Multisig;

impl Multisig {
    pub fn create_proposal(
        json_msg: &Value,
        _group_name: &str,
        contract_addr: &str,
        multisig_addr: &str,
        sender_addr: AccountId,
        chain: &Chain,
        coins: &[Coin],
    ) -> Result<MsgExecuteContract, TerraRustScriptError> {
        let encoded = encode(json_msg.to_string());
        let msg = json!({
          "propose": {
            "msgs": [
              {
                "wasm": {
                  "execute": {
                    "msg": encoded,
                    "funds": coins,
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
                    "funds": coins,
                    "contract_addr": contract_addr
                  }
                }
              }
            ])
        );

        let coins: Vec<CosmCoin> = coins.iter().map(|coin| CosmCoin{denom: coin.denom.parse().unwrap(), amount: coin.amount.to_string().replace(".", "").parse().unwrap()}).collect();

        Ok(cosmrs::cosmwasm::MsgExecuteContract{
          sender: sender_addr,
          contract: contract_addr.to_string().parse::<AccountId>()?,
          msg: serde_json::to_string(&msg)?.into_bytes(),
          funds: coins
        })
    }
}
