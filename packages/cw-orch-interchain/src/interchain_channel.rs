// This struct is used to create and/or track the state of a channel between two chains.
// This is very modular to be able to follow transactions, channel creation...

use base64::engine::general_purpose;
use base64::Engine;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::ChannelId;
use ibc_relayer_types::core::ics24_host::identifier::PortId;
use tokio::time::{sleep, Duration};
use tonic::transport::Channel;

use crate::daemon::CosmTxResponse;
use crate::daemon::DaemonError;
use crate::daemon::queriers::DaemonQuerier;
use crate::daemon::queriers::Node;

use super::interchain_env::NetworkId;

// type is from cosmos_sdk_proto::ibc::core::channel::v1::acknowledgement::Response
// We copy it here to implement serialization for this enum (which is not provided by the proto in the above crate)
#[cosmwasm_schema::cw_serde]
pub enum AckResponse {
    Result(String), // This is a base64 string
    Error(String),
}

#[derive(Debug, Clone)]
pub struct TxId {
    pub chain_id: NetworkId,
    pub channel: Channel,
    pub tx_hash: String,
}

#[derive(Debug, Clone)]
pub struct IbcPort {
    pub chain: Channel,
    pub chain_id: NetworkId,
    pub port: PortId,
    pub channel: Option<ChannelId>,
}

#[derive(Debug)]
pub struct InterchainChannel {
    connection_id: String,
    port_a: IbcPort,
    port_b: IbcPort,
}

// TODO some of those queries may be implemented (or are already implemented) in the IBC querier file ?
impl InterchainChannel {
    pub fn new(connection_id: String, port_a: IbcPort, port_b: IbcPort) -> Self {
        Self {
            connection_id,
            port_a,
            port_b,
        }
    }

    pub fn get_connection(&self) -> String {
        self.connection_id.clone()
    }

    pub fn get_chain(&self, chain_id: NetworkId) -> Result<IbcPort, DaemonError> {
        if chain_id == self.port_a.chain_id {
            Ok(self.port_a.clone())
        } else if chain_id == self.port_b.chain_id {
            Ok(self.port_b.clone())
        } else {
            return Err(DaemonError::ibc_err(format!(
                "chain {}, doesn't exist in the InterchainChannel object {:?}",
                chain_id, self
            )));
        }
    }

    fn get_ordered_ports_from(&self, from: NetworkId) -> Result<(IbcPort, IbcPort), DaemonError> {
        if from == self.port_a.chain_id {
            Ok((self.port_a.clone(), self.port_b.clone()))
        } else if from == self.port_b.chain_id {
            Ok((self.port_b.clone(), self.port_a.clone()))
        } else {
            return Err(DaemonError::ibc_err(format!(
                "chain {}, doesn't exist in the InterchainChannel object {:?}",
                from, self
            )));
        }
    }

    async fn get_tx_by_events_and_assert_one(
        channel: Channel,
        events: Vec<String>,
    ) -> Result<CosmTxResponse, DaemonError> {
        let txs = Node::new(channel.clone())
            .find_some_tx_by_events(events, None, None)
            .await?;
        if txs.len() != 1 {
            return Err(DaemonError::ibc_err("Found multiple transactions matching a send packet event, this is impossible (or cw-orch impl is at fault)"));
        }
        Ok(txs[0].clone())
    }

    // From is the channel from which the send packet has been sent
    pub async fn get_packet_send_tx(
        &self,
        from: NetworkId,
        packet_sequence: Sequence,
    ) -> Result<CosmTxResponse, DaemonError> {
        let (src_port, dst_port) = self.get_ordered_ports_from(from)?;

        let send_events_string = vec![
            format!("send_packet.packet_dst_port='{}'", dst_port.port),
            format!(
                "send_packet.packet_dst_channel='{}'",
                dst_port
                    .channel
                    .clone()
                    .ok_or(DaemonError::ibc_err(format!(
                        "No channel registered between {:?} and {:?} on connection {}",
                        self.port_a, self.port_b, self.connection_id
                    )))?
            ),
            format!("send_packet.packet_sequence='{}'", packet_sequence),
        ];

        Self::get_tx_by_events_and_assert_one(src_port.chain, send_events_string).await
    }

    // on is the chain on which the packet will be received
    pub async fn get_packet_receive_tx(
        &self,
        from: NetworkId,
        packet_sequence: Sequence,
    ) -> Result<CosmTxResponse, DaemonError> {
        let (_src_port, dst_port) = self.get_ordered_ports_from(from)?;

        let receive_events_string = vec![
            format!("recv_packet.packet_dst_port='{}'", dst_port.port),
            format!(
                "recv_packet.packet_dst_channel='{}'",
                dst_port
                    .channel
                    .clone()
                    .ok_or(DaemonError::ibc_err(format!(
                        "No channel registered between {:?} and {:?} on connection {}",
                        self.port_a, self.port_b, self.connection_id
                    )))?
            ),
            format!("recv_packet.packet_sequence='{}'", packet_sequence),
        ];

        Self::get_tx_by_events_and_assert_one(dst_port.chain, receive_events_string).await
    }

