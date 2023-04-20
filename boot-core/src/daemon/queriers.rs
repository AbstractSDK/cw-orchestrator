#[macro_export]
macro_rules! cosmos_query {
    ($self: ident, $module:ident, $request_type:ident { $($field:ident : $value:expr),* $(,)? }) => {{
        use crate::daemon::cosmos_modules::$module::{
            query_client::QueryClient, $request_type,
        };
        use paste::paste;

        let mut client = QueryClient::new($self.channel.clone());
        let request = $request_type { $($field : $value),* };

        paste! {
            client.[<$request_type:snake>](request).await?.into_inner()
        }
        ::log::debug!(
            "cosmos_query: {:?} resulted in: {:?}",
            request,
            resp
        );
        resp
    }};
}

mod bank;
mod cosmwasm;
mod ibc;
mod node;

pub use bank::Bank;
pub use cosmwasm::CosmWasm;
pub use ibc::Ibc;
pub use node::Node;

use tonic::transport::Channel;

/// Constructor for a querier over a given channel
pub trait DaemonQuerier {
    /// Construct an new querier over a given channel
    fn new(channel: Channel) -> Self;
}
