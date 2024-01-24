use cosmwasm_std::{coin, coins};
use counter_contract::{
    msg::{ExecuteMsg, InstantiateMsg},
    CounterContract, CounterExecuteMsgFns, CounterQueryMsgFns,
};

use cw_orch::prelude::bank::BankQuerier;
use cw_orch::prelude::bank::BankQuerierGetter;
use cw_orch::prelude::{CallAs, ContractInstance, OsmosisTestTube};
use cw_orch::prelude::{CwOrchExecute, CwOrchInstantiate, CwOrchUpload};
use cw_orch_traits::Stargate;
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin,
    osmosis::tokenfactory::v1beta1::{
        MsgCreateDenom, MsgCreateDenomResponse, MsgMint, MsgMintResponse,
    },
};
use osmosis_test_tube::Account;
use prost::Message;
use prost_types::Any;

pub const SUBDENOM: &str = "sub-denom";
pub fn main() -> anyhow::Result<()> {
    env_logger::init();
    let chain = OsmosisTestTube::new(coins(1_000_000_000_000, "uosmo"));

    let contract_counter = CounterContract::new(chain.clone());

    contract_counter.upload()?;
    contract_counter.instantiate(&InstantiateMsg { count: 0 }, None, None)?;
    contract_counter.execute(&ExecuteMsg::Increment {}, None)?;

    let sender = contract_counter.as_instance().get_chain().sender.clone();
    let sender_addr = sender.address().to_string();

    contract_counter.call_as(&sender).increment()?;
    contract_counter.get_count()?;

    // We create a new denom
    chain.commit_any::<MsgCreateDenomResponse>(
        vec![Any {
            type_url: MsgCreateDenom::TYPE_URL.to_string(),
            value: MsgCreateDenom {
                sender: sender_addr.clone(),
                subdenom: SUBDENOM.to_string(),
            }
            .encode_to_vec(),
        }],
        None,
    )?;
    let denom = format!("factory/{}/{}", sender_addr, SUBDENOM);
    // We mint some tokens
    chain.commit_any::<MsgMintResponse>(
        vec![Any {
            type_url: MsgMint::TYPE_URL.to_string(),
            value: MsgMint {
                sender: sender_addr.clone(),
                amount: Some(Coin {
                    amount: "100000".to_string(),
                    denom: denom.clone(),
                }),
                mint_to_address: sender_addr.clone(),
            }
            .encode_to_vec(),
        }],
        None,
    )?;

    // We send it to the contract
    chain.bank_send(
        contract_counter.address()?.to_string(),
        vec![coin(50_000, denom.clone())],
    )?;

    // We verify everything has worked correctly
    assert_eq!(
        chain
            .bank_querier()
            .balance(contract_counter.address()?, Some(denom.clone()))?
            .first()
            .cloned(),
        Some(coin(50_000, denom.clone()))
    );
    assert_eq!(
        chain
            .bank_querier()
            .balance(sender_addr, Some(denom.clone()))?
            .first()
            .cloned(),
        Some(coin(50_000, denom.clone()))
    );

    Ok(())
}
