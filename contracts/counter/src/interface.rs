#![allow(unused)]
// ANCHOR: custom_interface
use cw_orch::interface;
use cw_orch::prelude::*;
use cw_orch::prelude::queriers::Node;

use crate::CounterContract;
use crate::contract::CONTRACT_NAME;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Counter;

impl<Chain: CwEnv> Uploadable for Counter<Chain> {
    // Return the path to the wasm file
    fn wasm(&self) -> WasmPath {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let wasm_path = format!("{}/../artifacts/{}", crate_path, "mock.wasm");

        WasmPath::new(wasm_path).unwrap()
    }
    // Return a CosmWasm contract wrapper
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

// ANCHOR: daemon
impl Counter<Daemon> {
    /// Deploys the counter contract at a specific block height
    pub fn await_launch(&self) -> anyhow::Result<()> {
        let daemon = self.get_chain();
        let rt = daemon.rt_handle.clone();

        rt.block_on(
            async {
                // Get the node query client, there are a lot of other clients available.
                let node = daemon.query_client::<Node>();
                let mut latest_block = node.latest_block().await.unwrap() ;
                
                while latest_block.header.height.value() < 100 {
                    // wait for the next block
                    daemon.next_block().unwrap();
                    latest_block = node.latest_block().await.unwrap() ;
                }
            }
        );

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
