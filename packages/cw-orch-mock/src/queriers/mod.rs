use crate::MockBase;

use cosmwasm_std::Api;
use cw_multi_test::next_block;
use cw_orch_core::{
    environment::{DefaultQueriers, QueryHandler, StateInterface},
    CwEnvError,
};

pub mod bank;
mod env;
pub mod node;
pub mod wasm;

impl<A: Api, S: StateInterface> QueryHandler for MockBase<A, S> {
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

impl<A: Api, S: StateInterface> DefaultQueriers for MockBase<A, S> {
    type Bank = bank::MockBankQuerier<A>;
    type Wasm = wasm::MockWasmQuerier<A, S>;
    type Node = node::MockNodeQuerier<A>;
}
