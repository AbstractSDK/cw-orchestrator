use cosmwasm_std::IbcOrder;
use cw_orch_core::environment::IndexResponse;
use cw_orch_daemon::queriers::{Ibc, Node};
use cw_orch_daemon::{CosmTxResponse, Daemon, DaemonError};
use cw_orch_interchain_core::channel::{IbcPort, InterchainChannel};
use cw_orch_interchain_core::env::{ChainId, ChannelCreation};
use cw_orch_interchain_core::types::{FullIbcPacketAnalysis, IbcPacketOutcome, TxId};
use cw_orch_interchain_core::InterchainEnv;
use cw_orch_interchain_daemon::packet_inspector::find_ibc_packets_sent_in_tx;
use cw_orch_interchain_daemon::packet_inspector::PacketInspector;

use crate::core::HermesRelayer;
use cw_orch_interchain_daemon::InterchainDaemonError;
use old_ibc_relayer_types::core::ics04_channel::packet::Sequence;
use old_ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use tokio::time::sleep;
use tonic::transport::Channel;

use cw_orch_interchain_core::types::{
    ChannelCreationTransactionsResult, IbcTxAnalysis, InternalChannelCreationResult,
    SimpleIbcPacketAnalysis,
};
use cw_orch_interchain_daemon::ChannelCreator;
use futures::future::try_join4;
use std::str::FromStr;
use std::time::Duration;

impl InterchainEnv<Daemon> for HermesRelayer {
    type ChannelCreationResult = ();

    type Error = InterchainDaemonError;

    /// Get the daemon for a network-id in the interchain.
    fn chain(&self, chain_id: impl ToString) -> Result<Daemon, InterchainDaemonError> {
        self.daemons
            .get(&chain_id.to_string())
            .map(|(d, _, _)| d)
            .ok_or(InterchainDaemonError::DaemonNotFound(chain_id.to_string()))
            .cloned()
    }

    // In a daemon environmment, you don't create a channel between 2 chains, instead you just do it with external tools and returns here when the channel is ready
    fn _internal_create_channel(
        &self,
        src_chain: ChainId,
        dst_chain: ChainId,
        src_port: &PortId,
        dst_port: &PortId,
        version: &str,
        order: Option<IbcOrder>,
    ) -> Result<InternalChannelCreationResult<()>, Self::Error> {
        let connection_id =
            self.create_ibc_channel(src_chain, dst_chain, src_port, dst_port, version, order)?;

        Ok(InternalChannelCreationResult {
            result: (),
            src_connection_id: connection_id,
        })
    }

    // This function creates a channel and returns the 4 transactions hashes for channel creation
    fn get_channel_creation_txs(
        &self,
        src_chain: ChainId,
        ibc_channel: &mut InterchainChannel<Channel>,
        _channel_creation_result: (),
    ) -> Result<ChannelCreationTransactionsResult<Daemon>, Self::Error> {
        let (src_port, dst_port) = ibc_channel.get_mut_ordered_ports_from(src_chain)?;

        // We start by getting the connection-id of the counterparty chain
        let connection_end = self.rt_handle.block_on(
            Ibc::new_async(src_port.chain.clone())
                ._connection_end(src_port.connection_id.clone().unwrap()),
        )?;

        dst_port.connection_id = Some(connection_end.unwrap().counterparty.unwrap().connection_id);

        // Then we make sure the channel is indeed created between the two chains
        let channel_creation = self
            .rt_handle
            .block_on(self.find_channel_creation_tx(src_chain, ibc_channel))?;

        let src_channel_id = channel_creation
            .ack
            .event_attr_value("channel_open_ack", "channel_id")?;
        let dst_channel_id = channel_creation
            .confirm
            .event_attr_value("channel_open_confirm", "channel_id")?;

        log::info!("Successfully created a channel between {} and {} on  '{}:{}' and channels {}:'{}'(txhash : {}) and {}:'{}' (txhash : {})", 
            ibc_channel.port_a.port.clone(),
            ibc_channel.port_b.port.clone(),
            ibc_channel.port_a.connection_id.clone().unwrap(),
            ibc_channel.port_b.connection_id.clone().unwrap(),
            ibc_channel.port_a.chain_id.clone(),
            src_channel_id,
            channel_creation.ack.txhash,
            ibc_channel.port_b.chain_id.clone(),
            dst_channel_id,
            channel_creation.confirm.txhash,
        );

        Ok(ChannelCreationTransactionsResult {
            src_channel_id: ChannelId::from_str(&src_channel_id)?,
            dst_channel_id: ChannelId::from_str(&dst_channel_id)?,
            channel_creation_txs: channel_creation,
        })
    }

