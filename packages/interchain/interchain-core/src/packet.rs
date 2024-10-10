use cosmwasm_std::{Binary, StdError};
use cw_orch_core::environment::CwEnv;
use cw_orch_core::environment::IndexResponse;
use ibc_relayer_types::core::{
    ics04_channel::packet::Sequence,
    ics24_host::identifier::{ChannelId, PortId},
};

use crate::{results::NetworkId, tx::TxId};

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
#[must_use = "We recommend using `PacketAnalysis::assert()` to assert ibc success"]
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

/// The result of awaiting the Lifecycle of Single packet across IBC
///
/// This identifies:
/// - `send_tx`: The transaction in which the packet was sent (if available)
/// - `outcome`: The outcome of the Lifecycle and the corresponding transactions (Receive/Acknowledgement or Timeout)
#[derive(Clone)]
#[must_use = "We recommend using `PacketAnalysis::assert()` to assert IBC success"]
pub struct SinglePacketFlow<Chain: CwEnv> {
    /// The transaction during which the packet was sent
    ///
    /// Can optionally be specified, depending on the environment on which the implementation is done
    /// This is not available for the [`Mock`] implementation for instance
    pub send_tx: Option<TxId<Chain>>,
    /// Outcome transactions of the packet (+ eventual acknowledgment)
    pub outcome: IbcPacketOutcome<TxId<Chain>>,
}

/// The result of awaiting all packets sent across IBC during a single transaction.
///
/// This structure is nested and allows identifying all packets emitted during the subsequent (receive/acknowledgement/timeout) transactions
///
/// This identifies:
/// - `tx_id`: The original transaction that was used as a starting point of the awaiting procedure
/// - `packets`: For each packet sent inside this transaction, this contains the outcome of the lifecycle of the packet.
///     This also contains the result of awaiting all packets sent across IBC during each outcome transactions (receive/acknowledgement/timeout)
#[derive(Clone)]
#[must_use = "We recommend using `PacketAnalysis::assert()` to assert IBC success"]
pub struct NestedPacketsFlow<Chain: CwEnv> {
    /// Identification of the transaction
    pub tx_id: TxId<Chain>,
    /// Result of following a packet + Recursive Analysis of the resulting transactions for additional IBC packets
    pub packets: Vec<IbcPacketOutcome<NestedPacketsFlow<Chain>>>,
}

impl<Chain: CwEnv> IndexResponse for SinglePacketFlow<Chain> {
    fn events(&self) -> Vec<cosmwasm_std::Event> {
        let mut events: Vec<_> = self
            .send_tx
            .as_ref()
            .map(|tx| tx.response.events())
            .unwrap_or_default();
        let other_events = self.outcome.events();
        events.extend(other_events);

        events
    }

    fn event_attr_value(
        &self,
        event_type: &str,
        attr_key: &str,
    ) -> cosmwasm_std::StdResult<String> {
        self.send_tx
            .as_ref()
            .map(|r| r.event_attr_value(event_type, attr_key))
            .and_then(|res| res.ok())
            .or_else(|| self.outcome.event_attr_value(event_type, attr_key).ok())
            .ok_or(StdError::generic_err(format!(
                "event of type {event_type} does not have a value at key {attr_key}"
            )))
    }

    fn event_attr_values(&self, event_type: &str, attr_key: &str) -> Vec<String> {
        let mut all_results: Vec<_> = self
            .send_tx
            .as_ref()
            .map(|tx| tx.response.event_attr_values(event_type, attr_key))
            .unwrap_or_default();
        let other_results = self.outcome.event_attr_values(event_type, attr_key);
        all_results.extend(other_results);

        all_results
    }

    fn data(&self) -> Option<Binary> {
        unimplemented!("No data fields on Ibc Packet Flow, this is not well defined")
    }
}

impl<Chain: CwEnv> IndexResponse for NestedPacketsFlow<Chain> {
    fn events(&self) -> Vec<cosmwasm_std::Event> {
        let mut self_events = self.tx_id.response.events();
        let other_events = self
            .packets
            .iter()
            .flat_map(|packet_result| packet_result.events());
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
                    .find_map(|packet_result| {
                        packet_result.event_attr_value(event_type, attr_key).ok()
                    })
                    .ok_or(StdError::generic_err(format!(
                        "event of type {event_type} does not have a value at key {attr_key}"
                    )))
            })
    }

    fn event_attr_values(&self, event_type: &str, attr_key: &str) -> Vec<String> {
        let mut all_results = self.tx_id.response.event_attr_values(event_type, attr_key);

        all_results.extend(
            self.packets
                .iter()
                .flat_map(|packet_result| packet_result.event_attr_values(event_type, attr_key)),
        );

        all_results
    }

    fn data(&self) -> Option<Binary> {
        unimplemented!("No data fields on Ibc Tx Analysis")
    }
}

