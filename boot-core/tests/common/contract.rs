use cosmwasm_std::{Addr, Uint128};
use std::sync::Arc;
use tokio::runtime::Runtime;

use uid::Id as IdT;

#[derive(Copy, Clone, Eq, PartialEq)]
struct DeployId(());

type Id = IdT<DeployId>;

use boot_core::{
    instantiate_daemon_env, networks::LOCAL_JUNO, Contract, ContractWrapper, Daemon,
    DaemonOptionsBuilder,
};

const CW20_CONTRACT_WASM: &str = "/../boot-cw-plus/cw-artifacts/cw20_base.wasm";

pub fn start() -> (cosmwasm_std::Addr, Contract<Daemon>) {
    let runtime = Arc::new(Runtime::new().unwrap());

    let id = Id::new();

    let options = DaemonOptionsBuilder::default()
        .network(LOCAL_JUNO)
        .deployment_id(format!("{}", id))
        .build()
        .unwrap();

    let (sender, chain) = instantiate_daemon_env(&runtime, options).unwrap();

    // create contract base configuration
    let crate_path = env!("CARGO_MANIFEST_DIR");
    let wasm_path = format!("{}{}", crate_path, CW20_CONTRACT_WASM);
    log::info!("Using wasm path {}", wasm_path);

    let contract = Contract::new(format!("cw-plus:cw20_base:{}", id), chain)
        .with_mock(Box::new(
            ContractWrapper::new_with_empty(
                cw20_base::contract::execute,
                cw20_base::contract::instantiate,
                cw20_base::contract::query,
            )
            .with_migrate(cw20_base::contract::migrate),
        ))
        .with_wasm_path(wasm_path);

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
