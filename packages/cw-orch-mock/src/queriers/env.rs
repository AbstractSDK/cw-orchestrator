use cosmwasm_std::Api;
use cw_orch_core::environment::{
    EnvironmentInfo, EnvironmentQuerier, QueryHandler, StateInterface,
};

use crate::MockBase;

impl<A: Api, S: StateInterface> EnvironmentQuerier for MockBase<A, S> {
    fn env_info(&self) -> EnvironmentInfo {
        let block_info = self.block_info().unwrap();
        let chain_id = block_info.chain_id.clone();
        let chain_id_split = chain_id
            .rsplitn(2, '-')
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        // We make sure this doesn't error even if the chain id doesn't have the `osmosis-1` format
        let chain_name = if chain_id_split.len() > 2 {
            &chain_id_split[1]
        } else {
            &chain_id_split[0]
        };
        EnvironmentInfo {
            chain_id,
            chain_name: chain_name.clone(),
            deployment_id: "default".to_string(),
        }
    }
}
