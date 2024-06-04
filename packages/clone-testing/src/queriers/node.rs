use std::{cell::RefCell, rc::Rc};

use crate::{
    core::{AppResponse, CloneTestingApp},
    CloneTesting,
};
use cw_orch_core::{
    environment::{NodeQuerier, Querier, QuerierGetter, StateInterface},
    CwEnvError,
};

pub struct CloneNodeQuerier {
    app: Rc<RefCell<CloneTestingApp>>,
}

impl CloneNodeQuerier {
    fn new<S: StateInterface>(mock: &CloneTesting<S>) -> Self {
        Self {
            app: mock.app.clone(),
        }
    }
}

impl Querier for CloneNodeQuerier {
    type Error = CwEnvError;
}

impl<S: StateInterface> QuerierGetter<CloneNodeQuerier> for CloneTesting<S> {
    fn querier(&self) -> CloneNodeQuerier {
        CloneNodeQuerier::new(self)
    }
}

impl NodeQuerier for CloneNodeQuerier {
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
