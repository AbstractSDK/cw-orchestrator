use crate::{channel::InterchainChannel, env::ChannelCreation};
use cosmwasm_std::{Api, Binary, StdError};
use cw_orch_core::environment::IndexResponse;
use cw_orch_core::environment::QueryHandler;
use cw_orch_core::environment::{CwEnv, TxHandler};
use cw_orch_mock::{MockBase, MockState};
use ibc_relayer_types::core::{
    ics04_channel::packet::Sequence,
    ics24_host::identifier::{ChannelId, PortId},
};

/// Chain identification for cw-orch Ibc purposes
pub type NetworkId = String;

/// The result of following a single packet across IBC
/// This allows indentifying the different transactions involved as well as the result of the packet transmission
pub type SimpleIbcPacketAnalysis<Chain> = IbcPacketAnalysis<Chain, TxId<Chain>>;

/// The result of following all packets sent across IBC during a single transaction
pub type FullIbcPacketAnalysis<Chain> = IbcPacketAnalysis<Chain, IbcTxAnalysis<Chain>>;

/// Generic type to store the outcome of an IBC packet
#[derive(Clone)]
pub struct IbcPacketAnalysis<Chain: CwEnv, Tx> {
    /// The transaction during which the packet was sent
    pub send_tx: Option<TxId<Chain>>,
    /// Outcome transactions of the packet (+ eventual acknowledgment)
    pub outcome: IbcPacketOutcome<Tx>,
}

/// Identifies a transaction
#[derive(Clone)]
pub struct TxId<Chain: CwEnv> {
    /// Chain Id on which the transaction was broadcasted
    pub chain_id: String,
    /// Transactions response for the transaction (env specific)
    pub response: <Chain as TxHandler>::Response,
}

/// Result of the analysis of all packets sent in a transaction
#[derive(Clone)]
#[must_use = "We recommend using `into_result()` on this result to assert ibc success"]
pub struct IbcTxAnalysis<Chain: CwEnv> {
    /// Identification of the transaction
    pub tx_id: TxId<Chain>,
    /// Result of following a packet + Recursive Analysis of the resulting transactions for additional IBC packets
    pub packets: Vec<FullIbcPacketAnalysis<Chain>>,
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

/// Adds additional capabilities to CwEnv for use with ibc environments
pub trait IbcQueryHandler: CwEnv {
    /// Query handler for the environment
    /// This should allow users to query anything related to IBC functionalities on the environment (if possible)
    type Handler: Clone + Send + Sync;

    /// Returns the `IbcQueryHandler::Handler` associated with the environment
    fn ibc_handler(&self) -> Self::Handler;

    /// Returns the chain id of the environment (for ibc identification purposes)
    fn chain_id(&self) -> NetworkId;
}

#[cfg(feature = "daemon")]
// Temporary until we can actually push to cw-orch-daemon
impl IbcQueryHandler for cw_orch_daemon::Daemon {
    type Handler = tonic::transport::Channel;

    fn ibc_handler(&self) -> tonic::transport::Channel {
        self.channel()
    }

    fn chain_id(&self) -> NetworkId {
        use cw_orch_core::environment::ChainState;

        self.state().chain_data.chain_id.to_string()
    }
}

// Temporary until we can actually push to cw-orch-mock
impl<A: Api> IbcQueryHandler for MockBase<A, MockState> {
    type Handler = ();
    fn ibc_handler(&self) {}

    fn chain_id(&self) -> NetworkId {
        self.block_info().unwrap().chain_id
    }
}

// Return types for the env trait
/// Result returned by  InterchainEnv::_internal_create_channel
pub struct InternalChannelCreationResult<ChannelCreationResult> {
    /// Channel creation result specific the used chain
    pub result: ChannelCreationResult,
    /// Connection id on which the channel was created.
    /// This connection id is supposed to be known by the channel creation environment
    pub src_connection_id: String,
}

/// Result returned by  InterchainEnv::get_channel_creation_txs
pub struct ChannelCreationTransactionsResult<Chain: TxHandler> {
    /// Id of the channel that was just created on the src chain
    pub src_channel_id: ChannelId,
    /// Id of the channel that was just created on the dst chain
    pub dst_channel_id: ChannelId,
    /// Transactions involved in the channel creation
    pub channel_creation_txs: ChannelCreation<<Chain as TxHandler>::Response>,
}

/// Result returned by  InterchainEnv::create_channel
pub struct ChannelCreationResult<Chain: IbcQueryHandler> {
    /// Channel object containing every variable needed for identifying the channel that was just created
    pub interchain_channel: InterchainChannel<<Chain as IbcQueryHandler>::Handler>,
    /// Transactions involved in the channel creation + Their packet following analysis
    pub channel_creation_txs: ChannelCreation<IbcTxAnalysis<Chain>>,
}

mod debug {
    use cw_orch_core::environment::CwEnv;

