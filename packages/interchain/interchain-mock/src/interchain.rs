#![warn(missing_docs)]

use cosmwasm_std::{from_json, testing::MockApi, Api, Event, IbcOrder};
use cw_orch_core::{environment::QueryHandler, AppResponse};
use cw_orch_interchain_core::{
    channel::InterchainChannel,
    env::{ChainId, ChannelCreation},
    types::{
        ChannelCreationTransactionsResult, FullIbcPacketAnalysis, IbcPacketAnalysis, IbcPacketInfo,
        IbcPacketOutcome, IbcTxAnalysis, InternalChannelCreationResult, SimpleIbcPacketAnalysis,
        TxId,
    },
    InterchainEnv,
};
use cw_orch_mock::{
    cw_multi_test::{
        ibc::{
            relayer::{self, ChannelCreationResult},
            types::{Connection, MockIbcQuery},
        },
        MockApiBech32,
    },
    Mock, MockBech32, MockState,
};
use ibc_relayer_types::core::{
    ics04_channel::packet::Sequence,
    ics24_host::identifier::{ChannelId, PortId},
};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::InterchainMockError;

pub type MockBase<A> = cw_orch_mock::MockBase<A, MockState>;

/// Interchain environment for cw_multi_test Mock environment
/// This leverages Abstract's fork of cw_multi_test enabling IBC interactions
pub struct MockInterchainEnvBase<A: Api> {
    /// Mock chains registered within the structure
    pub mocks: HashMap<String, MockBase<A>>,
}

impl<A: Api> Clone for MockInterchainEnvBase<A> {
    fn clone(&self) -> Self {
        Self {
            mocks: self.mocks.clone(),
        }
    }
}

impl<A: Api> MockInterchainEnvBase<A> {
    /// Create an interchain structure from mocks
    pub fn from_mocks(chains: Vec<MockBase<A>>) -> Self {
        Self {
            mocks: chains
                .iter()
                .map(|d| {
                    let chain_id = d.block_info().unwrap().chain_id;
                    (chain_id, d.clone())
                })
                .collect(),
        }
    }

    /// Adds additional mocks to the interchain environment
    pub fn add_mocks(&mut self, mocks: Vec<MockBase<A>>) {
        self.mocks.extend(
            mocks
                .iter()
                .map(|m| (m.block_info().unwrap().chain_id, m.clone())),
        );
    }
}
type Sender<'a> = &'a str;
type Prefix = &'static str;

impl MockInterchainEnvBase<MockApi> {
    /// Creates a mock environments and associated mock chains from
    /// 1. Chain id
    /// 2. Sender address
    pub fn new(chains: Vec<(ChainId, Sender)>) -> Self {
        // We verify the chain ids are not the same
        let mut uniq = HashSet::new();
        if !chains.iter().all(move |x| uniq.insert(x.0)) {
            panic!("Can't create a mock interchain env with duplicate chain ids");
        }

        Self {
            mocks: chains
                .iter()
                .map(|(chain_id, sender)| {
                    let mock = Mock::new_with_chain_id(sender.to_string(), chain_id);
                    (chain_id.to_string(), mock)
                })
                .collect(),
        }
    }
}

impl MockInterchainEnvBase<MockApiBech32> {
    /// Creates a mock environments and associated mock chains from
    /// 1. Chain id
    /// 2. Chain pub address prefix ("cosmos", "juno", etc.)
    pub fn new(chains: Vec<(ChainId, Prefix)>) -> Self {
        Self {
            mocks: chains
                .iter()
                .map(|(chain_id, prefix)| {
                    let mock = MockBech32::new_with_chain_id(prefix, chain_id);
                    (chain_id.to_string(), mock)
                })
                .collect(),
        }
    }
}

impl<A: Api> InterchainEnv<MockBase<A>> for MockInterchainEnvBase<A> {
    type ChannelCreationResult = ChannelCreationResult;

    type Error = InterchainMockError;

    /// Get the daemon for a network-id in the interchain.
    fn get_chain(&self, chain_id: impl ToString) -> Result<MockBase<A>, InterchainMockError> {
        self.mocks
            .get(&chain_id.to_string())
            .ok_or(InterchainMockError::MockNotFound(chain_id.to_string()))
            .cloned()
    }

