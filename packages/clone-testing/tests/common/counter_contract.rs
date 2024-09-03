use counter_contract::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cw_orch::prelude::*;
pub const CONTRACT_ID: &str = "counter_contract";

#[cw_orch::interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg, id = CONTRACT_ID)]
pub struct CounterContract;

// ANCHOR: uploadable_impl
impl<Chain> Uploadable for CounterContract<Chain> {
    /// Return the path to the wasm file corresponding to the contract
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("counter_contract")
            .unwrap()
    }
    /// Returns a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                counter_contract::contract::execute,
                counter_contract::contract::instantiate,
                counter_contract::contract::query,
            )
            .with_migrate(counter_contract::contract::migrate),
        )
    }
}
