use std::{cell::RefCell, rc::Rc};

use crate::mock::cw_multi_test::AppResponse;
use cosmwasm_std::{BlockInfo, Timestamp};
use cw_orch_core::{
    environment::{NodeQuerier, Querier, QuerierGetter, StateInterface},
    CwEnvError,
};
use injective_test_tube::InjectiveTestApp;

use crate::injective_test_tube::InjectiveTestTube;

pub struct InjectiveTestTubeNodeQuerier {
    app: Rc<RefCell<InjectiveTestApp>>,
}

impl InjectiveTestTubeNodeQuerier {
    fn new<S: StateInterface>(mock: &InjectiveTestTube<S>) -> Self {
        Self {
            app: mock.app.clone(),
        }
    }
}

impl Querier for InjectiveTestTubeNodeQuerier {
    type Error = CwEnvError;
}

impl<S: StateInterface> QuerierGetter<InjectiveTestTubeNodeQuerier> for InjectiveTestTube<S> {
    fn querier(&self) -> InjectiveTestTubeNodeQuerier {
        InjectiveTestTubeNodeQuerier::new(self)
    }
}

impl NodeQuerier for InjectiveTestTubeNodeQuerier {
    type Response = AppResponse;
    fn latest_block(&self) -> Result<cosmwasm_std::BlockInfo, Self::Error> {
        Ok(BlockInfo {
            chain_id: "injective-1".to_string(),
            height: self.block_height()?,
            time: Timestamp::from_nanos(self.block_time()?.try_into().unwrap()),
        })
    }

    fn block_by_height(&self, _height: u64) -> Result<cosmwasm_std::BlockInfo, Self::Error> {
        unimplemented!()
    }

    fn block_height(&self) -> Result<u64, Self::Error> {
        Ok(self.app.borrow().get_block_height().try_into().unwrap())
    }

    fn block_time(&self) -> Result<u128, Self::Error> {
        Ok(self.app.borrow().get_block_time_nanos().try_into().unwrap())
    }

    fn simulate_tx(&self, _tx_bytes: Vec<u8>) -> Result<u64, Self::Error> {
        unimplemented!()
    }

    fn find_tx(&self, _hash: String) -> Result<Self::Response, Self::Error> {
        unimplemented!()
    }
}
