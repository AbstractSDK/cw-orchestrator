use boot_core::{contract, Contract, CwEnv};

use cw20_base::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cw_multi_test::ContractWrapper;

pub use cw20::msg::Cw20ExecuteMsgFns;
pub use cw20_base::msg::QueryMsgFns as Cw20QueryMsgFns;

#[contract(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Cw20Base;

// Implement chain-generic functions
impl<Chain: CwEnv> Cw20Base<Chain> {
    pub fn new(id: &str, chain: Chain) -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let file_path = &format!("{}{}", crate_path, "/cw-artifacts/cw20_base.wasm");
        Self(
            Contract::new(id, chain)
                .with_mock(Box::new(
                    ContractWrapper::new_with_empty(
                        cw20_base::contract::execute,
                        cw20_base::contract::instantiate,
                        cw20_base::contract::query,
                    )
                    .with_migrate(cw20_base::contract::migrate),
                ))
                .with_wasm_path(file_path),
        )
    }
}
