use cosmwasm_std::Api;
use cw_orch_core::environment::{
    EnvironmentInfo, EnvironmentQuerier, QueryHandler, StateInterface,
};

use crate::MockBase;

impl<A: Api, S: StateInterface> EnvironmentQuerier for MockBase<A, S> {
    fn env_info(&self) -> EnvironmentInfo {
        let block_info = self.block_info().unwrap();
        let chain_id = block_info.chain_id.clone();
        let chain_name = chain_id.rsplitn(2, '-').last().unwrap().to_string();

        EnvironmentInfo {
            chain_id,
            chain_name: chain_name.clone(),
            deployment_id: "default".to_string(),
        }
    }
}
