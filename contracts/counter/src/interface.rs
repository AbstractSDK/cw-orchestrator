// ANCHOR: custom_interface
use cw_orch::{interface, prelude::*};

use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct CounterContract;

impl<Chain: CwEnv> Uploadable for CounterContract<Chain> {
    /// Return the path to the wasm file corresponding to the contract
    fn wasm(&self) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("counter_contract")
            .unwrap()
    }
    /// Returns a CosmWasm contract wrapper
    fn wrapper(&self) -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                crate::contract::execute,
                crate::contract::instantiate,
                crate::contract::query,
            )
            .with_migrate(crate::contract::migrate),
        )
    }
}
// ANCHOR_END: custom_interface

use crate::contract::CONTRACT_NAME;
use cw_orch::anyhow::Result;
use cw_orch::prelude::queriers::Node;

// ANCHOR: daemon
impl CounterContract<Daemon> {
    /// Deploys the counter contract at a specific block height
    pub fn await_launch(&self) -> Result<()> {
        let daemon = self.get_chain();
        let rt = daemon.rt_handle.clone();

        rt.block_on(async {
            // Get the node query client, there are a lot of other clients available.
            let node = daemon.query_client::<Node>();
            let mut latest_block = node.latest_block().await.unwrap();

            while latest_block.header.height.value() < 100 {
                // wait for the next block
                daemon.next_block().unwrap();
                latest_block = node.latest_block().await.unwrap();
            }
        });

        let contract = CounterContract::new(CONTRACT_NAME, daemon.clone());

        // Upload the contract
        contract.upload().unwrap();

        // Instantiate the contract
        let msg = InstantiateMsg { count: 1i32 };
        contract.instantiate(&msg, None, None).unwrap();

        Ok(())
    }
}
// ANCHOR_END: daemon
