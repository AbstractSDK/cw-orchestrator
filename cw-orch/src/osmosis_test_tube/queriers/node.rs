use std::{cell::RefCell, rc::Rc};

use crate::mock::cw_multi_test::AppResponse;
use cosmwasm_std::{BlockInfo, Timestamp};
use cw_orch_core::{
    environment::{NodeQuerier, Querier, QuerierGetter, StateInterface},
    CwEnvError,
};
use osmosis_test_tube::OsmosisTestApp;

use crate::prelude::OsmosisTestTube;

pub struct OsmosisTestTubeNodeQuerier {
    app: Rc<RefCell<OsmosisTestApp>>,
}

impl OsmosisTestTubeNodeQuerier {
    fn new<S: StateInterface>(mock: &OsmosisTestTube<S>) -> Self {
        Self {
            app: mock.app.clone(),
        }
    }
}

impl Querier for OsmosisTestTubeNodeQuerier {
    type Error = CwEnvError;
}

impl<S: StateInterface> QuerierGetter<OsmosisTestTubeNodeQuerier> for OsmosisTestTube<S> {
    fn querier(&self) -> OsmosisTestTubeNodeQuerier {
        OsmosisTestTubeNodeQuerier::new(self)
    }
}

impl NodeQuerier for OsmosisTestTubeNodeQuerier {
    type Response = AppResponse;
    fn latest_block(&self) -> Result<cosmwasm_std::BlockInfo, Self::Error> {
        Ok(BlockInfo {
            chain_id: "osmosis-1".to_string(),
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
