//! Module for tracking a specific packet inside the interchain

use cosmrs::proto::ibc::core::channel::v1::State;
use cw_orch_core::environment::{ChainInfoOwned, ChainState};
use cw_orch_daemon::networks::parse_network;
use cw_orch_daemon::queriers::{Ibc, Node};
use cw_orch_daemon::GrpcChannel;
use cw_orch_daemon::TxResultBlockEvent;
use cw_orch_daemon::{CosmTxResponse, Daemon, DaemonError};
use cw_orch_interchain_core::channel::{IbcPort, InterchainChannel};
use cw_orch_interchain_core::env::ChainId;
use cw_orch_interchain_core::{
    IbcPacketInfo, IbcPacketOutcome, NestedPacketsFlow, SinglePacketFlow, TxId,
};
use futures_util::future::select_all;
use futures_util::FutureExt;

use crate::{IcDaemonResult, InterchainDaemonError};
use cw_orch_interchain_core::results::NetworkId;

use cw_orch_daemon::Channel;
use futures::future::try_join_all;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

use std::collections::HashMap;

/// Environment used to track IBC execution and updates on multiple chains.
/// This can be used to track specific IBC packets or get general information update on channels between multiple chains
/// This struct is safe to be sent between threads
/// In contrary to InterchainStructure that holds Daemon in its definition which is not sendable
#[derive(Default, Clone)]
pub(crate) struct PacketInspector {
    registered_chains: HashMap<NetworkId, Channel>,
}

// / TODO, change this doc comment that is not up to date anymore
// / Follow all IBC packets included in a transaction (recursively).
// / ## Example
// / ```no_run
// /  use cw_orch::prelude::PacketInspector;
// / # tokio_test::block_on(async {
// /  PacketInspector::default()
// /        .await_ibc_execution(
// /             "juno-1".to_string(),
// /             "2E68E86FEFED8459144D19968B36C6FB8928018D720CC29689B4793A7DE50BD5".to_string()
// /         ).await.unwrap();
// / # })
// / ```
impl PacketInspector {
    /// Adds a custom chain to the environment
    /// While following IBC packet execution, this struct will need to get specific chain information from the `chain_id` only
    /// More precisely, it will need to get a gRPC channel from a `chain_id`.
    /// This struct will use the `crate::prelude::networks::parse_network` function by default to do so.
    /// To override this behavior for specific chains (for example for local testing), you can specify a channel for a specific chain_id
    pub async fn new(custom_chains: Vec<&Daemon>) -> IcDaemonResult<Self> {
        let mut env = PacketInspector::default();

        for chain in custom_chains {
            env.registered_chains.insert(
                chain.state().chain_data.chain_id.to_string(),
                chain.channel(),
            );
        }
        Ok(env)
    }

    /// Following the IBC documentation of packets here : https://github.com/CosmWasm/cosmwasm/blob/main/IBC.md
    /// This function retrieves all ibc packets sent out during a transaction and follows them until they are acknoledged back on the sending chain
    ///
    /// 1. Send Packet. The provided transaction hash is used to retrieve all transaction logs from the sending chain.
    ///     In the logs, we can find all details that allow us to identify the transaction in which the packet is received in the remote chain
    ///     These include :
    ///     - The connection_id
    ///     - The destination port
    ///     - The destination channel
    ///     - The packet sequence (to identify a specific packet in the channel)
    ///
    ///     ## Remarks
    ///     - The packet data is also retrieved for logging
    ///     - Multiple packets can be sent out during the same transaction.
    ///         In order to identify them, we assume the order of the events is the same for all events of a single packet.
    ///         Ex: packet_connection = ["connection_id_of_packet_1", "connection_id_of_packet_2"]
    ///         Ex: packet_dst_port = ["packet_dst_port_of_packet_1", "packet_dst_port_of_packet_2"]
    ///     - The chain id of the destination chain is not available directly in the logs.
    ///         However, it is possible to query the node for the chain id of the counterparty chain linked by a connection
    ///
    /// 2. Follow all IBC packets until they are acknowledged on the origin chain
    ///
    /// 3. Scan all encountered transactions along the way for additional IBC packets
    #[async_recursion::async_recursion(?Send)]
    pub async fn wait_ibc(
        &self,
        src_chain: NetworkId,
        tx: CosmTxResponse,
    ) -> IcDaemonResult<NestedPacketsFlow<Daemon>> {
        // 1. Getting IBC related events for the current tx + finding all IBC packets sent out in the transaction
        let grpc_channel1 = self.get_grpc_channel(&src_chain).await?;

        let sent_packets =
            find_ibc_packets_sent_in_tx(src_chain.clone(), grpc_channel1.clone(), tx.clone())
                .await?;

        // 2. We follow the packet history for each packet found inside the transaction
        let ibc_packet_results = try_join_all(
            sent_packets
                .iter()
                .map(|packet| {
                    self.clone().follow_packet(
                        &src_chain,
                        packet.src_port.clone(),
                        packet.src_channel.clone(),
                        &packet.dst_chain_id,
                        packet.sequence,
                    )
                })
                .collect::<Vec<_>>(),
        )
        .await?
        .into_iter()
        .collect::<Vec<_>>();

        let send_tx_id = TxId::new(src_chain.clone(), tx);

        // We follow all results from outgoing packets in the resulting transactions
        let full_results = try_join_all(ibc_packet_results.into_iter().map(|ibc_result| async {
            let txs_to_analyze = match ibc_result.outcome.clone() {
                IbcPacketOutcome::Timeout { timeout_tx } => vec![timeout_tx],
                IbcPacketOutcome::Success {
                    receive_tx, ack_tx, ..
                } => vec![receive_tx, ack_tx],
            };

            let txs_results = try_join_all(
                txs_to_analyze
                    .iter()
                    .map(|tx| {
                        let chain_id = tx.chain_id.clone();
                        let response = tx.response.clone();
                        self.wait_ibc(chain_id.clone(), response)
                    })
                    .collect::<Vec<_>>(),
            )
            .await?;

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

            Ok::<_, InterchainDaemonError>(analyzed_outcome.clone())
        }))
        .await?;

        let tx_identification = NestedPacketsFlow {
            tx_id: send_tx_id.clone(),
            packets: full_results,
        };

        Ok(tx_identification)
    }

