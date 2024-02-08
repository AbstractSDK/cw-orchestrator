use std::{cell::RefCell, rc::Rc};

use cosmwasm_std::Api;
use cw_multi_test::AppResponse;
use cw_orch_core::{
    environment::{NodeQuerier, Querier, QuerierGetter, StateInterface},
    CwEnvError,
};

use crate::{core::MockApp, MockBase};

pub struct MockNodeQuerier<A: Api> {
    app: Rc<RefCell<MockApp<A>>>,
}

impl<A: Api> MockNodeQuerier<A> {
    fn new<S: StateInterface>(mock: &MockBase<A, S>) -> Self {
        Self {
            app: mock.app.clone(),
        }
    }
}

impl<A: Api> Querier for MockNodeQuerier<A> {
    type Error = CwEnvError;
}

impl<A: Api, S: StateInterface> QuerierGetter<MockNodeQuerier<A>> for MockBase<A, S> {
    fn querier(&self) -> MockNodeQuerier<A> {
        MockNodeQuerier::new(self)
    }
}

impl<A: Api> NodeQuerier for MockNodeQuerier<A> {
    type Response = AppResponse;

    fn latest_block(&self) -> Result<cosmwasm_std::BlockInfo, Self::Error> {
        Ok(self.app.borrow().block_info())
    }

    fn block_by_height(&self, _height: u64) -> Result<cosmwasm_std::BlockInfo, Self::Error> {
        unimplemented!()
    }

    fn block_height(&self) -> Result<u64, Self::Error> {
        let block_info = self.latest_block()?;

        Ok(block_info.height)
    }

    fn block_time(&self) -> Result<u128, Self::Error> {
        let block_info = self.latest_block()?;

        Ok(block_info.time.nanos() as u128)
    }

    fn simulate_tx(&self, _tx_bytes: Vec<u8>) -> Result<u64, Self::Error> {
        unimplemented!()
    }

    fn find_tx(&self, _hash: String) -> Result<Self::Response, Self::Error> {
        unimplemented!()
    }
}
