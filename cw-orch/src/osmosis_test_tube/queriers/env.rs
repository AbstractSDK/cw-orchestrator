use crate::osmosis_test_tube::OsmosisTestTube;
use cw_orch_core::environment::{EnvironmentInfo, EnvironmentQuerier, QueryHandler};

impl EnvironmentQuerier for OsmosisTestTube {
    fn env_info(&self) -> EnvironmentInfo {
        let block = self.block_info().unwrap();
        let chain_id = block.chain_id;
        let chain_name = chain_id.rsplitn(2, '-').collect::<Vec<_>>()[1].to_string();

        EnvironmentInfo {
            chain_id,
            chain_name,
            deployment_id: "default".to_string(),
        }
    }
}
