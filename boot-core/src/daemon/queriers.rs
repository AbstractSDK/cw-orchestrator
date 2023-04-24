mod bank;
mod cosmwasm;
mod node;

pub use bank::Bank;
pub use cosmwasm::CosmWasm;
pub use node::Node;

use tonic::transport::Channel;

/// Constructor for a querier over a given channel
pub trait DaemonQuerier {
    /// Construct an new querier over a given channel
    fn new(channel: Channel) -> Self;
}