pub mod success {
    use crate::{ack_parser::polytone_callback::Callback, tx::TxId};
    use cosmwasm_std::{Binary, Empty, StdError};
    use cw_orch_core::environment::CwEnv;
    use cw_orch_core::environment::IndexResponse;

    /// Contains the result (ack success) associated with various Ibc applications
    #[derive(Debug, PartialEq, Clone)]
    pub enum IbcAppResult<CustomResult = Empty> {
        /// Contains a successful result for Polytone
        Polytone(Callback),
        /// Signals a successful result for ICS20 (token transfer)
        Ics20,
        /// Contains a successful result according to the ICS004 standard
        Ics004(Vec<u8>),
        /// Contains a custom result. This is only used if a custom parsing function is specified
        Custom(CustomResult),
    }

    impl IbcAppResult<Empty> {
        /// Casts the Result into a Result with a specified CustomResult type
        pub fn into_custom<CustomResult>(self) -> IbcAppResult<CustomResult> {
            match self {
                IbcAppResult::Polytone(callback) => IbcAppResult::Polytone(callback),
                IbcAppResult::Ics20 => IbcAppResult::Ics20,
                IbcAppResult::Ics004(vec) => IbcAppResult::Ics004(vec),
                IbcAppResult::Custom(_) => unreachable!(),
            }
        }
    }

    /// Success packet outcome. This is the result of a packet analysis.
    /// The T generic is used to allow for raw transactions or analyzed transactions to be used
    #[derive(Debug, PartialEq, Clone)]
    pub struct IbcPacketResult<T, CustomResult = Empty> {
        /// The packets gets transmitted to the dst chain
        pub receive_tx: T,
        /// The ack is broadcasted back on the src chain
        pub ack_tx: T,
        /// The parsed and raw binary acknowledgement retrieved from `ack_tx`
        pub ibc_app_result: IbcAppResult<CustomResult>,
    }

    /// Success Packet Flow. This is the result of a packet analysis.
    ///
    /// This allows identifying the different transactions involved.
    #[derive(Clone)]
    pub struct SuccessSinglePacketFlow<Chain: CwEnv, CustomResult = Empty> {
        /// The transaction during which the packet was sent
        ///
        /// Can optionally be specified, depending on the environment on which the implementation is done
        /// This is not available for the [`Mock`] implementation for instance
        pub send_tx: Option<TxId<Chain>>,
        /// Result of the successful packet flow
        pub result: IbcPacketResult<TxId<Chain, CustomResult>, CustomResult>,
    }

    /// The result of following all packets sent across IBC during a single transaction.
    ///
    /// This structure is nested and will also await all packets emitted during the subsequent (receive/acknowledgement) transactions
    #[derive(Clone)]
    pub struct SuccessNestedPacketsFlow<Chain: CwEnv, CustomResult = Empty> {
        /// Identification of the transaction
        pub tx_id: TxId<Chain>,
        /// Result of following a packet + Recursive Analysis of the resulting transactions for additional IBC packets
        pub packets:
            Vec<IbcPacketResult<SuccessNestedPacketsFlow<Chain, CustomResult>, CustomResult>>,
    }

    impl<T: IndexResponse> IndexResponse for IbcPacketResult<T> {
        fn events(&self) -> Vec<cosmwasm_std::Event> {
            [self.receive_tx.events(), self.ack_tx.events()].concat()
        }

        fn event_attr_value(
            &self,
            event_type: &str,
            attr_key: &str,
        ) -> cosmwasm_std::StdResult<String> {
            self.receive_tx
                .event_attr_value(event_type, attr_key)
                .or_else(|_| self.ack_tx.event_attr_value(event_type, attr_key))
        }

        fn event_attr_values(&self, event_type: &str, attr_key: &str) -> Vec<String> {
            [
                self.receive_tx.event_attr_values(event_type, attr_key),
                self.ack_tx.event_attr_values(event_type, attr_key),
            ]
            .concat()
        }

        fn data(&self) -> Option<Binary> {
            unimplemented!("No data fields on Ibc Packet Flow, this is not well defined")
        }
    }

    impl<Chain: CwEnv> IndexResponse for SuccessSinglePacketFlow<Chain> {
        fn events(&self) -> Vec<cosmwasm_std::Event> {
            let mut events: Vec<_> = self
                .send_tx
                .as_ref()
                .map(|tx| tx.response.events())
                .unwrap_or_default();
            let other_events = self.result.events();
            events.extend(other_events);

            events
        }

