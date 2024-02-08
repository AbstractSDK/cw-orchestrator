use crate::MockBase;

use cosmwasm_std::testing::MockApi;
use cw_multi_test::{addons::MockApiBech32, next_block};
use cw_orch_core::{
    environment::{DefaultQueriers, QueryHandler, StateInterface},
    CwEnvError,
};

pub mod bank;
mod env;
pub mod node;
pub mod wasm;

impl<S: StateInterface> QueryHandler for MockBase<MockApi, S> {
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
impl<S: StateInterface> QueryHandler for MockBase<MockApiBech32, S> {
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

impl<S: StateInterface> DefaultQueriers for MockBase<MockApi, S> {
    type Bank = bank::MockBankQuerier<MockApi>;
    type Wasm = wasm::MockWasmQuerier<MockApi>;
    type Node = node::MockNodeQuerier<MockApi>;
}
impl<S: StateInterface> DefaultQueriers for MockBase<MockApiBech32, S> {
    type Bank = bank::MockBankQuerier<MockApiBech32>;
    type Wasm = wasm::MockWasmQuerier<MockApiBech32>;
    type Node = node::MockNodeQuerier<MockApiBech32>;
}
