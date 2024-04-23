use cw_orch_core::environment::{EnvironmentInfo, EnvironmentQuerier};

use crate::Daemon;

impl EnvironmentQuerier for Daemon {
    fn env_info(&self) -> EnvironmentInfo {
        let state = &self.daemon.sender.daemon_state;
        EnvironmentInfo {
            chain_id: state.chain_data.chain_id.to_string(),
            chain_name: state.chain_data.network_info.id.to_string(),
            deployment_id: state.deployment_id.clone(),
        }
    }
}
