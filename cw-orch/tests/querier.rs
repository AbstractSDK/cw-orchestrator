mod common;
use std::{str::FromStr, sync::Arc};

use common::channel::build_channel;
use cw_orch::{
    daemon::{
        error::DaemonError,
        queriers::{Bank, CosmWasm, DaemonQuerier, Gov, Ibc, Node, Staking},
    },
    prelude::{queriers::StakingBondStatus, *},
};

use speculoos::prelude::*;
use tokio::runtime::Runtime;

use cosmrs::{
    cosmwasm::MsgExecuteContract,
    tx::{self, Msg},
    AccountId, Denom,
};

/*
    Querier - Ibc
*/
#[test]
#[cfg(feature = "node-tests")]
fn ibc() {
    let rt = Arc::new(Runtime::new().unwrap());
    let channel = rt.block_on(build_channel());

    let ibc = Ibc::new(channel);

    let clients = rt.block_on(ibc.clients());
    asserting!("clients is ok").that(&clients).is_ok();
}

/*
    Querier - Staking
*/
#[test]
#[cfg(feature = "node-tests")]
fn staking() {
    let rt = Arc::new(Runtime::new().unwrap());
    let channel = rt.block_on(build_channel());

    let staking = Staking::new(channel);

    let params = rt.block_on(staking.params());
    asserting!("params is ok").that(&params).is_ok();

    let validators = rt.block_on(staking.validators(StakingBondStatus::Bonded));
    asserting!("validators is ok").that(&validators).is_ok();
    asserting!("validators is not empty")
        .that(&validators.unwrap().len())
        .is_equal_to(1);
}

/*
    Querier - Gov
*/
#[test]
#[cfg(feature = "node-tests")]
fn gov() {
    let rt = Arc::new(Runtime::new().unwrap());
    let channel = rt.block_on(build_channel());

    let gov = Gov::new(channel);

    let params = rt.block_on(gov.params("voting"));
    asserting!("params is ok").that(&params).is_ok();
}

/*
    Querier - Bank
*/
#[test]
#[cfg(feature = "node-tests")]
fn bank() {
    let rt = Arc::new(Runtime::new().unwrap());
    let channel = rt.block_on(build_channel());

    let bank = Bank::new(channel);

    let params = rt.block_on(bank.params());
    asserting!("params is ok").that(&params).is_ok();

    let balances = rt.block_on(bank.balance("juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y", None));
    asserting!("balances is ok").that(&balances).is_ok();

    let spendable_balances =
        rt.block_on(bank.spendable_balances("juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y"));
    asserting!("spendable_balances is ok")
        .that(&spendable_balances)
        .is_ok();

    let total_supply = rt.block_on(bank.total_supply());
    asserting!("total_supply is ok").that(&total_supply).is_ok();

    let supply_of = rt.block_on(bank.supply_of("ujunox"));
    asserting!("supply_of is ok").that(&supply_of).is_ok();

    let denom_metadata = rt.block_on(bank.denom_metadata("ucosm"));
    asserting!("denom_metadata is err, should not exists")
        .that(&denom_metadata)
        .is_err();

    let denoms_metadata = rt.block_on(bank.denoms_metadata(None));
    asserting!("denoms_metadata is ok, but empty")
        .that(&denoms_metadata)
        .is_ok();
}

/*
    Querier - CosmWasm
*/
#[test]
#[cfg(feature = "node-tests")]
fn cosmwasm() {
    let rt = Arc::new(Runtime::new().unwrap());
    let channel = rt.block_on(build_channel());

    let cw = CosmWasm::new(channel);

    let params = rt.block_on(cw.params());
    asserting!("params is ok").that(&params).is_ok();
}

/*
    Querier - Node
*/
#[test]
#[cfg(feature = "node-tests")]
fn node() {
    let rt = Arc::new(Runtime::new().unwrap());
    let channel = rt.block_on(build_channel());

    let node = Node::new(channel);

    let block_height = rt.block_on(node.block_height());
    asserting!("block_height is ok").that(&block_height).is_ok();

    let latest_block = rt.block_on(node.latest_block());
    asserting!("latest_block is ok").that(&latest_block).is_ok();

    let block_time = rt.block_on(node.block_time());
    asserting!("block_time is ok").that(&block_time).is_ok();
}

#[test]
#[cfg(feature = "node-tests")]
fn simulate_tx() {
    let rt = Arc::new(Runtime::new().unwrap());

    let channel = rt.block_on(build_channel());

    let node = Node::new(channel);

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
        funds: parse_cw_coins(&[]).unwrap(),
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
#[cfg(feature = "node-tests")]
fn contract_info() {
    let rt = Arc::new(Runtime::new().unwrap());
    let channel = rt.block_on(build_channel());
    let cosm_wasm = CosmWasm::new(channel);

    let (sender, contract) = common::contract::start(&rt);

    let _ = contract.upload();

    let init_msg = common::contract::get_init_msg(&sender);

    let _ = contract.instantiate(&init_msg, Some(&sender), None);

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
