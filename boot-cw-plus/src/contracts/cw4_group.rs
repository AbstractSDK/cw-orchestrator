use boot_core::{contract, Contract, CwEnv};
use cosmwasm_std::Empty;
use cw4_group::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cw_multi_test::ContractWrapper;

pub use cw4_group::msg::{
    ExecuteMsgFns as Cw3FlexMultisigExecuteMsgFns, QueryMsgFns as Cw3FlexMultisigQueryMsgFns,
};

#[contract(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw4Group;

// implement chain-generic functions
impl<Chain: CwEnv + Clone> Cw4Group<Chain> {
    pub fn new(id: &str, chain: Chain) -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let file_path = &format!("{}{}", crate_path, "/cw-artifacts/cw4_group.wasm");
        Self(
            Contract::new(id, chain)
                .with_mock(Box::new(ContractWrapper::new_with_empty(
                    cw4_group::contract::execute,
                    cw4_group::contract::instantiate,
                    cw4_group::contract::query,
                )))
                .with_wasm_path(file_path),
        )
    }
}