    /// Gets the grpc channel associed with a specific `chain_id`
    /// If it's not registered in this struct (using the `add_custom_chain` member), it will query the grpc from the chain regisry (`networks::parse_network` function)
    async fn get_grpc_channel<'a>(&self, chain_id: ChainId<'a>) -> IcDaemonResult<Channel> {
        let grpc_channel = self.registered_chains.get(chain_id);

        if let Some(dst_grpc_channel) = grpc_channel {
            Ok(dst_grpc_channel.clone())
        } else {
            // If no custom channel was registered, we try to get it from the registry
            let chain_data: ChainInfoOwned = parse_network(chain_id).unwrap().into(); // TODO, no unwrap here ?
            Ok(GrpcChannel::connect(&chain_data.grpc_urls, chain_id).await?)
        }
    }

    /// This is a wrapper to follow a packet directly in a single future
    /// Prefer the use of `await_ibc_execution` for following IBC packets related to a transaction
    pub async fn follow_packet<'a>(
        self,
        src_chain: ChainId<'a>,
        src_port: PortId,
        src_channel: ChannelId,
        dst_chain: ChainId<'a>,
        sequence: Sequence,
    ) -> IcDaemonResult<SinglePacketFlow<Daemon>> {
        let src_grpc_channel = self.get_grpc_channel(src_chain).await?;
        let dst_grpc_channel = self.get_grpc_channel(dst_chain).await?;

        // Then we check that the channel indeed exists
        let registered_channel = Ibc::new_async(src_grpc_channel.clone())
            ._channel(src_port.to_string(), src_channel.to_string())
            .await?;

        // We log to warn when the channel state is not right
        if registered_channel.state() != State::Open {
            log::warn!("Channel is not in an open state, the packet will most likely not be relayed. Channel information {:?}", registered_channel);
        }

        let counterparty = registered_channel.counterparty.unwrap();

        // Here the connection id is not used, because a port_id and a channel_id are sufficient to track a channel
        let ibc_channel = InterchainChannel::new(
            IbcPort {
                connection_id: None,
                chain: src_grpc_channel,
                chain_id: src_chain.to_string(),
                port: src_port,
                channel: Some(src_channel),
            },
            IbcPort {
                connection_id: None,
                chain: dst_grpc_channel,
                chain_id: dst_chain.to_string(),
                port: counterparty.port_id.parse().unwrap(),
                channel: Some(counterparty.channel_id.parse().unwrap()),
            },
        );

        // There are 2 possible outcomes for an IBC packet transfer
        // 1. The transfer succeeds, this is covered by the `InterchainChannel::follow_packet_cycle` method
        // 2. The transfer errors and the packet times out. This is covered by the `InterchainChannel::follow_packet_timeout` method
        // If either of those functions succeeds, the other one will never succeeds. That's why we are racing those 2 functions here.

        let (result, _, _) = select_all(vec![
            self.follow_packet_cycle(src_chain, &ibc_channel, sequence)
                .boxed(),
            self.follow_packet_timeout(src_chain, &ibc_channel, sequence)
                .boxed(),
        ])
        .await;

        result
    }

    /// This functions follows an IBC packet on the remote chain and back on its origin chain. It returns all encountered tx hashes
    /// 1. Receive packet. We use the identification of the packet to find the tx in which the packet was received
    ///     We make sure that only one transaction tracks receiving this packet.
    ///         If not, we sent out an error (this error actually comes from the code not identifying an IBC packet properly)
    ///         If such an error happens, it means this function is not implemented properly
    ///         We verify this transaction is not errored (it should never error)
    ///     
    /// 2. Acknowledgment. The last part of the packet lifetime is the acknowledgement the remote chain sents back.
    ///         In the same transaction as the one in which the packet is received, an packet acknowledgement should be sent back to the origin chain
    ///         We get this acknowledgment and deserialize it according to https://github.com/cosmos/cosmos-sdk/blob/v0.42.4/proto/ibc/core/channel/v1/channel.proto#L134-L147
    ///         If the acknowledgement doesn't follow the standard, we don't mind and continue
    /// 3. Identify the acknowledgement receive packet on the origin chain
    ///         Finally, we get the transaction hash of the transaction in which the acknowledgement is received on the origin chain.
    ///         This is also logged for debugging purposes
    ///
    /// We return the tx hash of the received packet on the remote chain as well as the ack packet transaction on the origin chain
    pub async fn follow_packet_cycle<'a>(
        &self,
        from: ChainId<'a>,
        ibc_channel: &'a InterchainChannel<Channel>,
        sequence: Sequence,
    ) -> Result<SinglePacketFlow<Daemon>, InterchainDaemonError> {
        let (src_port, dst_port) = ibc_channel.get_ordered_ports_from(from)?;

        // 0. Query the send tx hash for analysis
        let send_tx = self.get_packet_send_tx(from, ibc_channel, sequence).await?;

        // 1. Query the tx hash on the remote chains related to the packet the origin chain sent
        let received_tx = self
            .get_packet_receive_tx(from, ibc_channel, sequence)
            .await?;
        // We check if the tx errors (this shouldn't happen in IBC connections)
        if received_tx.code != 0 {
            return Err(DaemonError::TxFailed {
                code: received_tx.code,
                reason: format!(
                    "Raw log on {} : {}",
                    dst_port.chain_id,
                    received_tx.raw_log.clone()
                ),
            }
            .into());
        }

        // 2. We get the events related to the acknowledgements sent back on the remote chain

        let all_recv_events = received_tx.get_events("write_acknowledgement");
        let recv_event = all_recv_events
            .iter()
            .filter(|e| {
                e.get_first_attribute_value("packet_sequence").unwrap() == sequence.to_string()
            })
            .collect::<Vec<_>>()[0];

        let recv_packet_sequence = recv_event
            .get_first_attribute_value("packet_sequence")
            .unwrap();
        let recv_packet_data = recv_event.get_first_attribute_value("packet_data").unwrap();
        let acknowledgment = recv_event.get_first_attribute_value("packet_ack").unwrap();

        // We try to unpack the acknowledgement if possible, when it's following the standard format (is not enforced so it's not always possible)
        let decoded_ack_string =
            serde_json::from_str(&acknowledgment).unwrap_or(format!("{:x?}", acknowledgment));

        log::info!(
            target: &dst_port.chain_id,
            "IBC packet n°{} : {}, received on {} on tx {}, with acknowledgment sent back: {}",
            recv_packet_sequence,
            recv_packet_data,
            dst_port.chain_id,
            received_tx.txhash,
            decoded_ack_string
        );

        // 3. We look for the acknowledgement packet on the origin chain
        let ack_tx = self
            .get_packet_ack_receive_tx(&src_port.chain_id, ibc_channel, sequence)
            .await?;
        // First we check if the tx errors (this shouldn't happen in IBC connections)
        if ack_tx.code != 0 {
            return Err(DaemonError::TxFailed {
                code: ack_tx.code,
                reason: format!(
                    "Raw log on {} : {}",
                    src_port.chain_id.clone(),
                    ack_tx.raw_log
                ),
            }
            .into());
        }
        log::info!(
            target: &src_port.chain_id,
            "IBC packet n°{} acknowledgment received on {} on tx {}",
            sequence,
            src_port.chain_id.clone(),
            ack_tx.txhash
        );

        Ok(SinglePacketFlow {
            send_tx: Some(TxId::new(src_port.chain_id.clone(), send_tx)),
            outcome: IbcPacketOutcome::Success {
                receive_tx: TxId::new(dst_port.chain_id.clone(), received_tx),
                ack_tx: TxId::new(src_port.chain_id.clone(), ack_tx),
                ack: acknowledgment.as_bytes().into(),
            },
        })
    }

    /// This functions looks for timeouts of an IBC packet on its origin chain. It returns the tx hash of the timeout tx.
    pub async fn follow_packet_timeout<'a>(
        &self,
        from: ChainId<'a>,
        ibc_channel: &'a InterchainChannel<Channel>,
        sequence: Sequence,
    ) -> Result<SinglePacketFlow<Daemon>, InterchainDaemonError> {
        // 0. Query the send tx hash for analysis
        let send_tx = self.get_packet_send_tx(from, ibc_channel, sequence).await?;

        let (src_port, _dst_port) = ibc_channel.get_ordered_ports_from(from)?;

        // We query the tx hash of the timeout packet on the source chain
        let timeout_tx = self
            .get_packet_timeout_tx(from, ibc_channel, sequence)
            .await?;
        // We check if the tx errors (this shouldn't happen in IBC connections)
        if timeout_tx.code != 0 {
            return Err(DaemonError::TxFailed {
                code: timeout_tx.code,
                reason: format!(
                    "Raw log on {} : {}",
                    src_port.chain_id,
                    timeout_tx.raw_log.clone()
                ),
            }
            .into());
        }

        log::error!(
            target: &src_port.chain_id,
            "IBC packet n° {} :
                port : {}, 
                channel: {} received a timeout and was not broadcasted successfully on tx {}",
            sequence,
            src_port.port,
            src_port.channel.unwrap(),
            timeout_tx.txhash
        );

        // We return the tx hash of this transaction for future analysis
        Ok(SinglePacketFlow {
            send_tx: Some(TxId::new(src_port.chain_id.clone(), send_tx)),
            outcome: IbcPacketOutcome::Timeout {
                timeout_tx: TxId::new(src_port.chain_id.clone(), timeout_tx),
            },
        })
    }

    async fn get_tx_by_events_and_assert_one(
        channel: Channel,
        events: Vec<String>,
    ) -> Result<CosmTxResponse, InterchainDaemonError> {
        let txs = Node::new_async(channel.clone())
            ._find_some_tx_by_events(events, None, None)
            .await?;
        if txs.len() != 1 {
            return Err(DaemonError::ibc_err("Found multiple transactions matching a send packet event, this is impossible (or cw-orch impl is at fault)").into());
        }
        Ok(txs[0].clone())
    }

    // From is the channel from which the send packet has been sent
    pub async fn get_packet_send_tx<'a>(
        &self,
        from: ChainId<'a>,
        ibc_channel: &'a InterchainChannel<Channel>,
        packet_sequence: Sequence,
    ) -> Result<CosmTxResponse, InterchainDaemonError> {
        let (src_port, dst_port) = ibc_channel.get_ordered_ports_from(from)?;

        let send_events_string = vec![
            format!("send_packet.packet_dst_port='{}'", dst_port.port),
            format!(
                "send_packet.packet_dst_channel='{}'",
                dst_port
                    .channel
                    .clone()
                    .ok_or(DaemonError::ibc_err(format!(
                        "No channel registered between {:?} and {:?}",
                        src_port, dst_port
                    )))?
            ),
            format!("send_packet.packet_sequence='{}'", packet_sequence),
        ];

        Self::get_tx_by_events_and_assert_one(src_port.chain, send_events_string).await
    }

    // on is the chain on which the packet will be received
    pub async fn get_packet_receive_tx<'a>(
        &self,
        from: ChainId<'a>,
        ibc_channel: &'a InterchainChannel<Channel>,
        packet_sequence: Sequence,
    ) -> Result<CosmTxResponse, InterchainDaemonError> {
        let (src_port, dst_port) = ibc_channel.get_ordered_ports_from(from)?;

        let receive_events_string = vec![
            format!("recv_packet.packet_dst_port='{}'", dst_port.port),
            format!(
                "recv_packet.packet_dst_channel='{}'",
                dst_port
                    .channel
                    .clone()
                    .ok_or(DaemonError::ibc_err(format!(
                        "No channel registered between {:?} and {:?}",
                        src_port, dst_port
                    )))?
            ),
            format!("recv_packet.packet_sequence='{}'", packet_sequence),
        ];

        Self::get_tx_by_events_and_assert_one(dst_port.chain, receive_events_string).await
    }

    // on is the chain on which the packet will be received
    pub async fn get_packet_timeout_tx<'a>(
        &self,
        from: ChainId<'a>,
        ibc_channel: &'a InterchainChannel<Channel>,
        packet_sequence: Sequence,
    ) -> Result<CosmTxResponse, InterchainDaemonError> {
        let (src_port, dst_port) = ibc_channel.get_ordered_ports_from(from)?;

        let timeout_events_string = vec![
            format!("timeout_packet.packet_dst_port='{}'", dst_port.port),
            format!(
                "timeout_packet.packet_dst_channel='{}'",
                dst_port
                    .channel
                    .clone()
                    .ok_or(DaemonError::ibc_err(format!(
                        "No channel registered between {:?} and {:?}",
                        src_port, dst_port
                    )))?
            ),
            format!("timeout_packet.packet_sequence='{}'", packet_sequence),
        ];

        Self::get_tx_by_events_and_assert_one(src_port.chain, timeout_events_string).await
    }

    // From is the channel from which the original send packet has been sent
    pub async fn get_packet_ack_receive_tx<'a>(
        &self,
        from: ChainId<'a>,
        ibc_channel: &'a InterchainChannel<Channel>,
        packet_sequence: Sequence,
    ) -> Result<CosmTxResponse, InterchainDaemonError> {
        let (src_port, dst_port) = ibc_channel.get_ordered_ports_from(from)?;

        let ack_events_string = vec![
            format!("acknowledge_packet.packet_dst_port='{}'", dst_port.port),
            format!(
                "acknowledge_packet.packet_dst_channel='{}'",
                dst_port
                    .channel
                    .clone()
                    .ok_or(DaemonError::ibc_err(format!(
                        "No channel registered between {:?} and {:?}",
                        src_port, dst_port
                    )))?
            ),
            format!("acknowledge_packet.packet_sequence='{}'", packet_sequence),
        ];

        Self::get_tx_by_events_and_assert_one(src_port.chain, ack_events_string).await
    }
}

