
use crate::daemon::queriers::{DaemonQuerier, Ibc, Node};
use crate::daemon::GrpcChannel;
use crate::daemon::TxResultBlockEvent;
use crate::daemon::{CosmTxResponse, DaemonError};
use crate::prelude::networks::parse_network;
use crate::interchain_channel::TxId;
use crate::interchain_channel_builder::InterchainChannelBuilder;
use crate::InterchainError;

use anyhow::Result;
use futures::future::try_join_all;
use ibc_chain_registry::chain::ChainData;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ChannelId, PortId};
use tonic::transport::Channel;

use crate::interchain_env::NetworkId;
use std::collections::HashMap;


/// Structure to hold information about a sent packet
pub struct IbcPacketInfo {
    src_port: PortId,
    src_channel: ChannelId,
    sequence: Sequence,
    dst_chain_id: NetworkId,
}

/// Environement used to track IBC execution and updates on multiple chains.
/// This can be used to track specific IBC packets or get general information update on channels between multiple chains
/// This struct is safe to be sent between threads
/// In contrary to InterchainStructure that holds Daemon in its definition which is not sendable
#[derive(Default, Clone)]
pub(crate) struct PacketInspector {
    registered_chains: HashMap<NetworkId, Channel>,
}

/// TODO, change this doc comment that is not up to date anymore
/// Follow all IBC packets included in a transaction (recursively).
/// ## Example
/// ```no_run
///  use cw_orch::prelude::PacketInspector;
/// # tokio_test::block_on(async {
///  PacketInspector::default()
///        .await_ibc_execution(
///             "juno-1".to_string(),
///             "2E68E86FEFED8459144D19968B36C6FB8928018D720CC29689B4793A7DE50BD5".to_string()
///         ).await.unwrap();
/// # })
/// ```
impl PacketInspector {
    /// Adds a custom chain to the environment
    /// While following IBC packet execution, this struct will need to get specific chain information from the `chain_id` only
    /// More precisely, it will need to get a gRPC channel from a `chain_id`.
    /// This struct will use the `crate::prelude::networks::parse_network` function by default to do so.
    /// To override this behavior for specific chains (for example for local testing), you can specify a channel for a specific chain_id
    pub async fn new(custom_chains: &Vec<ChainData>) -> Result<Self> {
        let mut env = PacketInspector::default();

        for chain in custom_chains {
            let grpc_channel = GrpcChannel::connect(&chain.apis.grpc, &chain.chain_id).await?;
            env.registered_chains
                .insert(chain.chain_id.to_string(), grpc_channel);
        }
        Ok(env)
    }

    /// Adds a custom chain to the environment
    /// While following IBC packet execution, this struct will need to get specific chain information from the `chain_id` only
    /// More precisely, it will need to get a gRPC channel from a `chain_id`.
    /// This struct will use the `crate::prelude::networks::parse_network` function by default to do so.
    /// To override this behavior for specific chains (for example for local testing), you can specify a channel for a specific chain_id
    pub fn from_channels(custom_chains: &Vec<(NetworkId, Channel)>) -> Result<Self> {
        let mut env = PacketInspector::default();

        for (chain_id, grpc_channel) in custom_chains {
            env.registered_chains
                .insert(chain_id.to_string(), grpc_channel.clone());
        }
        Ok(env)
    }

