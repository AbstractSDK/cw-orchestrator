use boot_core::{contract, Contract, CwEnv};
use cosmwasm_std::Empty;
use cw1_whitelist::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cw_multi_test::ContractWrapper;

pub use cw1_whitelist::msg::{
    ExecuteMsgFns as Cw1WhitelistExecuteMsgFns, QueryMsgFns as Cw1WhitelistQueryMsgFns,
};

#[contract(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw1Whitelist;

impl<Chain: CwEnv + Clone> Cw1Whitelist<Chain> {
    pub fn new(id: &str, chain: Chain) -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let file_path = &format!("{}{}", crate_path, "/cw-artifacts/cw1_whitelist.wasm");
        Self(
            Contract::new(id, chain)
                .with_mock(Box::new(ContractWrapper::new_with_empty(
                    cw1_whitelist::contract::execute,
                    cw1_whitelist::contract::instantiate,
                    cw1_whitelist::contract::query,
                )))
                .with_wasm_path(file_path),
        )
    }
}
