use cw_orch_core::environment::{EnvironmentInfo, EnvironmentQuerier};

use crate::{senders::sender_trait::SenderTrait, DaemonBase};

impl<Sender: SenderTrait> EnvironmentQuerier for DaemonBase<Sender> {
    fn env_info(&self) -> EnvironmentInfo {
        EnvironmentInfo {
            chain_id: self.daemon.sender.chain_info().chain_id.clone(),
            chain_name: self
                .daemon
                .sender
                .chain_info()
                .network_info
                .chain_name
                .clone(),
            deployment_id: self.daemon.state.deployment_id.clone(),
        }
    }
}
