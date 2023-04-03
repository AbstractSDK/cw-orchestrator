use boot_core::{contract, Contract, CwEnv};
use cw20_ics20::msg::{ExecuteMsg, InitMsg, MigrateMsg, QueryMsg};
use cw_multi_test::ContractWrapper;

pub use cw20_ics20::msg::{
    ExecuteMsgFns as Cw20Ics20ExecuteMsgFns, QueryMsgFns as Cw20Ics20QueryMsgFns,
};

#[contract(InitMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Cw20Ics20;

// implement chain-generic functions
impl<Chain: CwEnv> Cw20Ics20<Chain> {
    pub fn new(id: &str, chain: Chain) -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let file_path = &format!("{}{}", crate_path, "/cw-artifacts/cw20_ics20.wasm");
        Self(
            Contract::new(id, chain)
                .with_mock(Box::new(ContractWrapper::new_with_empty(
                    cw20_ics20::contract::execute,
                    cw20_ics20::contract::instantiate,
                    cw20_ics20::contract::query,
                )))
                .with_wasm_path(file_path),
        )
    }
}
