//! This defines types and traits associated with IBC channels.
//! Those structures are mostly used internally for dealing with channel creation and analysis
//! But they can also be used in a user application if they need specific channel description

use ibc_relayer_types::core::ics24_host::identifier::ChannelId;
use ibc_relayer_types::core::ics24_host::identifier::PortId;

use crate::env::ChainId;
use crate::types::NetworkId;
use crate::InterchainError;

/// Identifies a channel between two IBC connected chains.
/// This describes only 1 side of the channel
#[derive(Debug, Clone)]
pub struct IbcPort<Channel> {
    /// The chain id of the network which belongs on one side of the channel
    pub chain_id: NetworkId,
    /// The connection-id on the network
    /// This might be not specified, especially when creating a channel
    pub connection_id: Option<String>,
    /// The port Id, that the channel binds on
    pub port: PortId,
    /// The channel-id on the network
    /// This might be not specified, especially when creating a channel
    pub channel: Option<ChannelId>,
    /// An accessor that allows interacting with the chain
    /// Examples
    /// Daemon: Transport channel that allows querying a node (for IBC queries for instance)
    /// Mock: This is empty because a cw-multi-test app is not `Sync` and therefore can't be used in this interchain environment
    pub chain: Channel,
}

/// Store information about a channel between 2 blockchains
/// The order of port_a and port_b is not important
/// Even if there is a src and dst chain, the order for an IBC channel doesn't matter
#[derive(Debug, Clone)]
pub struct InterchainChannel<Channel>
where
    Channel: Clone + Send + Sync,
{
    /// Port on one side of the channel
    pub port_a: IbcPort<Channel>,
    /// Port on the other side of the channel
    pub port_b: IbcPort<Channel>,
}

// TODO some of those queries may be implemented (or are already implemented) in the IBC querier file ?
impl<Channel> InterchainChannel<Channel>
where
    Channel: Clone + Send + Sync,
{
    /// Simple helper to create a new channel from its ports
    pub fn new(port_a: IbcPort<Channel>, port_b: IbcPort<Channel>) -> Self {
        Self { port_a, port_b }
    }

    /// Returns the channelidentification of one side of the channel
    /// Errors if the chain_id is not registered in the object
    pub fn get_chain(&self, chain_id: &str) -> Result<IbcPort<Channel>, InterchainError> {
        if chain_id.eq(&self.port_a.chain_id) {
            Ok(self.port_a.clone())
        } else if chain_id.eq(&self.port_b.chain_id) {
            Ok(self.port_b.clone())
        } else {
            return Err(InterchainError::ChainNotFound(chain_id.to_string()));
        }
    }

    /// Returns 2 ports of the channel if the order `(from_channel, to_channel)`
    /// where `from_channel` is the channel on the side of the `from` argument
    /// Errors if `from` is not registered in the object
    pub fn get_ordered_ports_from(
        &self,
        from: ChainId,
    ) -> Result<(IbcPort<Channel>, IbcPort<Channel>), InterchainError> {
        if from.eq(&self.port_a.chain_id) {
            Ok((self.port_a.clone(), self.port_b.clone()))
        } else if from.eq(&self.port_b.chain_id) {
            Ok((self.port_b.clone(), self.port_a.clone()))
        } else {
            return Err(InterchainError::ChainNotFound(from.to_string()));
        }
    }

    /// Similar to [InterchainChannel<Channel>::get_order_ports_from], with mutable access on the returned ports
    /// Errors if `from` is not registered in the object
    pub fn get_mut_ordered_ports_from(
        &mut self,
        from: ChainId,
    ) -> Result<(&mut IbcPort<Channel>, &mut IbcPort<Channel>), InterchainError> {
        if from.eq(&self.port_a.chain_id) {
            Ok((&mut self.port_a, &mut self.port_b))
        } else if from.eq(&self.port_b.chain_id) {
            Ok((&mut self.port_b, &mut self.port_a))
        } else {
            return Err(InterchainError::ChainNotFound(from.to_string()));
        }
    }
}
