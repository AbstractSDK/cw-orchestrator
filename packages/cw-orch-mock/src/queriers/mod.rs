use crate::Mock;

use cw_multi_test::next_block;
use cw_orch_core::{
    environment::{DefaultQueriers, QueryHandler, StateInterface},
    CwEnvError,
};

pub mod bank;
mod env;
pub mod node;
pub mod wasm;

impl<S: StateInterface> QueryHandler for Mock<S> {
    type Error = CwEnvError;

    fn wait_blocks(&self, amount: u64) -> Result<(), CwEnvError> {
        self.app.borrow_mut().update_block(|b| {
            b.height += amount;
            b.time = b.time.plus_seconds(5 * amount);
        });
        Ok(())
    }

    fn wait_seconds(&self, secs: u64) -> Result<(), CwEnvError> {
        self.app.borrow_mut().update_block(|b| {
            b.time = b.time.plus_seconds(secs);
            b.height += secs / 5;
        });
        Ok(())
    }

    fn next_block(&self) -> Result<(), CwEnvError> {
        self.app.borrow_mut().update_block(next_block);
        Ok(())
    }
}

impl<S: StateInterface> DefaultQueriers for Mock<S> {
    type B = bank::MockBankQuerier;
    type W = wasm::MockWasmQuerier;
    type N = node::MockNodeQuerier;
}
