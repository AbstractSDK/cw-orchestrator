// ANCHOR: custom_interface
use cw_orch::{interface, prelude::*};

use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use cw_orch_on_chain::core::OnChain;

use cosmwasm_std::{Deps, Env};
use std::collections::HashMap;

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct CounterContract;

impl<Chain: CwEnv> Uploadable for CounterContract<Chain> {
    /// Return the path to the wasm file corresponding to the contract
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("counter_contract")
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

use crate::contract::CONTRACT_NAME;
use cosmwasm_std::DepsMut;
use cw_orch::anyhow::Result;
use cw_orch::prelude::queriers::Node;
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

// ANCHOR: daemon
impl CounterContract<Daemon> {
    /// Deploys the counter contract at a specific block height
    pub fn await_launch(&self) -> Result<()> {
        let daemon = self.environment();
        let rt = daemon.rt_handle.clone();

        rt.block_on(async {
            // Get the node query client, there are a lot of other clients available.
            let node: Node = daemon.querier();
            let mut latest_block = node.latest_block().unwrap();

            while latest_block.height < 100 {
                // wait for the next block
                daemon.next_block().unwrap();
                latest_block = node.latest_block().unwrap();
            }
        });

        let contract = CounterContract::new(CONTRACT_NAME, daemon.clone());

        // Upload the contract
        contract.upload().unwrap();

        // Instantiate the contract
        let msg = InstantiateMsg { count: 1i32 };
        contract.instantiate(&msg, None, &[]).unwrap();

        Ok(())
    }
}
// ANCHOR_END: daemon