    // From is the channel from which the original send packet has been sent
    pub async fn get_packet_ack_receive_tx(
        &self,
        from: NetworkId,
        packet_sequence: Sequence,
    ) -> Result<CosmTxResponse, DaemonError> {
        let (src_port, dst_port) = self.get_ordered_ports_from(from)?;

        let ack_events_string = vec![
            format!("acknowledge_packet.packet_dst_port='{}'", dst_port.port),
            format!(
                "acknowledge_packet.packet_dst_channel='{}'",
                dst_port
                    .channel
                    .clone()
                    .ok_or(DaemonError::ibc_err(format!(
                        "No channel registered between {:?} and {:?} on connection {}",
                        self.port_a, self.port_b, self.connection_id
                    )))?
            ),
            format!("acknowledge_packet.packet_sequence='{}'", packet_sequence),
        ];

        Self::get_tx_by_events_and_assert_one(src_port.chain, ack_events_string).await
    }

    pub async fn get_channel_creation_ack(
        &self,
        from: NetworkId,
    ) -> Result<Vec<CosmTxResponse>, DaemonError> {
        let (src_port, dst_port) = self.get_ordered_ports_from(from)?;

        let channel_creation_events_ack_events = vec![
            format!("channel_open_ack.port_id='{}'", src_port.port), // client is on chain1
            format!("channel_open_ack.counterparty_port_id='{}'", dst_port.port), // host is on chain2
            format!("channel_open_ack.connection_id='{}'", self.connection_id),
        ];
        // Here we just want to query all transactions with events, no other condition
        Node::new(src_port.chain.clone())
            .find_tx_by_events(
                channel_creation_events_ack_events,
                None,
                Some(cosmrs::proto::cosmos::tx::v1beta1::OrderBy::Desc),
            )
            .await
    }

    pub async fn get_channel_creation_confirm(
        &self,
        from: NetworkId,
    ) -> Result<Vec<CosmTxResponse>, DaemonError> {
        let (src_port, dst_port) = self.get_ordered_ports_from(from)?;

        let channel_creation_events_ack_events = vec![
            format!("channel_open_confirm.port_id='{}'", dst_port.port),
            format!(
                "channel_open_confirm.counterparty_port_id='{}'",
                src_port.port
            ), // host is on chain2
               // TODO because
               //format!("channel_open_confirm.connection_id='{}'", self.connection_id),
        ];

        // Here we just want to query all transactions with events, no other condition
        Node::new(dst_port.chain.clone())
            .find_tx_by_events(
                channel_creation_events_ack_events,
                None,
                Some(cosmrs::proto::cosmos::tx::v1beta1::OrderBy::Desc),
            )
            .await
    }

    // We get the last transactions that is related to creating a channel from chain from to the counterparty chain defined in the structure
    pub async fn get_last_channel_creation(
        &self,
        from: NetworkId,
    ) -> Result<(Option<CosmTxResponse>, Option<CosmTxResponse>), DaemonError> {
        let current_channel_creation_a = self
            .get_channel_creation_ack(from.clone())
            .await?
            .get(0)
            .cloned();

        let current_channel_creation_b = self
            .get_channel_creation_confirm(from)
            .await?
            .get(0)
            .cloned();

        Ok((current_channel_creation_a, current_channel_creation_b))
    }

    // We get the last transactions that is related to creating a channel from chain from to the counterparty chain defined in the structure
    pub async fn get_last_channel_creation_hash(
        &self,
        from: NetworkId,
    ) -> Result<(Option<String>, Option<String>), DaemonError> {
        let (current_channel_creation_a, current_channel_creation_b) =
            self.get_last_channel_creation(from).await?;
        Ok((
            current_channel_creation_a.map(|tx| tx.txhash),
            current_channel_creation_b.map(|tx| tx.txhash),
        ))
    }

