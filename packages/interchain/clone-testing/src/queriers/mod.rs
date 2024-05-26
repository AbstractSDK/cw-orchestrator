use crate::{CloneTesting, MockState};

use clone_cw_multi_test::next_block;

use cw_orch_core::{
    environment::{DefaultQueriers, QueryHandler},
    CwEnvError,
};
pub mod bank;
mod env;
pub mod node;
pub mod wasm;

impl QueryHandler for CloneTesting {
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

impl DefaultQueriers for CloneTesting {
    type Bank = bank::CloneBankQuerier;
    type Wasm = wasm::CloneWasmQuerier<MockState>;
    type Node = node::CloneNodeQuerier;
}
