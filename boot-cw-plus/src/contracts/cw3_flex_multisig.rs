use boot_core::{contract, Contract, CwEnv};
use cosmwasm_std::Empty;
use cw3_flex_multisig::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cw_multi_test::ContractWrapper;

pub use cw3_flex_multisig::msg::{
    ExecuteMsgFns as Cw3FlexMultisigExecuteMsgFns, QueryMsgFns as Cw3FlexMultisigQueryMsgFns,
};

#[contract(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw3FlexMultisig;

// implement chain-generic functions
impl<Chain: CwEnv + Clone> Cw3FlexMultisig<Chain> {
    pub fn new(id: &str, chain: Chain) -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let file_path = &format!("{}{}", crate_path, "/cw-artifacts/cw3_flex_multisig.wasm");
        Self(
            Contract::new(id, chain)
                .with_mock(Box::new(ContractWrapper::new_with_empty(
                    cw3_flex_multisig::contract::execute,
                    cw3_flex_multisig::contract::instantiate,
                    cw3_flex_multisig::contract::query,
                )))
                .with_wasm_path(file_path),
        )
    }
}
