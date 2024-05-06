use cw_orch_core::environment::{EnvironmentInfo, EnvironmentQuerier};

use crate::Daemon;

impl EnvironmentQuerier for Daemon {
    fn env_info(&self) -> EnvironmentInfo {
        let chain_info = &self.daemon.chain_info;
        let locked_state = self.daemon.state.lock().unwrap();

        EnvironmentInfo {
            chain_id: chain_info.chain_id.to_string(),
            chain_name: chain_info.network_info.chain_name.to_string(),
            deployment_id: locked_state.deployment_id.clone(),
        }
    }
}