    // In a daemon environment, you don't create a channel between 2 chains, instead you just do it with external tools and returns here when the channel is ready
    fn _internal_create_channel(
        &self,
        src_chain: ChainId,
        dst_chain: ChainId,
        src_port: &PortId,
        dst_port: &PortId,
        version: &str,
        order: Option<IbcOrder>,
    ) -> Result<InternalChannelCreationResult<ChannelCreationResult>, Self::Error> {
        if src_chain.eq(dst_chain) {
            panic!("You can't create an interchain connection between the same chain (because of rust borrow mut rules) {}", src_chain);
        }

        // We need to create a channel between the two chains. This is a job for the relayer
        let src_mock = self.get_chain(src_chain)?;
        let dst_mock = self.get_chain(dst_chain)?;

        // We verify that there is a connection between the 2 chains (this requires indexed-map or reverse mapping )
        // We need to specify the connection id no ?
        // We need to register connections if we want to create channels !
        // We connect the first connection
        let connections: Vec<(String, Connection)> = from_json(src_mock.app.borrow().ibc_query(
            MockIbcQuery::ChainConnections {
                chain_id: dst_chain.to_string(),
            },
        )?)?;

        // We verify there is a connection. If there is none, we create one
        let connection_id = if let Some((connection_id, _)) = connections.first() {
            connection_id.clone()
        } else {
            let (src_connection_id, _) = relayer::create_connection(
                &mut src_mock.app.borrow_mut(),
                &mut dst_mock.app.borrow_mut(),
            )?;
            src_connection_id
        };

        let channel_creation = relayer::create_channel(
            &mut src_mock.app.borrow_mut(),
            &mut dst_mock.app.borrow_mut(),
            connection_id.clone(),
            src_port.to_string(),
            dst_port.to_string(),
            version.to_string(),
            order.unwrap_or(IbcOrder::Unordered),
        )?;

        log::info!("Successfully created a channel between {} and {} on '{}:{}' and channels {}:'{}' and {}:'{}'",
            src_port,
            dst_port,
            connection_id,
            "Not specified",
            src_chain,
            channel_creation.src_channel,
            dst_chain,
            channel_creation.dst_channel,
        );

        Ok(InternalChannelCreationResult {
            result: channel_creation,
            src_connection_id: connection_id,
        })
    }

    // This function creates a channel and returns the 4 transactions hashes for channel creation
    fn get_channel_creation_txs(
        &self,
        _src_chain: ChainId,
        _ibc_channel: &mut InterchainChannel<()>,
        channel_creation_result: ChannelCreationResult,
    ) -> Result<ChannelCreationTransactionsResult<MockBase<A>>, Self::Error> {
        let ChannelCreationResult {
            src_channel,
            dst_channel,
            init,
            r#try,
            ack,
            confirm,
        } = channel_creation_result;

        Ok(ChannelCreationTransactionsResult {
            src_channel_id: ChannelId::from_str(&src_channel)?,
            dst_channel_id: ChannelId::from_str(&dst_channel)?,
            channel_creation_txs: ChannelCreation {
                init: init.into(),
                r#try: r#try.into(),
                ack: ack.into(),
                confirm: confirm.into(),
            },
        })
    }