    pub async fn find_new_channel_creation_tx(
        &self,
        from: NetworkId,
        last_chain_creation_hashes: &(Option<String>, Option<String>),
    ) -> Result<(CosmTxResponse, CosmTxResponse), DaemonError> {
        for _ in 0..5 {
            match self.get_last_channel_creation(from.clone()).await {
                Ok(tx) => {
                    if let Some(ack_tx) = tx.0 {
                        if let Some(confirm_tx) = tx.1 {
                            if ack_tx.txhash
                                != last_chain_creation_hashes
                                    .0
                                    .clone()
                                    .unwrap_or("".to_string())
                                && confirm_tx.txhash
                                    != last_chain_creation_hashes
                                        .1
                                        .clone()
                                        .unwrap_or("".to_string())
                            {
                                return Ok((ack_tx, confirm_tx));
                            }
                        }
                    }
                    log::debug!("No new TX by events found");
                    log::debug!("Waiting 10s");
                    sleep(Duration::from_secs(10)).await;
                }
                Err(e) => {
                    log::debug!("{:?}", e);
                    break;
                }
            }
        }

        Err(DaemonError::AnyError(anyhow::Error::msg(format!(
	        "No new channel creation TX newer than (from_tx_hash: {:?}) or (to_tx_hash: {:?}) found",
	        last_chain_creation_hashes.0, last_chain_creation_hashes.1
	    ))))
    }

    /// This functions follows an IBC packet on the distant chain and back on its origin chain. It returns all encountered tx hashes
    /// 1. Receive packet. We use the identification of the packet to find the tx in which the packet was received
    ///     We make sure that only one transaction tracks receiving this packet.
    ///         If not, we sent out an error (this error actually comes from the code not identifying an IBC packet properly)
    ///         If such an error happens, it means this function is not implemented properly
    ///         We verify this transaction is not errored (it should never error)
    ///     
    /// 2. Acknowledgment. The last part of the packet lifetime is the acknowledgement the distant chain sents back.
    ///         In the same transaction as the one in which the packet is received, an packet acknowledgement should be sent back to the origin chain
    ///         We get this acknowledgment and deserialize it according to https://github.com/cosmos/cosmos-sdk/blob/v0.42.4/proto/ibc/core/channel/v1/channel.proto#L134-L147
    ///         If the acknowledgement doesn't follow the standard, we don't mind and continue
    /// 3. Identify the acknowledgement receive packet on the origin chain
    ///         Finally, we get the transaction hash of the transaction in which the acknowledgement is received on the origin chain.
    ///         This is also logged for debugging purposes
    ///
    /// We return the tx hash of the received packet on the distant chain as well as the ack packet transaction on the origin chain
    pub async fn follow_packet(
        &self,
        from: NetworkId,
        sequence: Sequence,
    ) -> Result<Vec<TxId>, DaemonError> {
        let (src_port, dst_port) = self.get_ordered_ports_from(from.clone())?;

        // 1. Query the tx hash on the distant chains related to the packet the origin chain sent
        let counterparty_grpc_channel = dst_port.chain;

        let received_tx = self.get_packet_receive_tx(from, sequence).await?;
        // We check if the tx errors (this shouldn't happen in IBC connections)
        if received_tx.code != 0 {
            return Err(DaemonError::TxFailed {
                code: received_tx.code,
                reason: format!(
                    "Raw log on {} : {}",
                    dst_port.chain_id,
                    received_tx.raw_log.clone()
                ),
            });
        }

        // 2. We get the events related to the acknowledgements sent back on the distant chain
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
        let parsed_ack: Result<AckResponse, serde_json::Error> =
            serde_json::from_str(&acknowledgment);

        let decoded_ack: String = if let Ok(ack_result) = parsed_ack {
            match ack_result {
                AckResponse::Result(b) => {
                    match std::str::from_utf8(
                        &general_purpose::STANDARD
                            .decode(b.clone())
                            .unwrap_or(vec![]),
                    ) {
                        Ok(d) => format!("Decoded successful ack : {}", d),
                        Err(_) => format!("Couldn't decode following successful ack : {}", b),
                    }
                }
                AckResponse::Error(e) => format!("Ack error : {}", e),
            }
        } else {
            acknowledgment.clone()
        };

        log::info!(
            target: &dst_port.chain_id,
            "IBC packet n°{} : {}, received on {} on tx {}, with acknowledgment sent back: {}",
            recv_packet_sequence,
            recv_packet_data,
            dst_port.chain_id,
            received_tx.txhash,
            decoded_ack
        );

        // 3. We look for the acknowledgement packet on the origin chain
        let ack_tx = self
            .get_packet_ack_receive_tx(src_port.chain_id.clone(), sequence)
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
            });
        }
        log::info!(
            target: &src_port.chain_id,
            "IBC packet n°{} acknowledgment received on {} on tx {}",
            sequence,
            src_port.chain_id.clone(),
            ack_tx.txhash
        );
        Ok(vec![
            TxId {
                chain_id: dst_port.chain_id.clone(),
                channel: counterparty_grpc_channel,
                tx_hash: received_tx.txhash.clone(),
            },
            TxId {
                chain_id: src_port.chain_id.clone(),
                channel: src_port.chain,
                tx_hash: ack_tx.txhash,
            },
        ])
    }
}
