use crate::error::CwOrchError;

use cw_orch_core::environment::{DefaultQueriers, QueryHandler, StateInterface};

use super::InjectiveTestTube;

pub mod bank;
mod env;
pub mod node;
pub mod wasm;

impl<S: StateInterface> QueryHandler for InjectiveTestTube<S> {
    type Error = CwOrchError;

    fn wait_blocks(&self, _amount: u64) -> Result<(), CwOrchError> {
        panic!("Can't wait blocks on osmosis_test_tube")
    }

    fn wait_seconds(&self, secs: u64) -> Result<(), CwOrchError> {
        self.app.borrow().increase_time(secs);
        Ok(())
    }

    fn next_block(&self) -> Result<(), CwOrchError> {
        panic!("Can't wait blocks on osmosis_test_tube")
    }
}

impl<S: StateInterface> DefaultQueriers for InjectiveTestTube<S> {
    type Bank = bank::InjectiveTestTubeBankQuerier;
    type Wasm = wasm::InjectiveTestTubeWasmQuerier;
    type Node = node::InjectiveTestTubeNodeQuerier;
}
