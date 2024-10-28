use cosmwasm_std::IbcOrder;
use cw_orch_core::environment::{ChainInfoOwned, ChainState, IndexResponse};
use cw_orch_daemon::queriers::{Ibc, Node};
use cw_orch_daemon::{CosmTxResponse, Daemon, DaemonError, RUNTIME};
use cw_orch_interchain_core::channel::{IbcPort, InterchainChannel};
use cw_orch_interchain_core::env::{ChainId, ChannelCreation};
use cw_orch_interchain_core::{InterchainEnv, NestedPacketsFlow, SinglePacketFlow};

use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use tokio::time::sleep;
use tonic::transport::Channel;

use crate::channel_creator::{ChannelCreationValidator, ChannelCreator};
use crate::interchain_log::InterchainLog;
use crate::packet_inspector::PacketInspector;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

use crate::{IcDaemonResult, InterchainDaemonError};

use cw_orch_interchain_core::results::{
    ChannelCreationTransactionsResult, InternalChannelCreationResult, NetworkId,
};
use futures::future::try_join4;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use tokio::runtime::Handle;

/// Represents a set of locally running blockchain nodes and a Hermes relayer.
#[derive(Clone)]
pub struct DaemonInterchain<C: ChannelCreator = ChannelCreationValidator> {
    /// Daemons indexable by network id, i.e. "juno-1", "osmosis-2", ...
    daemons: HashMap<NetworkId, Daemon>,

    channel_creator: C,

    // Allows logging on separate files
    log: Option<InterchainLog>,

    rt_handle: Handle,
}

impl<C: ChannelCreator> DaemonInterchain<C> {
    /// Builds a new [`DaemonInterchain`] instance.
    /// For use with starship, we advise to use [`cw_orch_starship::Starship::interchain_env`] instead
    /// channel_creator allows you to specify an object that is able to create channels
    /// Use [`crate::ChannelCreationValidator`] for manual channel creations.
    pub fn new<T>(chains: Vec<T>, channel_creator: &C) -> IcDaemonResult<Self>
    where
        T: Into<ChainInfoOwned>,
    {
        Self::new_with_runtime(chains, channel_creator, RUNTIME.handle())
    }

    /// Builds a new [`DaemonInterchain`] instance.
    /// For use with starship, we advise to use [`cw_orch_starship::Starship::interchain_env`] instead
    /// channel_creator allows you to specify an object that is able to create channels
    /// Use [`crate::ChannelCreationValidator`] for manual channel creations.
    /// runtime allows you to control the async runtime (for advanced devs)
    pub fn new_with_runtime<T>(
        chains: Vec<T>,
        channel_creator: &C,
        runtime: &Handle,
    ) -> IcDaemonResult<Self>
    where
        T: Into<ChainInfoOwned>,
    {
        let mut env = Self::raw(runtime, channel_creator);

        // We create daemons for each chains
        for chain_data in chains {
            env.build_daemon(runtime, chain_data.into(), None::<String>)?;
        }

        Ok(env)
    }

    /// This creates an interchain environment from existing daemon instances
    /// The `channel_creator` argument will be responsible for creation interchain channel
    /// If using starship, prefer using Starship::interchain_env for environment creation
    pub fn from_daemons(daemons: Vec<Daemon>, channel_creator: &C) -> Self {
        let mut env = Self::raw(&daemons.first().unwrap().rt_handle, channel_creator);
        env.add_daemons(daemons);
        env
    }

    fn raw(rt: &Handle, channel_creator: &C) -> Self {
        Self {
            daemons: HashMap::new(),
            channel_creator: channel_creator.clone(),
            log: None,
            rt_handle: rt.clone(),
        }
    }

    /// Build a daemon from chain data and mnemonic and add it to the current configuration
    fn build_daemon(
        &mut self,
        runtime: &Handle,
        chain_data: ChainInfoOwned,
        mnemonic: Option<impl Into<String>>,
    ) -> IcDaemonResult<()> {
        let mut daemon_builder = Daemon::builder(chain_data);
        let mut daemon_builder = daemon_builder.handle(runtime);

        daemon_builder = if let Some(mn) = mnemonic {
            daemon_builder.mnemonic(mn)
        } else {
            daemon_builder
        };

        // State is shared between daemons, so if a daemon already exists, we use its state
        daemon_builder = if let Some(daemon) = self.daemons.values().next() {
            daemon_builder.state(daemon.state())
        } else {
            daemon_builder
        };

        let daemon = daemon_builder.build().unwrap();

        self.add_daemons(vec![daemon]);

        Ok(())
    }

