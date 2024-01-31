use crate::error::CwOrchError;

use cw_orch_core::environment::{DefaultQueriers, QueryHandler, StateInterface};

use super::OsmosisTestTube;

pub mod bank;
mod env;
pub mod node;
pub mod wasm;

impl<S: StateInterface> QueryHandler for OsmosisTestTube<S> {
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

impl<S: StateInterface> DefaultQueriers for OsmosisTestTube<S> {
    type B = bank::OsmosisTestTubeBankQuerier;
    type W = wasm::OsmosisTestTubeWasmQuerier;
    type N = node::OsmosisTestTubeNodeQuerier;
}
