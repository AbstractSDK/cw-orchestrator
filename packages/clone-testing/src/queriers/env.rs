use crate::CloneTesting;

use cw_orch_core::environment::{EnvironmentInfo, EnvironmentQuerier};

impl EnvironmentQuerier for CloneTesting {
    fn env_info(&self) -> EnvironmentInfo {
        let state = &self.state.borrow().daemon_state;
        EnvironmentInfo {
            chain_id: state.chain_data.chain_id.to_string(),
            chain_name: state.chain_data.network_info.chain_name.clone(),
            deployment_id: state.deployment_id.clone(),
        }
    }
}
