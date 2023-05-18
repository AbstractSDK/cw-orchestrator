use futures::future::{try_join_all, join_all};
use futures::Future;
use tonic::transport::Channel;
use base64::{engine::general_purpose, Engine as _};
use anyhow::{bail, Result};


use cosmwasm_std::StdError;
use ibc_chain_registry::chain::{ChainData};

use crate::{
    CosmTxResponse, DaemonError, InterchainInfrastructure,
    daemon::
        DaemonAsync,
    queriers::{
        DaemonQuerier, Ibc, Node
    },
    networks::parse_network,
};

use super::IcResult;

pub async fn get_channel(chain_id: String, configure_local_network: Option<bool>) -> Result<Channel>{

    let mut chains: Vec<ChainData> = vec![parse_network(&chain_id).into()];
    if configure_local_network.unwrap_or(false) {
        InterchainInfrastructure::configure_networks(&mut chains).await?;
    }

    Ok(DaemonAsync::builder()
        .chain(chains[0].clone())
        .build().await?.channel()
    )
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
/// ```no_run,ignore
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
pub async fn follow_trail(chain1: String, channel1: Channel, tx_hash: String, configure_local_network: Option<bool>) -> Result<()> 
{


    // 1. Getting IBC related events for the current tx
    let tx = Node::new(channel1.clone())
        .find_tx_with_retries(tx_hash.clone(), 3).await?;


    let send_packet_events = tx.get_events("send_packet");
    if send_packet_events.is_empty() {
        return Ok(());
    }
    log::info!("Investigating sent packet events on tx {}", tx_hash);

    let connections: Vec<String> = send_packet_events
        .iter()
        .map(|e| e.get_first_attribute_value("packet_connection").unwrap())
        .collect();
    let dest_ports: Vec<String> = send_packet_events
        .iter()
        .map(|e| e.get_first_attribute_value("packet_dst_port").unwrap())
        .collect();
    let dest_channels: Vec<String> = send_packet_events
        .iter()
        .map(|e| e.get_first_attribute_value("packet_dst_channel").unwrap())
        .collect();
    let sequences: Vec<String> = send_packet_events
        .iter()
        .map(|e| e.get_first_attribute_value("packet_sequence").unwrap())
        .collect();
    let packet_datas: Vec<String> = send_packet_events
        .iter()
        .map(|e| e.get_first_attribute_value("packet_data").unwrap())
        .collect();

    // 2. Query the tx hashes on the distant chains related to the packets the origin chain sent
    let events_strings = connections.iter().enumerate().map(|(i, _ )| {
        log::info!(
            "IBC packet n° {}, sent on {} on tx {}, with data: {}",
            sequences[i],
            chain1,
            tx_hash,
            packet_datas[i]
        );

        vec![
        format!("recv_packet.packet_connection='{}'", connections[i]),
        format!("recv_packet.packet_dst_port='{}'", dest_ports[i]),
        format!("recv_packet.packet_dst_channel='{}'", dest_channels[i]),
        format!("recv_packet.packet_sequence='{}'", sequences[i]),
    ]});


    let chain_ids: Vec<String> = try_join_all(
        connections.iter().map(|c| async{
            Ok::<_, anyhow::Error>(Ibc::new(channel1.clone()).connection_client(c.clone()).await?.chain_id)
        })
        .collect::<Vec<_>>()
    ).await?;

    let counter_party_grpc_channels: Vec<Channel> = 
        join_all(chain_ids.iter().map(|chain| async{
            get_channel(chain.clone(), configure_local_network).await.unwrap()
        })
    ).await;

    let received_txs: Vec<CosmTxResponse> = try_join_all(
            events_strings.enumerate().map(|(i,event_query)| {
            let this_counter_part_channel = counter_party_grpc_channels[i].clone();
            let this_chain_id = chain_ids[i].clone();
            async move {


            let txs = Node::new(this_counter_part_channel)
                .find_tx_by_events(event_query, None, None)
                .await
                .unwrap();

            // We need to make sure there is only 1 transaction with such events (always should be the case)
            if txs.len() != 1 {
                bail!(StdError::generic_err(
                    "Found multiple transactions matching the events, not possible"
                ));
            }
            let received_tx = &txs[0];
            // We check if the tx errors (this shouldn't happen in IBC connections)
            if received_tx.code != 0 {
                bail!(DaemonError::TxFailed {
                    code: received_tx.code,
                    reason: format!(
                        "Raw log on {} : {}",
                        this_chain_id,
                        received_tx.raw_log.clone()
                    ),
                });
            }
            Ok(received_tx.clone())
        }}).collect::<Vec<_>>()
    ).await?;

    // 3. We query the acknowledgements packets on the origin chain
    let ack_txs: Vec<CosmTxResponse> = try_join_all(
        received_txs.iter().enumerate().map(|(i, received_tx)|{
            let this_connection = connections[i].clone();
            let this_dest_channel = dest_channels[i].clone();
            let this_dest_port = dest_ports[i].clone();
            let this_sequence = sequences[i].clone();
            let this_counter_party_chain_id = chain_ids[i].clone();

            let channel1 = channel1.clone();
            let chain1 = chain1.clone();


            async move{
            // a. We get the events related to the acknowledgements sent back on the distant chain
            let recv_packet_sequence = received_tx.get_events("write_acknowledgement")[0] // There is only one acknowledgement per transaction possible
                .get_first_attribute_value("packet_sequence")
                .unwrap();
            let recv_packet_data = received_tx.get_events("write_acknowledgement")[0]
                .get_first_attribute_value("packet_data")
                .unwrap();
            let acknowledgment = received_tx.get_events("write_acknowledgement")[0]
                .get_first_attribute_value("packet_ack")
                .unwrap();

            // We try to unpack the acknowledgement if possible, when it's following the standard format (is not enforced so it's not always possible)
            let parsed_ack: Result<AckResponse, serde_json::Error> = serde_json::from_str(&acknowledgment);

            let decoded_ack: String = if let Ok(ack_result) = parsed_ack {
                match ack_result {
                    AckResponse::Result(b) => {
                        format!(
                            "Decoded successful ack : {}",
                            std::str::from_utf8(&general_purpose::STANDARD.decode(b)?)?
                        )
                    }
                    AckResponse::Error(e) => format!("Ack error : {}", e),
                }
            } else {
                acknowledgment.clone()
            };

            
            log::info!(
                "IBC packet n°{} : {}, received on {} on tx {}, with acknowledgment sent back: {}",
                recv_packet_sequence,
                recv_packet_data,
                this_counter_party_chain_id,
                received_tx.txhash,
                decoded_ack
            );


            // b. We look for the acknowledgement packet on the origin chain
            let ack_events_string = vec![
                format!("acknowledge_packet.packet_connection='{}'", this_connection),
                format!("acknowledge_packet.packet_dst_port='{}'", this_dest_port),
                format!("acknowledge_packet.packet_dst_channel='{}'", this_dest_channel),
                format!("acknowledge_packet.packet_sequence='{}'", this_sequence),
            ];
            let txs = Node::new(channel1)
                .find_tx_by_events(ack_events_string, None, None)
                .await
                .unwrap();

            if txs.len() != 1 {
                bail!(StdError::generic_err(
                    "Found multiple transactions matching the events, not possible"
                ));
            }
            let ack_tx = &txs[0];
            // First we check if the tx errors (this shouldn't happen in IBC connections)
            if ack_tx.code != 0 {
                bail!(DaemonError::TxFailed {
                    code: ack_tx.code,
                    reason: format!(
                        "Raw log on {} : {}",
                        chain1.clone(),
                        ack_tx.raw_log.clone()
                    ),
                })
            }
            log::info!(
                "IBC packet n°{} acknowledgment received on {} on tx {}",
                this_sequence,
                chain1,
                ack_tx.txhash
            );

            Ok(ack_tx.clone())
        }})
        .collect::<Vec<_>>()
    ).await?;
    
    // All the tx hashes should now should also be analyzed for outgoing IBC transactions
    try_join_all(
        received_txs.iter().enumerate().map(|(i,tx)| {
            let counter_party_grpc = counter_party_grpc_channels[i].clone();
            let chain_id = chain_ids[i].clone();
            let hash = tx.txhash.clone();
            tokio::spawn(follow_trail(chain_id, counter_party_grpc, hash, None))
        })
        .collect::<Vec<_>>()
    ).await?.into_iter().collect::<Result<Vec<_>>>()?; // tokio::spawn yields a result, so we need to transform the resulting vec of result to a result of vec
    
    try_join_all(ack_txs.iter().map(|tx| {
        let channel1 = channel1.clone();
        let chain1 = chain1.clone();
        let hash = tx.txhash.clone();
        tokio::spawn(follow_trail(chain1, channel1, hash, None))
    
    }).collect::<Vec<_>>()
    ).await?.into_iter().collect::<Result<Vec<_>>>()?;

    Ok(())
}