    /// Enables logging on multiple files to separate chains from each other
    pub fn with_log(&mut self) {
        let log = InterchainLog::default();
        self.add_to_log(self.daemons.values().cloned().collect());
        self.log = Some(log)
    }

    /// Add already constructed daemons to the environment
    pub fn add_daemons(&mut self, daemons: Vec<Daemon>) {
        self.daemons.extend(
            daemons
                .iter()
                .map(|d| (d.state().chain_data.chain_id.to_string(), d.clone())),
        );

        self.add_to_log(daemons)
    }

    // Adds the daemon to the log environment
    fn add_to_log(&mut self, daemons: Vec<Daemon>) {
        if let Some(log) = self.log.as_mut() {
            log.add_chains(
                &daemons
                    .iter()
                    .map(|d| d.state().chain_data.chain_id.to_string())
                    .collect(),
            )
        }
    }
}

impl<C: ChannelCreator> InterchainEnv<Daemon> for DaemonInterchain<C> {
    type ChannelCreationResult = ();

    type Error = InterchainDaemonError;

    /// Get the daemon for a network-id in the interchain.
    fn get_chain(&self, chain_id: impl ToString) -> Result<Daemon, InterchainDaemonError> {
        self.daemons
            .get(&chain_id.to_string())
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
        let connection_id = self
            .channel_creator
            .create_ibc_channel(src_chain, dst_chain, src_port, dst_port, version, order)?;

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
    fn await_packets(
        &self,
        chain_id: ChainId,
        tx_response: impl Into<CosmTxResponse>,
    ) -> Result<NestedPacketsFlow<Daemon>, Self::Error> {
        let tx_response = tx_response.into();
        log::info!(
            target: chain_id,
            "Investigating sent packet events on tx {}",
            tx_response.txhash
        );

        // We crate an interchain env object that is safe to send between threads
        let interchain_env = self
            .rt_handle
            .block_on(PacketInspector::new(self.daemons.values().collect()))?;

        // We follow the trail
        let ibc_trail = self
            .rt_handle
            .block_on(interchain_env.wait_ibc(chain_id.to_string(), tx_response))?;

        Ok(ibc_trail)
    }

    // This function follow the execution of an IBC packet across the chain
    fn await_single_packet(
        &self,
        src_chain: ChainId,
        src_port: PortId,
        src_channel: ChannelId,
        dst_chain: ChainId,
        sequence: Sequence,
    ) -> Result<SinglePacketFlow<Daemon>, Self::Error> {
        // We crate an interchain env object that is safe to send between threads
        let interchain_env = self
            .rt_handle
            .block_on(PacketInspector::new(self.daemons.values().collect()))?;

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
    fn chains<'a>(&'a self) -> impl Iterator<Item = &'a Daemon>
    where
        Daemon: 'a,
    {
        self.daemons.values()
    }
}

impl<C: ChannelCreator> DaemonInterchain<C> {
    /// This function follows every IBC packet sent out in a tx result
    /// This allows only providing the transaction hash when you don't have access to the whole response object
    ///
    /// ```rust,no_run
    /// use cw_orch::prelude::*;
    /// use cw_orch::daemon::networks::{OSMOSIS_1, ARCHWAY_1};
    /// use cw_orch_interchain::prelude::*;
    ///
    /// let dst_chain = ARCHWAY_1;
    /// let src_chain = OSMOSIS_1;
    ///
    /// let interchain = DaemonInterchain::new(
    ///     vec![src_chain.clone(), dst_chain],
    ///     &ChannelCreationValidator,
    /// ).unwrap();
    ///
    /// interchain
    ///     .await_packets_for_txhash(
    ///         src_chain.chain_id,
    ///         "D2C5459C54B394C168B8DFA214670FF9E2A0349CCBEF149CF5CB508A5B3BCB84".to_string(),
    ///     ).unwrap().assert().unwrap();
    /// ```
    pub fn await_packets_for_txhash(
        &self,
        chain_id: ChainId,
        packet_send_tx_hash: String,
    ) -> Result<NestedPacketsFlow<Daemon>, InterchainDaemonError> {
        let grpc_channel1 = self.get_chain(chain_id)?.channel();

        let tx = self.rt_handle.block_on(
            Node::new_async(grpc_channel1.clone())._find_tx(packet_send_tx_hash.clone()),
        )?;

        let ibc_trail = self.await_packets(chain_id, tx)?;

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
