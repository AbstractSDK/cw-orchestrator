use super::super::{sender::Wallet, DaemonAsync};
use crate::{queriers::DaemonQuerier, DaemonBuilder};

use tokio::runtime::Handle;
use tonic::transport::Channel;

#[derive(Clone)]
/**
    Represents a blockchain node.
    Is constructed with the [DaemonBuilder].

    ## Usage

    ```rust,no_run
    use cw_orch::prelude::Daemon;
    use cw_orch::networks::JUNO_1;
    use tokio::runtime::Runtime;

    let rt = Runtime::new().unwrap();
    let daemon: Daemon = Daemon::builder()
        .chain(JUNO_1)
        .handle(rt.handle())
        .build()
        .unwrap();
    ```
    ## Environment Execution

    The Daemon implements [`TxHandler`] which allows you to perform transactions on the chain.

    ## Querying

    Different Cosmos SDK modules can be queried through the daemon by calling the [`Daemon.query_client<Querier>`] method with a specific querier.
    See [Querier](crate::queriers) for examples.
*/
pub struct Daemon {
    pub daemon: DaemonAsync,
    /// Runtime handle to execute async tasks
    pub rt_handle: Handle,
}

impl Daemon {
    /// Get the daemon builder
    pub fn builder() -> DaemonBuilder {
        DaemonBuilder::default()
    }

    /// Perform a query with a given querier
    /// See [Querier](crate::queriers) for examples.
    pub fn query_client<Querier: DaemonQuerier>(&self) -> Querier {
        self.daemon.query_client()
    }

    /// Get the channel configured for this Daemon
    pub fn channel(&self) -> Channel {
        self.daemon.state.grpc_channel.clone()
    }

    /// Get the channel configured for this Daemon
    pub fn wallet(&self) -> Wallet {
        self.daemon.sender.clone()
    }
}
