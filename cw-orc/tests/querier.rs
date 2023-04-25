mod common;
use std::{str::FromStr, sync::Arc};

use common::channel::build_channel;
use cw_orc::{
    queriers::{CosmWasm, DaemonQuerier, Node},
    ContractInstance, CwOrcInstantiate, CwOrcUpload, DaemonError,
};
use speculoos::prelude::*;
use tokio::runtime::Runtime;

use cosmrs::{
    cosmwasm::MsgExecuteContract,
    tx::{self, Msg},
    AccountId, Denom,
};

/*
*/
#[test]
fn general() {
    let rt = Arc::new(Runtime::new().unwrap());
    let channel = rt.block_on(build_channel()).unwrap();

    let node = Node::new(channel.clone());
    let block_height = rt.block_on(node.block_height());
    asserting!("block_height is ok").that(&block_height).is_ok();

    let latest_block = rt.block_on(node.latest_block());
    asserting!("latest_block is ok").that(&latest_block).is_ok();

    let block_time = rt.block_on(node.block_time());
    asserting!("block_time is ok").that(&block_time).is_ok();
}

#[test]
fn simulate_tx() {
    let rt = Arc::new(Runtime::new().unwrap());

    let channel = rt.block_on(build_channel()).unwrap();

    let node = Node::new(channel.clone());

    let exec_msg = cw20_base::msg::ExecuteMsg::Mint {
        recipient: "terra1fd68ah02gr2y8ze7tm9te7m70zlmc7vjyyhs6xlhsdmqqcjud4dql4wpxr".into(),
        amount: 128u128.into(),
    };

    let exec_msg: MsgExecuteContract = MsgExecuteContract {
        sender: AccountId::from_str(
            "terra1ygcvxp9s054q8u2q4hvl52ke393zvgj0sllahlycm4mj8dm96zjsa45rzk",
        )
        .unwrap(),
        contract: AccountId::from_str(
            "terra1nsuqsk6kh58ulczatwev87ttq2z6r3pusulg9r24mfj2fvtzd4uq3exn26",
        )
        .unwrap(),
        msg: serde_json::to_vec(&exec_msg).unwrap(),
        funds: parse_cw_coins(&vec![]).unwrap(),
    };

    let msgs = [exec_msg]
        .into_iter()
        .map(Msg::into_any)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let memo = String::from("");

    let body = tx::Body::new(msgs, memo, 100u32);

    let simulate_tx = rt.block_on(node.simulate_tx(body.into_bytes().unwrap()));

    asserting!("that simulate_tx worked but msg is wrong")
        .that(&simulate_tx)
        .is_err();
}

#[test]
fn contract_info() {
    let rt = Arc::new(Runtime::new().unwrap());
    let channel = rt.block_on(build_channel()).unwrap();
    let cosm_wasm = CosmWasm::new(channel.clone());

    let (sender, contract) = common::contract::start(&rt);

    let _ = contract.upload();

    let init_msg = common::contract::get_init_msg(&sender);

    let _ = contract.instantiate(&init_msg, Some(&sender.clone()), None);

    let contract_address = contract.address().unwrap();

    let contract_info = rt.block_on(cosm_wasm.contract_info(contract_address));

    asserting!("contract info is ok")
        .that(&contract_info)
        .is_ok();
}

fn parse_cw_coins(coins: &[cosmwasm_std::Coin]) -> Result<Vec<cosmrs::Coin>, DaemonError> {
    coins
        .iter()
        .map(|cosmwasm_std::Coin { amount, denom }| {
            Ok(cosmrs::Coin {
                amount: amount.u128(),
                denom: Denom::from_str(denom)?,
            })
        })
        .collect::<Result<Vec<_>, DaemonError>>()
}
