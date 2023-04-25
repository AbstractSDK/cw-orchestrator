use cosmwasm_std::{Addr, Uint128};
use tokio::runtime::Runtime;

use uid::Id as IdT;

#[derive(Copy, Clone, Eq, PartialEq)]
struct DeployId(());

type Id = IdT<DeployId>;

use boot_core::{
    contract, networks::LOCAL_JUNO, Contract, ContractWrapper, Daemon, Mock, Uploadable, WasmPath,
};

const CW20_CONTRACT_WASM: &str = "/../boot-cw-plus/cw-artifacts/cw20_base.wasm";

#[contract(
    cw20_base::msg::InstantiateMsg,
    cw20_base::msg::ExecuteMsg,
    cw20_base::msg::QueryMsg,
    cw20_base::msg::MigrateMsg
)]
pub struct Cw20;

impl Uploadable<Mock> for Cw20<Mock> {
    fn source(&self) -> <Mock as boot_core::TxHandler>::ContractSource {
        Box::new(
            ContractWrapper::new_with_empty(
                cw20_base::contract::execute,
                cw20_base::contract::instantiate,
                cw20_base::contract::query,
            )
            .with_migrate(cw20_base::contract::migrate),
        )
    }
}

impl Uploadable<Daemon> for Cw20<Daemon> {
    fn source(&self) -> <Daemon as boot_core::TxHandler>::ContractSource {
        // create contract base configuration
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let wasm_path = format!("{}{}", crate_path, CW20_CONTRACT_WASM);
        log::info!("Using wasm path {}", wasm_path);
        WasmPath::new(wasm_path).unwrap()
    }
}

pub fn start(runtime: &Runtime) -> (cosmwasm_std::Addr, Cw20<Daemon>) {
    let id = Id::new();

    let daemon = Daemon::builder()
        .chain(LOCAL_JUNO)
        .handle(runtime.handle())
        .build()
        .unwrap();

    let sender = daemon.sender.address().unwrap();

    let contract = Cw20(Contract::new(format!("cw-plus:cw20_base:{}", id), daemon));

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
