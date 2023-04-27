use cosmwasm_std::{Addr, Uint128};
use tokio::runtime::Runtime;

use uid::Id as IdT;
#[derive(Copy, Clone, Eq, PartialEq)]
struct DeployId(());

type Id = IdT<DeployId>;

use cw_orch::{
    networks::LOCAL_JUNO, Daemon
};

use cw20_base::contract::Cw20Base;
// Use cw-plus contract defined with the interface macro

pub fn start(runtime: &Runtime) -> (cosmwasm_std::Addr, Cw20Base<Daemon>) {
    let id = Id::new();

    let daemon = Daemon::builder()
        .chain(LOCAL_JUNO)
        .handle(runtime.handle())
        .build()
        .unwrap();

    let sender = daemon.sender.address().unwrap();

    let contract = Cw20Base::new(format!("cw-plus:cw20_base:{}", id), daemon);

    (sender, contract)
}

pub fn get_init_msg(sender: &Addr) -> cw20_base::msg::InstantiateMsg {
    cw20_base::msg::InstantiateMsg {
        name: "Token".to_owned(),
        symbol: "TOK".to_owned(),
        decimals: 6u8,
        initial_balances: vec![cw20::Cw20Coin {
            address: sender.to_string(),
            amount: Uint128::from(10000u128),
        }],
        mint: None,
        marketing: None,
    }
}
