use crate::error::CwOrchError;
use cosmwasm_std::{Addr, BlockInfo, Timestamp};
use cw_orch_core::environment::{queriers::QueryHandler, StateInterface};
use osmosis_test_tube::{Module, Wasm};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

use super::{map_err, OsmosisTestTube};

pub mod bank;
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

    fn block_info(&self) -> Result<cosmwasm_std::BlockInfo, CwOrchError> {
        Ok(BlockInfo {
            chain_id: "osmosis-1".to_string(),
            height: self.app.borrow().get_block_height().try_into().unwrap(),
            time: Timestamp::from_nanos(
                self.app.borrow().get_block_time_nanos().try_into().unwrap(),
            ),
        })
    }

    fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, CwOrchError> {
        let query = Wasm::new(&*self.app.borrow())
            .query(contract_address.as_ref(), query_msg)
            .map_err(map_err)?;

        Ok(query)
    }
}
