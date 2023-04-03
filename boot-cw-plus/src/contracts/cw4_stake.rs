use boot_core::{contract, Contract, CwEnv};
use cosmwasm_std::Empty;
use cw4_stake::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cw_multi_test::ContractWrapper;

pub use cw4_stake::msg::{
    ExecuteMsgFns as Cw4StakeExecuteMsgFns, QueryMsgFns as Cw4StakeQueryMsgFns,
};

#[contract(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw4Stake;

// implement chain-generic functions
impl<Chain: CwEnv + Clone> Cw4Stake<Chain> {
    pub fn new(id: &str, chain: Chain) -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let file_path = &format!("{}{}", crate_path, "/cw-artifacts/cw4_stake.wasm");
        Self(
            Contract::new(id, chain)
                .with_mock(Box::new(ContractWrapper::new_with_empty(
                    cw4_stake::contract::execute,
                    cw4_stake::contract::instantiate,
                    cw4_stake::contract::query,
                )))
                .with_wasm_path(file_path),
        )
    }
}
