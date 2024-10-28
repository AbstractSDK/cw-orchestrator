// ANCHOR: custom_interface
use cw_orch::{interface, prelude::*};

use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cw_orch_on_chain::core::OnChain;

use cosmwasm_std::{Deps, Env};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct CounterContract;

impl<Chain: CwEnv> Uploadable for CounterContract<Chain> {
    /// Return the path to the wasm file corresponding to the contract
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("counter_cousin_contract")
            .unwrap()
    }
    /// Returns a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(ContractWrapper::new_with_empty(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        ))
    }
}
// ANCHOR_END: custom_interface
use cosmwasm_std::DepsMut;
use cw_orch_on_chain::core::OnChainDeps;

impl<'a> CounterContract<OnChain<'a>> {
    pub fn load(
        deps: impl Into<OnChainDeps<'a>>,
        env: &Env,
        contract_id: &str,
    ) -> CounterContract<OnChain<'a>> {
        let chain = OnChain::new(deps, env);
        CounterContract::new(contract_id, chain)
    }

    pub fn save(
        deps: DepsMut<'a>,
        env: &Env,
        contract_id: &str,
        address: Addr,
    ) -> CounterContract<OnChain<'a>> {
        let chain = OnChain::new(deps, env);
        let contract = CounterContract::new(contract_id, chain);
        contract.set_address(&address);
        contract
    }

    pub fn with_address(
        deps: Deps<'a>,
        env: &Env,
        contract_id: &str,
        address: Addr,
    ) -> CounterContract<OnChain<'a>> {
        let chain = OnChain::new(deps, env);
        let contract = CounterContract::new(contract_id, chain);
        contract.set_address(&address);
        contract
    }
}