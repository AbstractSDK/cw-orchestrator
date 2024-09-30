//! Analysis of committed IBC related Txs and Packets.

use crate::ibc_query::IbcQueryHandler;
use crate::packet::IbcPacketOutcome;
use crate::tx::TxId;
use crate::{IbcAckParser, InterchainError};
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

/// Result of the analysis of all packets sent in a transaction
#[derive(Clone)]
pub struct IbcTxAnalysis<Chain: CwEnv> {
    /// Identification of the transaction
    pub tx_id: TxId<Chain>,
    /// Result of following a packet + Recursive Analysis of the resulting transactions for additional IBC packets
    pub packets: Vec<FullIbcPacketAnalysis<Chain>>,
}

/// Contains packet information after it was successfully acknowledged on the sending chain
#[derive(Clone)]
pub struct SuccessIbcPacket<Chain: CwEnv> {
    /// Identification of the transaction
    pub send_tx: TxId<Chain>,
    /// Raw bytes returned during the acknowledgement
    pub packet_ack: Binary,
}


impl<Chain: CwEnv> IbcTxAnalysis<Chain> {
    /// Assert that all packets were not timeout
    pub fn assert_no_timeout(&self) -> Result<Vec<SuccessIbcPacket<Chain>>, InterchainError> {
        Ok(self
            .packets
            .iter()
            .map(|p| p.assert_no_timeout())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    /// Returns all packets that were successful without asserting there was no timeout
    pub fn get_success_packets(&self) -> Result<Vec<SuccessIbcPacket<Chain>>, InterchainError> {
        Ok(self
            .packets
            .iter()
            .map(|p| p.get_success_packets())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }

    /// Tries to parses all acknowledgements into polytone, ics20 and ics004 acks.
    /// Errors if some packet doesn't conform to those results.
    pub fn into_result(&self) -> Result<(), InterchainError> {
        self.packets.iter().try_for_each(|p| p.into_result())?;
        Ok(())
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


impl<Chain: CwEnv> FullIbcPacketAnalysis<Chain> {
    /// Tries to parses all acknowledgements into polytone, ics20 and ics004 acks.
    /// Errors if some packet doesn't conform to those results.
    /// Use [`InterchainEnv::parse_ibc`] if you want to handle your own acks
    pub fn into_result(&self) -> Result<(), InterchainError> {
        match &self.outcome {
            IbcPacketOutcome::Success {
                ack,
                receive_tx,
                ack_tx,
            } => {
                receive_tx.into_result()?;
                ack_tx.into_result()?;

                if IbcAckParser::polytone_ack(ack).is_ok() {
                    return Ok(());
                }
                if IbcAckParser::ics20_ack(ack).is_ok() {
                    return Ok(());
                }
                if IbcAckParser::ics004_ack(ack).is_ok() {
                    return Ok(());
                }

                Err(InterchainError::AckDecodingFailed(
                    ack.clone(),
                    String::from_utf8_lossy(ack.as_slice()).to_string(),
                ))
            }
            IbcPacketOutcome::Timeout { .. } => Err(InterchainError::PacketTimeout {}),
        }
    }

    /// Returns all successful packets gathered during the packet following procedure
    /// Doesn't error if a packet has timed-out
    pub fn get_success_packets(&self) -> Result<Vec<SuccessIbcPacket<Chain>>, InterchainError> {
        match &self.outcome {
            IbcPacketOutcome::Success {
                ack,
                receive_tx,
                ack_tx,
            } => Ok([
                vec![SuccessIbcPacket {
                    send_tx: self.send_tx.clone().unwrap(),
                    packet_ack: ack.clone(),
                }],
                receive_tx.get_success_packets()?,
                ack_tx.get_success_packets()?,
            ]
            .concat()),
            IbcPacketOutcome::Timeout { .. } => Ok(vec![]),
        }
    }

    /// Returns all successful packets gathered during the packet following procedure
    /// Errors if a packet has timed-out
    pub fn assert_no_timeout(&self) -> Result<Vec<SuccessIbcPacket<Chain>>, InterchainError> {
        match &self.outcome {
            IbcPacketOutcome::Success {
                ack,
                receive_tx,
                ack_tx,
            } => Ok([
                vec![SuccessIbcPacket {
                    send_tx: self.send_tx.clone().unwrap(),
                    packet_ack: ack.clone(),
                }],
                receive_tx.assert_no_timeout()?,
                ack_tx.assert_no_timeout()?,
            ]
            .concat()),
            IbcPacketOutcome::Timeout { .. } => Err(InterchainError::PacketTimeout {}),
        }
    }
}

mod debug {
    use cw_orch_core::environment::CwEnv;

    use super::{IbcPacketAnalysis, IbcTxAnalysis, SuccessIbcPacket};

    impl<C: CwEnv, Tx: std::fmt::Debug> std::fmt::Debug for IbcPacketAnalysis<C, Tx> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("IbcPacketAnalysis")
                .field("send_tx", &self.send_tx)
                .field("outcome", &self.outcome)
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


    impl<C: CwEnv> std::fmt::Debug for SuccessIbcPacket<C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SuccessIbcPacket")
                .field("sent_tx", &self.send_tx)
                .field("packet_ack", &self.packet_ack)
                .finish()
        }
    }
}
