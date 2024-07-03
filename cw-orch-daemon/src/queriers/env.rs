use cw_orch_core::environment::{EnvironmentInfo, EnvironmentQuerier};

use crate::{senders::query::QuerySender, DaemonBase};

impl<Sender: QuerySender> EnvironmentQuerier for DaemonBase<Sender> {
    fn env_info(&self) -> EnvironmentInfo {
        let binding = self.daemon.sender();
        let info = binding.chain_info();
        EnvironmentInfo {
            chain_id: info.chain_id.clone(),
            chain_name: info.network_info.chain_name.clone(),
            deployment_id: self.daemon.state.deployment_id.clone(),
        }
    }
}