    use super::{IbcPacketAnalysis, IbcTxAnalysis, TxId};

    impl<C: CwEnv, Tx: std::fmt::Debug> std::fmt::Debug for IbcPacketAnalysis<C, Tx> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("IbcPacketAnalysis")
                .field("send_tx", &self.send_tx)
                .field("outcome", &self.outcome)
                .finish()
        }
    }

    impl<C: CwEnv> std::fmt::Debug for TxId<C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TxId")
                .field("chain_id", &self.chain_id)
                .field("response", &self.response)
                .finish()
        }
    }

    impl<C: CwEnv> std::fmt::Debug for IbcTxAnalysis<C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("IbcTxAnalysis")
                .field("tx_id", &self.tx_id)
                .field("packets", &self.packets)
                .finish()
        }
    }
}

impl<Chain: CwEnv> IndexResponse for IbcTxAnalysis<Chain> {
    fn events(&self) -> Vec<cosmwasm_std::Event> {
        let mut self_events = self.tx_id.response.events();
        let other_events =
            self.packets
                .iter()
                .flat_map(|packet_result| match &packet_result.outcome {
                    IbcPacketOutcome::Timeout { timeout_tx } => timeout_tx.events(),
                    IbcPacketOutcome::Success {
                        receive_tx,
                        ack_tx,
                        ack: _,
                    } => [receive_tx.events(), ack_tx.events()].concat(),
                });
        self_events.extend(other_events);
        self_events
    }

    fn event_attr_value(
        &self,
        event_type: &str,
        attr_key: &str,
    ) -> cosmwasm_std::StdResult<String> {
        self.tx_id
            .response
            .event_attr_value(event_type, attr_key)
            .or_else(|_| {
                self.packets
                    .iter()
                    .find_map(|packet_result| match &packet_result.outcome {
                        IbcPacketOutcome::Timeout { timeout_tx } => {
                            timeout_tx.event_attr_value(event_type, attr_key).ok()
                        }
                        IbcPacketOutcome::Success {
                            receive_tx,
                            ack_tx,
                            ack: _,
                        } => receive_tx
                            .event_attr_value(event_type, attr_key)
                            .or_else(|_| ack_tx.event_attr_value(event_type, attr_key))
                            .ok(),
                    })
                    .ok_or(StdError::generic_err(format!(
                        "event of type {event_type} does not have a value at key {attr_key}"
                    )))
            })
    }

    fn event_attr_values(&self, event_type: &str, attr_key: &str) -> Vec<String> {
        let mut all_results = self.tx_id.response.event_attr_values(event_type, attr_key);

        all_results.extend(self.packets.iter().flat_map(|packet_result| {
            match &packet_result.outcome {
                IbcPacketOutcome::Timeout { timeout_tx } => {
                    timeout_tx.event_attr_values(event_type, attr_key)
                }
                IbcPacketOutcome::Success {
                    receive_tx,
                    ack_tx,
                    ack: _,
                } => [
                    receive_tx.event_attr_values(event_type, attr_key),
                    ack_tx.event_attr_values(event_type, attr_key),
                ]
                .concat(),
            }
        }));

        all_results
    }

    fn data(&self) -> Option<Binary> {
        unimplemented!("No data fields on Ibc Tx Analysis")
    }
}

/// Contains structs to hold parsed IBC ack results
pub mod parse {
    use cosmwasm_std::Binary;
    use cw_orch_core::environment::CwEnv;

    use super::TxId;

    /// Contains packet information after it was successfully acknowledged on the sending chain
    #[derive(Clone)]
    pub struct SuccessIbcPacket<Chain: CwEnv> {
        /// Identification of the transaction
        pub send_tx: TxId<Chain>,
        /// Raw bytes returned during the acknowledgement
        pub packet_ack: Binary,
    }

    mod debug {
        use cw_orch_core::environment::CwEnv;

        use super::SuccessIbcPacket;

        impl<C: CwEnv> std::fmt::Debug for SuccessIbcPacket<C> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("SuccessIbcPacket")
                    .field("sent_tx", &self.send_tx)
                    .field("packet_ack", &self.packet_ack)
                    .finish()
            }
        }
    }
}