fn get_events(events: &[TxResultBlockEvent], attr_name: &str) -> Vec<String> {
    events
        .iter()
        .map(|e| e.get_first_attribute_value(attr_name).unwrap())
        .collect()
}

#[allow(missing_docs)]
pub async fn find_ibc_packets_sent_in_tx(
    chain: NetworkId,
    grpc_channel: Channel,
    tx: CosmTxResponse,
) -> IcDaemonResult<Vec<IbcPacketInfo>> {
    let send_packet_events = tx.get_events("send_packet");
    if send_packet_events.is_empty() {
        return Ok(vec![]);
    }

    let connections = get_events(&send_packet_events, "packet_connection");
    let src_ports = get_events(&send_packet_events, "packet_src_port");
    let src_channels = get_events(&send_packet_events, "packet_src_channel");
    let sequences = get_events(&send_packet_events, "packet_sequence");
    let packet_datas = get_events(&send_packet_events, "packet_data");
    let chain_ids = try_join_all(
        connections
            .iter()
            .map(|c| async {
                Ok::<_, InterchainDaemonError>(
                    Ibc::new_async(grpc_channel.clone())
                        ._connection_client(c.clone())
                        .await?
                        .chain_id,
                )
            })
            .collect::<Vec<_>>(),
    )
    .await?;

    let mut ibc_packets = vec![];
    for i in 0..src_ports.len() {
        // We create the ibcPacketInfo struct
        ibc_packets.push(IbcPacketInfo {
            src_port: src_ports[i].parse()?,
            src_channel: src_channels[i].parse()?,
            sequence: sequences[i].parse()?,
            dst_chain_id: chain_ids[i].clone(),
        });

        // We query the destination ports and channels to log as well
        let ibc = Ibc::new_async(grpc_channel.clone());
        let counterparty = ibc
            ._channel(src_ports[i].clone(), src_channels[i].clone())
            .await?
            .counterparty
            .expect(
                "Unreachable, Channel needs to be open on both sides to be able to send packet! ",
            );

        // We log the packets we follow.
        log::info!(
            target: &chain,
            "IBC packet n° {} :
                src_port : {}, 
                src_channel: {},
                dst_port : {}, 
                dst_channel: {}, 
                data: {}",
            sequences[i],
            src_ports[i],
            src_channels[i],
            counterparty.port_id,
            counterparty.channel_id,
            packet_datas[i]
        );
    }

    Ok(ibc_packets)
}