    // This function follows every IBC packet sent out in a tx result
    fn wait_ibc(
        &self,
        chain_id: ChainId,
        tx_response: CosmTxResponse,
    ) -> Result<IbcTxAnalysis<Daemon>, Self::Error> {
        log::info!(
            target: chain_id,
            "Investigating sent packet events on tx {}",
            tx_response.txhash
        );

        // 1. Getting IBC related events for the current tx + finding all IBC packets sent out in the transaction
        let daemon_1 = self.chain(chain_id)?;
        let grpc_channel1 = daemon_1.channel();

        let sent_packets = daemon_1.rt_handle.block_on(find_ibc_packets_sent_in_tx(
            chain_id.to_string(),
            grpc_channel1.clone(),
            tx_response.clone(),
        ))?;

        // 2. We follow the packet history for each packet found inside the transaction
        let ibc_packet_results = sent_packets
            .iter()
            .map(|packet| {
                self.clone().follow_packet(
                    chain_id,
                    packet.src_port.clone(),
                    packet.src_channel.clone(),
                    &packet.dst_chain_id,
                    packet.sequence,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        let send_tx_id = TxId {
            chain_id: chain_id.to_string(),
            response: tx_response,
        };

        // We follow all results from outgoing packets in the resulting transactions
        let full_results = ibc_packet_results
            .into_iter()
            .map(|ibc_result| {
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
                        self.wait_ibc(&chain_id, response)
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

                Ok::<_, InterchainDaemonError>(analyzed_result.clone())
            })
            .collect::<Result<_, _>>()?;

        let tx_identification = IbcTxAnalysis {
            tx_id: send_tx_id.clone(),
            packets: full_results,
        };

        Ok(tx_identification)
    }

    // This function follow the execution of an IBC packet across the chain
    fn follow_packet(
        &self,
        src_chain: ChainId,
        src_port: PortId,
        src_channel: ChannelId,
        dst_chain: ChainId,
        sequence: Sequence,
    ) -> Result<SimpleIbcPacketAnalysis<Daemon>, Self::Error> {
        // We crate an interchain env object that is safe to send between threads
        let interchain_env = self.rt_handle.block_on(PacketInspector::new(
            self.daemons.values().map(|(d, _, _)| d).collect(),
        ))?;

        // We try to relay the packets using the HERMES relayer

        self.force_packet_relay(
            src_chain,
            src_port.clone(),
            src_channel.clone(),
            dst_chain,
            sequence,
        );

        // We follow the trail
        let ibc_trail = self.rt_handle.block_on(interchain_env.follow_packet(
            src_chain,
            src_port,
            src_channel,
            dst_chain,
            sequence,
        ))?;

        Ok(ibc_trail)
    }
}

impl HermesRelayer {
    /// This function follows every IBC packet sent out in a tx result
    /// This allows only providing the transaction hash when you don't have access to the whole response object
    pub fn wait_ibc_from_txhash(
        &self,
        chain_id: ChainId,
        packet_send_tx_hash: String,
    ) -> Result<IbcTxAnalysis<Daemon>, InterchainDaemonError> {
        let grpc_channel1 = self.chain(chain_id)?.channel();

        let tx = self.rt_handle.block_on(
            Node::new_async(grpc_channel1.clone())._find_tx(packet_send_tx_hash.clone()),
        )?;

        let ibc_trail = self.wait_ibc(chain_id, tx)?;

        Ok(ibc_trail)
    }

    async fn find_channel_creation_tx<'a>(
        &self,
        src_chain: ChainId<'a>,
        ibc_channel: &InterchainChannel<Channel>,
    ) -> Result<ChannelCreation<CosmTxResponse>, InterchainDaemonError> {
        for _ in 0..5 {
            match self.get_last_channel_creation(src_chain, ibc_channel).await {
                Ok(tx) => {
                    if tx.init.is_some()
                        && tx.r#try.is_some()
                        && tx.ack.is_some()
                        && tx.confirm.is_some()
                    {
                        let creation = ChannelCreation {
                            init: tx.init.unwrap(),
                            r#try: tx.r#try.unwrap(),
                            ack: tx.ack.unwrap(),
                            confirm: tx.confirm.unwrap(),
                        };
                        return Ok(creation);
                    }
                    log::debug!("No new TX by events found");
                    log::debug!("Waiting 20s");
                    sleep(Duration::from_secs(20)).await;
                }
                Err(e) => {
                    log::debug!("{:?}", e);
                    break;
                }
            }
        }

        Err(InterchainDaemonError::ChannelCreationEventsNotFound {
            src_chain: src_chain.to_string(),
            channel: ibc_channel.clone(),
        })
    }

    /// Queries  the last transactions that is related to creating a channel from chain from to the counterparty chain defined in the structure
    async fn get_last_channel_creation<'a>(
        &self,
        src_chain: ChainId<'a>,
        ibc_channel: &InterchainChannel<Channel>,
    ) -> Result<ChannelCreation<Option<CosmTxResponse>>, InterchainDaemonError> {
        let (channel_init, channel_try, channel_ack, channel_confirm) = try_join4(
            self.get_channel_creation_init(src_chain, ibc_channel),
            self.get_channel_creation_try(src_chain, ibc_channel),
            self.get_channel_creation_ack(src_chain, ibc_channel),
            self.get_channel_creation_confirm(src_chain, ibc_channel),
        )
        .await?;

        Ok(ChannelCreation::new(
            channel_init,
            channel_try,
            channel_ack,
            channel_confirm,
        ))
    }

    async fn get_channel_creation_init<'a>(
        &self,
        from: ChainId<'a>,
        ibc_channel: &'a InterchainChannel<Channel>,
    ) -> Result<Option<CosmTxResponse>, InterchainDaemonError> {
        let (src_port, dst_port) = ibc_channel.get_ordered_ports_from(from)?;

        let channel_creation_events_init_events = vec![
            format!("channel_open_init.port_id='{}'", src_port.port),
            format!("channel_open_init.counterparty_port_id='{}'", dst_port.port),
            format!(
                "channel_open_init.connection_id='{}'",
                src_port.connection_id.clone().unwrap()
            ),
        ];

        Ok(find_one_tx_by_events(src_port, channel_creation_events_init_events).await?)
    }

