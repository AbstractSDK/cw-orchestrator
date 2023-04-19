use boot_core::{contract, Contract, CwEnv};
use cosmwasm_std::Empty;
use cw1_subkeys::msg::{ExecuteMsg, QueryMsg};
pub use cw1_subkeys::msg::{
    ExecuteMsgFns as Cw1SubkeysExecuteMsgFns, QueryMsgFns as Cw1SubkeysQueryMsgFns,
};
use cw1_whitelist::msg::InstantiateMsg;
use cw_multi_test::ContractWrapper;

#[contract(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw1Subkeys;

// implement chain-generic functions
impl<Chain: CwEnv + Clone> Cw1Subkeys<Chain> {
    pub fn new(id: &str, chain: Chain) -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let file_path = &format!("{}{}", crate_path, "/cw-artifacts/cw1_subkeys.wasm");
        Self(
            Contract::new(id, chain)
                .with_mock(Box::new(ContractWrapper::new_with_empty(
                    cw1_subkeys::contract::execute,
                    cw1_subkeys::contract::instantiate,
                    cw1_subkeys::contract::query,
                )))
                .with_wasm_path(file_path),
        )
    }
}
