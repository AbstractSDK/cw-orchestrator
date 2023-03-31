use boot_core::{contract, Contract, CwEnv};
use cosmwasm_std::Empty;
use cw1_whitelist::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cw_multi_test::ContractWrapper;

#[contract(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw1;

// implement chain-generic functions
impl<Chain: CwEnv + Clone> Cw1<Chain> {
    pub fn new(id: &str, chain: Chain) -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let file_path = &format!("{}{}", crate_path, "/cw-artifacts/cw1_whitelist.wasm");
        Self(
            Contract::new(id, chain)
                .with_mock(Box::new(ContractWrapper::new_with_empty(
                    cw20_base::contract::execute,
                    cw20_base::contract::instantiate,
                    cw20_base::contract::query,
                )))
                .with_wasm_path(file_path),
        )
    }
    pub fn set_variant(self, filename: &str) -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let file_path = &format!("{}{}{}", crate_path, "/cw-artifacts/", filename);
        Self(self.0.with_wasm_path(file_path))
    }
}