        fn event_attr_value(
            &self,
            event_type: &str,
            attr_key: &str,
        ) -> cosmwasm_std::StdResult<String> {
            self.send_tx
                .as_ref()
                .map(|r| r.event_attr_value(event_type, attr_key))
                .and_then(|res| res.ok())
                .or_else(|| self.result.event_attr_value(event_type, attr_key).ok())
                .ok_or(StdError::generic_err(format!(
                    "event of type {event_type} does not have a value at key {attr_key}"
                )))
        }

        fn event_attr_values(&self, event_type: &str, attr_key: &str) -> Vec<String> {
            let mut all_results: Vec<_> = self
                .send_tx
                .as_ref()
                .map(|tx| tx.response.event_attr_values(event_type, attr_key))
                .unwrap_or_default();
            let other_results = self.result.event_attr_values(event_type, attr_key);
            all_results.extend(other_results);

            all_results
        }

        fn data(&self) -> Option<Binary> {
            unimplemented!("No data fields on SuccessSinglePacketFlow, this is not well defined")
        }
    }

    impl<Chain: CwEnv> IndexResponse for SuccessNestedPacketsFlow<Chain> {
        fn events(&self) -> Vec<cosmwasm_std::Event> {
            let mut self_events = self.tx_id.response.events();
            let other_events = self
                .packets
                .iter()
                .flat_map(|packet_result| packet_result.events());
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
                        .find_map(|packet_result| {
                            packet_result.event_attr_value(event_type, attr_key).ok()
                        })
                        .ok_or(StdError::generic_err(format!(
                            "event of type {event_type} does not have a value at key {attr_key}"
                        )))
                })
        }

        fn event_attr_values(&self, event_type: &str, attr_key: &str) -> Vec<String> {
            let mut all_results = self.tx_id.response.event_attr_values(event_type, attr_key);

            all_results.extend(
                self.packets.iter().flat_map(|packet_result| {
                    packet_result.event_attr_values(event_type, attr_key)
                }),
            );

            all_results
        }

        fn data(&self) -> Option<Binary> {
            unimplemented!("No data fields on SuccessNestedPacketsFlow")
        }
    }
}

mod debug {
    use cw_orch_core::environment::CwEnv;

    use super::{
        success::{SuccessNestedPacketsFlow, SuccessSinglePacketFlow},
        NestedPacketsFlow, SinglePacketFlow,
    };

    impl<C: CwEnv> std::fmt::Debug for SinglePacketFlow<C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SinglePacketFlow")
                .field("send_tx", &self.send_tx)
                .field("outcome", &self.outcome)
                .finish()
        }
    }

    impl<C: CwEnv> std::fmt::Debug for NestedPacketsFlow<C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("NestedPacketsFlow")
                .field("tx_id", &self.tx_id)
                .field("packets", &self.packets)
                .finish()
        }
    }

    impl<C: CwEnv, CustomResult: std::fmt::Debug> std::fmt::Debug
        for SuccessSinglePacketFlow<C, CustomResult>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SuccessSinglePacketFlow")
                .field("sent_tx", &self.send_tx)
                .field("result", &self.result)
                .finish()
        }
    }

    impl<C: CwEnv, CustomResult: std::fmt::Debug> std::fmt::Debug
        for SuccessNestedPacketsFlow<C, CustomResult>
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SuccessNestedPacketsFlow")
                .field("tx_id", &self.tx_id)
                .field("packets", &self.packets)
                .finish()
        }
    }
}

mod index_response {
    use cosmwasm_std::Binary;
    use cw_orch_core::environment::IndexResponse;

    use super::IbcPacketOutcome;

    impl<T: IndexResponse> IndexResponse for IbcPacketOutcome<T> {
        fn events(&self) -> Vec<cosmwasm_std::Event> {
            match &self {
                IbcPacketOutcome::Timeout { timeout_tx } => timeout_tx.events(),
                IbcPacketOutcome::Success {
                    receive_tx,
                    ack_tx,
                    ack: _,
                } => [receive_tx.events(), ack_tx.events()].concat(),
            }
        }

        fn event_attr_value(
            &self,
            event_type: &str,
            attr_key: &str,
        ) -> cosmwasm_std::StdResult<String> {
            match &self {
                IbcPacketOutcome::Timeout { timeout_tx } => {
                    timeout_tx.event_attr_value(event_type, attr_key)
                }
                IbcPacketOutcome::Success {
                    receive_tx,
                    ack_tx,
                    ack: _,
                } => receive_tx
                    .event_attr_value(event_type, attr_key)
                    .or_else(|_| ack_tx.event_attr_value(event_type, attr_key)),
            }
        }

        fn event_attr_values(&self, event_type: &str, attr_key: &str) -> Vec<String> {
            match &self {
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
        }

        fn data(&self) -> Option<Binary> {
            unimplemented!("No data fields on Ibc Packet Flow, this is not well defined")
        }
    }
}