    /// Following the IBC documentation of packets here : https://github.com/CosmWasm/cosmwasm/blob/main/IBC.md
    /// This function retrieves all ibc packets sent out during a transaction and follows them until they are acknoledged back on the sending chain
    ///
    /// 1. Send Packet. The provided transaction hash is used to retrieve all transaction logs from the sending chain.
    ///     In the logs, we can find all details that allow us to identify the transaction in which the packet is received in the distant chain
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
    /// 2. Follow all IBC pacjets until they are acknowledged on the origin chain
    ///
    /// 3. Scan all encountered transactions along the way for additional IBC packets
    #[async_recursion::async_recursion]
    pub async fn await_ibc_execution(
        &self,
        chain1: NetworkId,
        packet_send_tx_hash: String,
    ) -> Result<()> {
        // 1. Getting IBC related events for the current tx + finding all IBC packets sent out in the transaction
        let grpc_channel1 = self.get_grpc_channel(&chain1).await;

        let tx = Node::new(grpc_channel1.clone())
            .find_tx(packet_send_tx_hash.clone())
            .await?;

        log::info!(
            target: &chain1,
            "Investigating sent packet events on tx {}",
            packet_send_tx_hash
        );
        let sent_packets =
            find_ibc_packets_sent_in_tx(chain1.clone(), grpc_channel1.clone(), tx).await?;

        // 2. We follow the packet history for each packet found inside the transaction
        let txs_to_follow = try_join_all(
            sent_packets
                .iter()
                .map(|packet| {
                    self.clone().follow_packet(
                        chain1.clone(),
                        packet.src_port.clone(),
                        grpc_channel1.clone(),
                        packet.src_channel.clone(),
                        packet.dst_chain_id.clone(),
                        packet.sequence,
                    )
                })
                .collect::<Vec<_>>(),
        )
        .await?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        // 3. We analyze all the encountered tx hashes for outgoing IBC transactions
        try_join_all(
            txs_to_follow
                .iter()
                .map(|tx| {
                    let chain_id = tx.chain_id.clone();
                    let hash = tx.tx_hash.clone();
                    self.await_ibc_execution(chain_id, hash)
                })
                .collect::<Vec<_>>(),
        )
        .await?;

        Ok(())
    }

    /// Gets the grpc channel associed with a specific `chain_id`
    /// If it's not registered in this struct (using the `add_custom_chain` member), it will query the grpc from the chain regisry (`networks::parse_network` function)
    async fn get_grpc_channel(&self, chain_id: &NetworkId) -> Channel {
        let grpc_channel = self.registered_chains.get(chain_id);

        if let Some(dst_grpc_channel) = grpc_channel {
            dst_grpc_channel.clone()
        } else {
            // If no custom channel was registered, we try to get it from the registry
            let chain_data: ChainData = parse_network(chain_id).unwrap().into(); // TODO, no unwrap here ?
            GrpcChannel::connect(&chain_data.apis.grpc, &ChainId::from_string(chain_id))
                .await
                .unwrap()
        }
    }

    /// This is a wrapper to follow a packet directly in a single future
    /// Only used internally.
    /// Use `await_ibc_execution` for following IBC packets related to a transaction
    async fn follow_packet(
        self,
        src_chain: NetworkId,
        src_port: PortId,
        src_grpc_channel: Channel,
        src_channel: ChannelId,
        dst_chain: NetworkId,
        sequence: Sequence,
    ) -> Result<Vec<TxId>, DaemonError> {
        let dst_grpc_channel = self.get_grpc_channel(&dst_chain).await;

        // That's all we need to generate an InterchainChannel object.
        let interchain_channel = InterchainChannelBuilder::default()
            .chain_a(src_chain.clone())
            .port_a(src_port.clone())
            .grpc_channel_a(src_grpc_channel)
            .chain_b(dst_chain)
            // No need for the port_b here
            .grpc_channel_b(dst_grpc_channel)
            .channel_from(src_channel)
            .await?;

        interchain_channel.follow_packet(src_chain, sequence).await
    }
}

fn get_events(events: &[TxResultBlockEvent], attr_name: &str) -> Vec<String> {
    events
        .iter()
        .map(|e| e.get_first_attribute_value(attr_name).unwrap())
        .collect()
}

async fn find_ibc_packets_sent_in_tx(
    chain: NetworkId,
    grpc_channel: Channel,
    tx: CosmTxResponse,
) -> Result<Vec<IbcPackteInfo>, InterchainError> {
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
                Ok::<_, InterchainError>(
                    Ibc::new(grpc_channel.clone())
                        .connection_client(c.clone())
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
        ibc_packets.push(IbcPackteInfo {
            src_port: src_ports[i].parse()?,
            src_channel: src_channels[i].parse()?,
            sequence: sequences[i].parse()?,
            dst_chain_id: chain_ids[i].clone(),
        });

        // We log the packets we follow.
        log::info!(
            target: &chain,
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
