use std::sync::Arc;

use crate::{error::DaemonError, DaemonBase, GrpcChannel};

use cw_orch_core::environment::ChainInfoOwned;

use tonic::transport::Channel;

use super::{builder::SenderBuilder, query::QuerySender};

/// Daemon that does not support signing.
/// Will err on any attempt to sign a transaction or retrieve a sender address.
pub type QueryOnlyDaemon = DaemonBase<QueryOnlySender>;

pub struct QueryOnlySenderOptions {}

/// Signer of the transactions and helper for address derivation
#[derive(Clone)]
pub struct QueryOnlySender {
    /// gRPC channel
    pub channel: Channel,
    /// Information about the chain
    pub chain_info: Arc<ChainInfoOwned>,
}

impl SenderBuilder for QueryOnlySenderOptions {
    type Error = DaemonError;
    type Sender = QueryOnlySender;

    async fn build(&self, chain_info: &Arc<ChainInfoOwned>) -> Result<Self::Sender, Self::Error> {
        let channel = GrpcChannel::from_chain_info(chain_info.as_ref()).await?;

        Ok(QueryOnlySender {
            channel,
            chain_info: chain_info.clone(),
        })
    }
}

impl QuerySender for QueryOnlySender {
    type Error = DaemonError;
    type Options = QueryOnlySenderOptions;

    fn channel(&self) -> Channel {
        self.channel.clone()
    }
}

#[cfg(test)]
mod tests {
    use cw_orch_networks::networks::JUNO_1;

    use super::QueryOnlyDaemon;
    use crate::DaemonBuilder;

    #[test]
    #[serial_test::serial]
    fn build() {
        let _query_only_daemon: QueryOnlyDaemon =
            DaemonBuilder::new(JUNO_1).build_sender(()).unwrap();
    }
}
