


use crate::daemon::error::DaemonError;
use crate::interchain::interchain_channel::TxId;
use ibc_chain_registry::chain::ChainData;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use crate::daemon::channel::GrpcChannel;
use crate::daemon::tx_resp::TxResultBlockEvent;
use crate::interchain::interchain_channel_builder::InterchainChannelBuilder;
use crate::daemon::queriers::{DaemonQuerier, Node, Ibc};
use crate::prelude::networks::parse_network;
use futures::future::try_join_all;
use anyhow::{Result, bail};
use tonic::transport::Channel;

use crate::daemon::channel::ChannelAccess;
use crate::interchain::infrastructure::NetworkId;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct InterchainEnv{
	registered_chains: HashMap<NetworkId, Channel>,
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

impl InterchainEnv{
	pub fn add_custom_chain(&mut self, chain_id: NetworkId, channel: impl ChannelAccess) -> Result<&mut Self>{
		// We check the chain is not registered yet in the object
		if self.registered_chains.contains_key(&chain_id){
			bail!("You can't register a chain twice in interchain env");
		}
		self.registered_chains.insert(chain_id, channel.channel());
		Ok(self)
	}

	#[async_recursion::async_recursion]
	pub async fn follow_trail(&self, chain1: NetworkId, tx_hash: String) -> Result<()> {
	    // 1. Getting IBC related events for the current tx
	    let grpc_channel1 = self.get_channel(&chain1).await;

	    let tx = Node::new(grpc_channel1.clone()).find_tx(tx_hash.clone()).await?;

	    let send_packet_events = tx.get_events("send_packet");
	    if send_packet_events.is_empty() {
	        return Ok(());
	    }

	    log::info!(
	        target: &chain1,
	        "Investigating sent packet events on tx {}",
	        tx_hash
	    );
	    let connections = get_events(&send_packet_events, "packet_connection");
	    let src_ports = get_events(&send_packet_events, "packet_src_port");
	    let src_channels = get_events(&send_packet_events, "packet_src_channel");
	    let sequences = get_events(&send_packet_events, "packet_sequence");
	    let packet_datas = get_events(&send_packet_events, "packet_data");
	    let chain_ids = try_join_all(
	        connections
	            .iter()
	            .map(|c| async {
	                Ok::<_, anyhow::Error>(
	                    Ibc::new(grpc_channel1.clone())
	                        .connection_client(c.clone())
	                        .await?
	                        .chain_id,
	                )
	            })
	            .collect::<Vec<_>>(),
	    )
	    .await?;

	    // We log the packets we follow.
	    for i in 0..src_ports.len() {
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
	        src_ports
	            .iter()
	            .enumerate()
	            .map(|(i, _)| self.clone().follow_packet(
	            	chain1.clone(),
	            	src_ports[i].clone(),
	            	grpc_channel1.clone(),
	            	src_channels[i].clone(),
	            	chain_ids[i].clone(),
	            	sequences[i].clone()
	            ))
	            .collect::<Vec<_>>(),
	    )
	    .await?
	    .into_iter()
	    .flatten()
	    .collect::<Vec<_>>();

	    // We analyze all the tx hashes for outgoing IBC transactions
	    try_join_all(
	        txs_to_follow
	            .iter()
	            .map(|tx| {
	                let chain_id = tx.chain_id.clone();
	                let hash = tx.tx_hash.clone();
	                self.follow_trail(chain_id, hash)
	            })
	            .collect::<Vec<_>>(),
	    )
	    .await?;

	    Ok(())
	}

	async fn get_channel(&self, chain: &NetworkId) -> Channel{
		let grpc_channel = self.registered_chains.get(chain);

	    if let Some(dst_grpc_channel) = grpc_channel{
	    	dst_grpc_channel.clone()
	    }else{
	    	// If no custom channel was registered, we try to get it from the registry
	    	let chain_data: ChainData = parse_network(chain).into();
	    	GrpcChannel::connect(
	            &chain_data.apis.grpc,
	            &ChainId::from_string(chain),
	        )
	        .await.unwrap()
	    }
	}

	async fn follow_packet(self, src_chain: NetworkId, src_port: String, src_grpc_channel: Channel, src_channel: String, dst_chain: NetworkId, sequence: String) -> Result<Vec<TxId>, DaemonError>{
	    let dst_grpc_channel = self.get_channel(&dst_chain).await;

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

