use boot_core::{contract, Contract, CwEnv};
use cosmwasm_std::Empty;
use cw3_fixed_multisig::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cw_multi_test::ContractWrapper;

pub use cw3_fixed_multisig::msg::{
    ExecuteMsgFns as Cw3FixedMultisigExecuteMsgFns, QueryMsgFns as Cw3FixedMultisigQueryMsgFns,
};

#[contract(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw3FixedMultisig;

// implement chain-generic functions
impl<Chain: CwEnv + Clone> Cw3FixedMultisig<Chain> {
    pub fn new(id: &str, chain: Chain) -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let file_path = &format!("{}{}", crate_path, "/cw-artifacts/cw3_fixed_multisig.wasm");
        Self(
            Contract::new(id, chain)
                .with_mock(Box::new(ContractWrapper::new_with_empty(
                    cw3_fixed_multisig::contract::execute,
                    cw3_fixed_multisig::contract::instantiate,
                    cw3_fixed_multisig::contract::query,
                )))
                .with_wasm_path(file_path),
        )
    }
}
