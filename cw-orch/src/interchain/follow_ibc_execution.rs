use futures::future::{try_join_all};

use anyhow::{Result};

use tonic::transport::Channel;


use ibc_chain_registry::chain::ChainData;

use crate::{
    daemon::{channel::GrpcChannel, tx_resp::TxResultBlockEvent},
    networks::parse_network,
    queriers::{DaemonQuerier, Ibc, Node}, InterchainInfrastructure, interchain::interchain_channel_builder::InterchainChannelBuilder,
};

pub async fn get_channel(
    chain_id: String
) -> Result<Channel> {
    let mut chains: Vec<ChainData> = vec![parse_network(&chain_id).into()];

    InterchainInfrastructure::configure_networks(&mut chains).await?;
    

    Ok(GrpcChannel::connect(&chains[0].apis.grpc, &chains[0].chain_id).await?)
}

// type is from cosmos_sdk_proto::ibc::core::channel::v1::acknowledgement::Response
// We copy it here to implement serialization for this enum (which is not provided by the proto in the above crate)
#[cosmwasm_schema::cw_serde]
pub enum AckResponse {
    Result(String), // This is a base64 string
    Error(String),
}

// In this function, we need to :
// 1. Get all ibc outgoing messages from the transaction
// attribute type : send_packet
// Things to track
// connection
// dest-port
// dest_channel
// packet_sequence
// timeout_timestamp (for stopping the search) - Not needed here

// 2.  For each message find the transaction hash of the txs the message during which the message is broadcasted to the distant chain
// This only works for 2 chains for now, we don't handle more chains

// 3. Then we look for the acknowledgment packet that should always be traced back during this transaction for all packets

/// Follow all IBC packets included in a transaction (recursively).
/// ## Example
/// ```no_run 
///  use cw_orch::prelude::{DaemonAsync};
///
///  let grpc_channel = DaemonAsync::builder()
///     .chain("juno-1")
///     .build().await.unwrap()
///     .channel().unwrap();
///
/// follow_trail(
///         "juno-1".to_string()
///         grpc_channel,
///         "2E68E86FEFED8459144D19968B36C6FB8928018D720CC29689B4793A7DE50BD5".to_string()
/// ).await.unwrap();
/// ```

/// Following the IBC documentation of packets here : https://github.com/CosmWasm/cosmwasm/blob/main/IBC.md
/// This function has 3 steps, needed to follow the lifetime of an IBC packet :
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
/// 2. Receive packet. For each packet received, we use the identification of the packet that was sent in the original tx to find the tx in which the packet was received
///     We make sure that only one transaction tracks receiving this packet.
///         If not, we sent out an error (this error actually comes from the code not identifying an IBC packet properly)
///         If such an error happens, it means this function is not implemented properly
///         We verify this transaction is not errored (it should never error)
///     
/// 3. Acknowledgment. The last part of the packet lifetime is the acknowledgement the distant chain sents back.
///     a. Identify acknowledgement
///         In the same transaction as the one in which the packet is received, an packet acknowledgement should be sent back to the origin chain
///         We get this acknowledgment and deserialize it according to https://github.com/cosmos/cosmos-sdk/blob/v0.42.4/proto/ibc/core/channel/v1/channel.proto#L134-L147
///         If the acknowledgement doesn't follow the standard, we don't mind and continue
///     b. Identify the acknowledgement receive packet on the origin chain
///         Finally, we get the transaction hash of the transaction in which the acknowledgement is received on the origin chain.
///         This is also logged for debugging purposes
///
/// 4. Finally, some additionnal packets may have been sent out during the whole process. We need to check the generated transactions for additional packets.
///    This 4th step make the whole process recursive. Those two transactions potentially sends out additional packets !
///         - The receive Packet transaction identified in 2. on the distant chain
///         - The receive Acknowledgement transaction identified in 3.b. on the origin chain

#[async_recursion::async_recursion]
pub async fn follow_trail(
    chain1: String,
    channel1: Channel,
    tx_hash: String,
) -> Result<()> {
    // 1. Getting IBC related events for the current tx
    let tx = Node::new(channel1.clone()).find_tx(tx_hash.clone()).await?;

    let send_packet_events = tx.get_events("send_packet");
    if send_packet_events.is_empty() {
        return Ok(());
    }

    log::info!(target: &chain1, "Investigating sent packet events on tx {}", tx_hash);
    let connections = get_events(&send_packet_events, "packet_connection");
    let src_ports = get_events(&send_packet_events, "packet_src_port");
    let src_channels = get_events(&send_packet_events, "packet_src_channel");
    let sequences = get_events(&send_packet_events, "packet_sequence");
    let packet_datas = get_events(&send_packet_events, "packet_data");
    let chain_ids = try_join_all(
        connections.iter().map(|c| async {
            Ok::<_, anyhow::Error>(
                Ibc::new(channel1.clone())
                    .connection_client(c.clone())
                    .await?
                    .chain_id,
            )
        })
        .collect::<Vec<_>>(),
    )
    .await?;

    // We log the packets we follow.
    for i in 0..src_ports.len(){
         log::info!(
            target: &chain1,
            "IBC packet n° {}, sent on {} on tx {}, with data: {}",
            sequences[i],
            chain1,
            tx_hash,
            packet_datas[i]
        );
    }

    // We follow the IBC trail for all packets found inside the transaction
    let txs_to_follow = try_join_all(
        src_ports.iter().enumerate().map(|(i,_)| {
            let chain_a = chain1.clone();
            let port_a = src_ports[i].clone();
            let channel_a = src_channels[i].clone();

            let chain_b = chain_ids[i].clone();

            let sequence = sequences[i].clone();

            async move {

            // That's all we need to generate an InterchainChannel object.
            let interchain_channel = InterchainChannelBuilder::default()
                .chain_a(chain_a.clone())
                .port_a(port_a)
                .is_local_chain_a()

                .chain_b(chain_b)
                .is_local_chain_b()
                
                .channel_from(channel_a).await?;

            interchain_channel.follow_packet(chain_a, sequence).await
        }}).collect::<Vec<_>>()
    ).await?.into_iter().flatten().collect::<Vec<_>>();

    // We analyze all the tx hashes for outgoing IBC transactions
    try_join_all(
        txs_to_follow
            .iter()
            .map(|tx| {
                let chain_id = tx.chain_id.clone();
                let counter_party_grpc = tx.channel.clone();
                let hash = tx.tx_hash.clone();
                tokio::spawn(follow_trail(chain_id, counter_party_grpc, hash))
            })
            .collect::<Vec<_>>(),
    )
    .await?
    .into_iter()
    .collect::<Result<Vec<_>>>()?; // tokio::spawn yields a result, so we need to transform the resulting vec of result to a result of vec

    Ok(())
}


fn get_events(events: &[TxResultBlockEvent], attr_name: &str) -> Vec<String>{
    events
        .iter()
        .map(|e| e.get_first_attribute_value(attr_name).unwrap())
        .collect()
}