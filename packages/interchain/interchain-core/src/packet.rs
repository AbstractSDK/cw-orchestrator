use cosmwasm_std::Binary;
use ibc_relayer_types::core::{ics04_channel::packet::Sequence, ics24_host::identifier::{ChannelId, PortId}};

use crate::results::NetworkId;

/// Structure to hold simple information about a sent packet
#[derive(Debug, Clone)]
pub struct IbcPacketInfo {
    /// Port on which is packet was sent
    pub src_port: PortId,
    /// Channel on which is packet was sent
    pub src_channel: ChannelId,
    /// Packet identification (sequence is `u64` number)
    pub sequence: Sequence,
    /// Chain identification to which the packet was sent
    pub dst_chain_id: NetworkId,
}

/// Raw packet outcome
/// The T generic is used to allow for raw transactions or analyzed transactions to be used
#[derive(Debug, PartialEq, Clone)]
pub enum IbcPacketOutcome<T> {
    /// Packet timeout
    Timeout {
        /// Only a timeout transaction gets broadcasted
        timeout_tx: T,
    },
    /// Packet successfully transferred
    Success {
        /// The packets gets transmitted to the dst chain
        receive_tx: T,
        /// The ack is broadcasted back on the src chain
        ack_tx: T,
        /// The raw binary acknowledgement retrieved from `ack_tx`
        ack: Binary,
    },
}