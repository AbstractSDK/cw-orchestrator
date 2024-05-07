use cw_orch::{
    environment::{DefaultQueriers, QueryHandler, StateInterface},
    prelude::CwOrchError,
};

use super::OsmosisTestTube;

pub mod bank;
mod env;
pub mod node;
pub mod wasm;

impl<S: StateInterface> QueryHandler for OsmosisTestTube<S> {
    type Error = CwOrchError;

    fn wait_blocks(&self, amount: u64) -> Result<(), CwOrchError> {
        self.wait_seconds(amount * 10)
    }

    fn wait_seconds(&self, secs: u64) -> Result<(), CwOrchError> {
        self.app.borrow().increase_time(secs);
        Ok(())
    }

    fn next_block(&self) -> Result<(), CwOrchError> {
        self.wait_blocks(1)
    }
}

impl<S: StateInterface> DefaultQueriers for OsmosisTestTube<S> {
    type Bank = bank::OsmosisTestTubeBankQuerier;
    type Wasm = wasm::OsmosisTestTubeWasmQuerier<S>;
    type Node = node::OsmosisTestTubeNodeQuerier;
}
