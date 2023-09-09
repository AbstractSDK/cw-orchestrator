//! # DaemonQuerier
//!
//! DaemonAsync queriers are gRPC query clients for the CosmosSDK modules. They can be used to query the different modules (Bank, Ibc, Authz, ...).
//!
//! ## Usage
//!
//! You will need to acquire a [gRPC channel](Channel) to a running CosmosSDK node to be able to use the queriers.
//! Here is an example of how to acquire one using the DaemonAsync builder.
//!
//! ```no_run
//! // require the querier you want to use, in this case Node
//! use cw_orch_daemon::{queriers::Node, DaemonAsync, networks, queriers::DaemonQuerier};
//! # tokio_test::block_on(async {
//! // call the builder and configure it as you need
//! let daemon = DaemonAsync::builder()
//!     .chain(networks::LOCAL_JUNO)
//!     .build()
//!     .await.unwrap();
//! // now you can use the Node querier:
//! let node = Node::new(daemon.channel());
//! let node_info = node.info();
//! # })
//! ```

/// macro for constructing and performing a query on a CosmosSDK module.
#[macro_export]
macro_rules! cosmos_query {
    ($self:ident, $module:ident, $func_name:ident, $request_type:ident { $($field:ident : $value:expr),* $(,)?  }) => {
        {
        use $crate::cosmos_modules::$module::{
            query_client::QueryClient, $request_type,
        };
        let mut client = QueryClient::new($self.channel.clone());
        #[allow(clippy::redundant_field_names)]
        let request = $request_type { $($field : $value),* };
        let response = client.$func_name(request.clone()).await?.into_inner();
        ::log::trace!(
            "cosmos_query: {:?} resulted in: {:?}",
            request,
            response
        );
        response
    }
};
}

mod bank;
mod cosmwasm;
mod feegrant;
mod gov;
mod ibc;
mod node;
mod staking;

pub use bank::Bank;
pub use cosmwasm::CosmWasm;
pub use feegrant::Feegrant;
pub use ibc::Ibc;
pub use node::Node;

// this two containt structs that are helpers for the queries
pub use gov::*;
pub use staking::*;

use tonic::transport::Channel;

/// Constructor for a querier over a given channel
pub trait DaemonQuerier {
    /// Construct an new querier over a given channel
    fn new(channel: Channel) -> Self;
}
