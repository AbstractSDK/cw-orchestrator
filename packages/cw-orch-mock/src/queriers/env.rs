use cosmwasm_std::testing::MockApi;
use cw_multi_test::addons::MockApiBech32;
use cw_orch_core::environment::{
    EnvironmentInfo, EnvironmentQuerier, QueryHandler, StateInterface,
};

use crate::MockBase;

impl<S: StateInterface> EnvironmentQuerier for MockBase<MockApiBech32, S> {
    fn env_info(&self) -> EnvironmentInfo {
        let block_info = self.block_info().unwrap();
        let chain_id = block_info.chain_id.clone();
        let chain_name = chain_id.rsplitn(2, '-').collect::<Vec<_>>()[1].to_string();
        EnvironmentInfo {
            chain_id,
            chain_name,
            deployment_id: "default".to_string(),
        }
    }
}

impl<S: StateInterface> EnvironmentQuerier for MockBase<MockApi, S> {
    fn env_info(&self) -> EnvironmentInfo {
        let block_info = self.block_info().unwrap();
        let chain_id = block_info.chain_id.clone();
        let chain_name = chain_id.rsplitn(2, '-').collect::<Vec<_>>()[1].to_string();
        EnvironmentInfo {
            chain_id,
            chain_name,
            deployment_id: "default".to_string(),
        }
    }
}