    fn await_packets(
        &self,
        chain_id: ChainId,
        tx_response: impl Into<AppResponse>,
    ) -> Result<IbcTxAnalysis<MockBase<A>>, Self::Error> {
        let tx_response = tx_response.into();
        // We start by analyzing sent packets in the response
        let packets = find_ibc_packets_sent_in_tx(&self.get_chain(chain_id)?, &tx_response)?;

        let send_tx_id = TxId {
            chain_id: chain_id.to_string(),
            response: tx_response,
        };

        let packet_analysis = packets
            .iter()
            .map(|packet| {
                let ibc_result = self.await_single_packet(
                    chain_id,
                    packet.src_port.clone(),
                    packet.src_channel.clone(),
                    &packet.dst_chain_id,
                    packet.sequence,
                )?;

                // for each resulting tx, we analyze them
                let txs_to_analyze = match ibc_result.outcome.clone() {
                    IbcPacketOutcome::Timeout { timeout_tx } => vec![timeout_tx],
                    IbcPacketOutcome::Success {
                        receive_tx, ack_tx, ..
                    } => vec![receive_tx, ack_tx],
                };
                let txs_results = txs_to_analyze
                    .iter()
                    .map(|tx| {
                        let chain_id = tx.chain_id.clone();
                        let response = tx.response.clone();
                        self.await_packets(&chain_id, response)
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let analyzed_outcome = match ibc_result.outcome {
                    IbcPacketOutcome::Timeout { .. } => IbcPacketOutcome::Timeout {
                        timeout_tx: txs_results[0].clone(),
                    },
                    IbcPacketOutcome::Success { ack, .. } => IbcPacketOutcome::Success {
                        ack: ack.clone(),
                        receive_tx: txs_results[0].clone(),
                        ack_tx: txs_results[1].clone(),
                    },
                };

                let analyzed_result = FullIbcPacketAnalysis {
                    send_tx: Some(send_tx_id.clone()),
                    outcome: analyzed_outcome,
                };

                // We return the packet analysis

                Ok(analyzed_result)
            })
            .collect::<Result<Vec<_>, InterchainMockError>>()?;

        let response = IbcTxAnalysis {
            tx_id: send_tx_id,
            packets: packet_analysis,
        };

        Ok(response)
    }

    // This function follow the execution of an IBC packet across the chain
    /// In mock, it also relays the packet
    fn await_single_packet(
        &self,
        src_chain: ChainId,
        src_port: PortId,
        src_channel: ChannelId,
        dst_chain: ChainId,
        sequence: Sequence,
    ) -> Result<SimpleIbcPacketAnalysis<MockBase<A>>, Self::Error> {
        let src_mock = self.get_chain(src_chain)?;
        let dst_mock = self.get_chain(dst_chain)?;

        // We get the packet data from the chain directly
        let relay_result = relayer::relay_packet(
            &mut src_mock.app.borrow_mut(),
            &mut dst_mock.app.borrow_mut(),
            src_port.to_string(),
            src_channel.to_string(),
            sequence.into(),
        )?;

        let outcome = match relay_result.result {
            relayer::RelayingResult::Timeout {
                timeout_tx,
                close_channel_confirm: _,
            } => IbcPacketOutcome::Timeout {
                timeout_tx: TxId {
                    response: timeout_tx.into(),
                    chain_id: src_chain.to_string(),
                },
            },
            relayer::RelayingResult::Acknowledgement { tx, ack } => {
                let ack_string =
                    serde_json::from_slice(ack.as_slice()).unwrap_or(format!("{:x?}", ack));

                log::info!(
                    "IBC packet n°{}, successfully relayed between {} and {}, with acknowledgment sent back: {}",
                    sequence,
                    src_chain,
                    dst_chain,
                    ack_string,
                );
                IbcPacketOutcome::Success {
                    receive_tx: TxId {
                        response: relay_result.receive_tx.into(),
                        chain_id: dst_chain.to_string(),
                    },
                    ack_tx: TxId {
                        response: tx.into(),
                        chain_id: src_chain.to_string(),
                    },
                    ack,
                }
            }
        };

        let analysis_result = IbcPacketAnalysis {
            send_tx: None, // This is not available in this context unfortunately
            outcome,
        };

        Ok(analysis_result)
    }

    fn chains<'a>(&'a self) -> impl Iterator<Item = &'a MockBase<A>>
    where
        MockBase<A>: 'a,
    {
        self.mocks.values()
    }
}

fn get_events(tx: &AppResponse, event: &str) -> Vec<Event> {
    tx.events
        .iter()
        .filter(|e| e.ty == event)
        .cloned()
        .collect()
}
fn get_all_events_values(events: &[Event], attribute: &str) -> Vec<String> {
    events
        .iter()
        .flat_map(|e| {
            e.attributes
                .iter()
                .filter(|a| a.key == attribute)
                .map(|a| a.value.clone())
        })
        .collect()
}

fn find_ibc_packets_sent_in_tx<A: Api>(
    chain: &MockBase<A>,
    tx: &AppResponse,
) -> Result<Vec<IbcPacketInfo>, InterchainMockError> {
    let send_packet_events = get_events(tx, "send_packet");
    if send_packet_events.is_empty() {
        return Ok(vec![]);
    }

    let connections = get_all_events_values(&send_packet_events, "packet_connection");
    let src_ports = get_all_events_values(&send_packet_events, "packet_src_port");
    let src_channels = get_all_events_values(&send_packet_events, "packet_src_channel");
    let sequences = get_all_events_values(&send_packet_events, "packet_sequence");
    let packet_datas = get_all_events_values(&send_packet_events, "packet_data");
    let chain_ids = connections
        .iter()
        .map(|c| {
            let connection: Connection =
                from_json(chain.app.borrow().ibc_query(MockIbcQuery::ConnectedChain {
                    connection_id: c.to_string(),
                })?)?;
            Ok::<_, InterchainMockError>(connection.counterparty_chain_id)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut ibc_packets = vec![];
    for i in 0..src_ports.len() {
        // We create the ibcPacketInfo struct
        ibc_packets.push(IbcPacketInfo {
            src_port: src_ports[i].parse()?,
            src_channel: src_channels[i].parse()?,
            sequence: sequences[i].parse()?,
            dst_chain_id: chain_ids[i].clone(),
        });

        // We log the packets we follow.
        log::info!(
            "IBC packet n° {} :
                port : {},
                channel: {}
                data: {}",
            sequences[i],
            src_ports[i],
            src_channels[i],
            packet_datas[i]
        );
    }

    Ok(ibc_packets)
}