    async fn get_channel_creation_try<'a>(
        &self,
        from: ChainId<'a>,
        ibc_channel: &'a InterchainChannel<Channel>,
    ) -> Result<Option<CosmTxResponse>, InterchainDaemonError> {
        let (src_port, dst_port) = ibc_channel.get_ordered_ports_from(from)?;

        let channel_creation_try_events = vec![
            format!("channel_open_try.port_id='{}'", dst_port.port),
            format!("channel_open_try.counterparty_port_id='{}'", src_port.port),
            format!(
                "channel_open_try.connection_id='{}'",
                dst_port.connection_id.clone().unwrap()
            ),
        ];

        log::debug!(
            "Try {} {:?}",
            dst_port.chain_id,
            channel_creation_try_events
        );

        Ok(find_one_tx_by_events(dst_port, channel_creation_try_events).await?)
    }

    async fn get_channel_creation_ack<'a>(
        &self,
        from: ChainId<'a>,
        ibc_channel: &'a InterchainChannel<Channel>,
    ) -> Result<Option<CosmTxResponse>, InterchainDaemonError> {
        let (src_port, dst_port) = ibc_channel.get_ordered_ports_from(from)?;

        let channel_creation_ack_events = vec![
            format!("channel_open_ack.port_id='{}'", src_port.port),
            format!("channel_open_ack.counterparty_port_id='{}'", dst_port.port),
            format!(
                "channel_open_ack.connection_id='{}'",
                src_port.connection_id.clone().unwrap()
            ),
        ];

        Ok(find_one_tx_by_events(src_port, channel_creation_ack_events).await?)
    }

    async fn get_channel_creation_confirm<'a>(
        &self,
        from: ChainId<'a>,
        ibc_channel: &'a InterchainChannel<Channel>,
    ) -> Result<Option<CosmTxResponse>, InterchainDaemonError> {
        let (src_port, dst_port) = ibc_channel.get_ordered_ports_from(from)?;

        let channel_creation_confirm_events = vec![
            format!("channel_open_confirm.port_id='{}'", dst_port.port),
            format!(
                "channel_open_confirm.counterparty_port_id='{}'",
                src_port.port
            ),
            format!(
                "channel_open_confirm.connection_id='{}'",
                dst_port.connection_id.clone().unwrap()
            ),
        ];

        Ok(find_one_tx_by_events(dst_port, channel_creation_confirm_events).await?)
    }
}

async fn find_one_tx_by_events(
    port: IbcPort<Channel>,
    events: Vec<String>,
) -> Result<Option<CosmTxResponse>, DaemonError> {
    let optional_tx = Node::new_async(port.chain.clone())
        ._find_tx_by_events(
            events,
            None,
            Some(cosmrs::proto::cosmos::tx::v1beta1::OrderBy::Desc),
        )
        .await?;

    Ok(optional_tx.first().cloned())
}
