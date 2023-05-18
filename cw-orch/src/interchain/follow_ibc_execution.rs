use futures::future::{try_join_all, join_all};
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

pub async fn get_channel(chain_id: String) -> Result<Channel>{

    let mut chains: Vec<ChainData> = vec![parse_network(&chain_id).into()];
    InterchainInfrastructure::configure_networks(&mut chains).await?;

    Ok(DaemonAsync::builder()
        .chain(chains[0].clone())
        .deployment_id("interchain")
        .build().await?.channel()
    )
}


// This was coded thanks to this wonderful guide : https://github.com/CosmWasm/cosmwasm/blob/main/IBC.md

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

#[async_recursion::async_recursion]
pub async fn follow_trail(chain1: String, channel1: Channel, tx_hash: String) -> Result<()>{


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
            get_channel(chain.clone()).await.unwrap()
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
            tokio::spawn(follow_trail(chain_id, counter_party_grpc, hash))
        })
        .collect::<Vec<_>>()
    ).await?.into_iter().collect::<Result<Vec<_>>>()?; // tokio::spawn yields a result, so we need to transform the resulting vec of result to a result of vec
    
    try_join_all(ack_txs.iter().map(|tx| {
        let channel1 = channel1.clone();
        let chain1 = chain1.clone();
        let hash = tx.txhash.clone();
        tokio::spawn(follow_trail(chain1, channel1, hash))
    
    }).collect::<Vec<_>>()
    ).await?.into_iter().collect::<Result<Vec<_>>>()?;

    Ok(())
}
