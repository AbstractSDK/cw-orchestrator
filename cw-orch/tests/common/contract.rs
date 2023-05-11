use cosmwasm_std::{Addr, Uint128};
use cw_orch::interface;
use tokio::runtime::Runtime;

use uid::Id as IdT;

#[derive(Copy, Clone, Eq, PartialEq)]
struct DeployId(());

type Id = IdT<DeployId>;

use cw_orch::{contract::Contract, environment::TxHandler, prelude::*};

// path to local cw20.wasm artifact
const CW20_CONTRACT_WASM: &str = "tests/common/artifacts/cw20_base.wasm";

#[interface(
    cw20_base::msg::InstantiateMsg,
    cw20_base::msg::ExecuteMsg,
    cw20_base::msg::QueryMsg,
    cw20_base::msg::MigrateMsg
)]
pub struct Cw20<Chain>;

impl<Chain: CwEnv> Cw20<Chain> {
    pub fn new(chain: Chain) -> Self {
        let id = Id::new();
        Self(Contract::new(format!("cw-plus:cw20_base:{}", id), chain))
    }
}

impl<Chain: CwEnv> Uploadable for Cw20<Chain> {
    fn wasm(&self) -> <Daemon as TxHandler>::ContractSource {
        // create contract base configuration
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let wasm_path = format!("{}/{}", crate_path, CW20_CONTRACT_WASM);
        log::info!("Using wasm path {}", wasm_path);
        WasmPath::new(wasm_path).unwrap()
    }
    fn wrapper(&self) -> <Mock as TxHandler>::ContractSource {
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

pub fn start(runtime: &Runtime) -> (cosmwasm_std::Addr, Cw20<Daemon>) {
    let daemon = Daemon::builder()
        .chain(networks::LOCAL_JUNO)
        .handle(runtime.handle())
        .build()
        .unwrap();

    let sender = daemon.sender.address().unwrap();

    let contract = Cw20::new(daemon);

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
