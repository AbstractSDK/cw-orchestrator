use cosmrs::proto::ibc::core::channel::v1::State;
use diff::Diff;
use futures_util::future::join_all;
use log::*;

use std::collections::HashSet;
use std::collections::{hash_map::RandomState, HashMap};
use std::{fmt::Display, time::Duration};
use tonic::{async_trait, transport::Channel};

use crate::queriers::{DaemonQuerier, Ibc, Node};

use self::logged_state::LoggedState;

use super::channel::ChannelAccess;

#[derive(derive_builder::Builder)]
pub struct IbcTrackerConfig<S: LoggedState> {
    #[builder(default = "Duration::from_secs(4)")]
    /// Customize the log interval. If not set, the default is 4 seconds.
    pub(crate) log_interval: Duration,
    // #[builder(default = "log::LevelFilter::Info")]
    /// Customize the log level. If not set, the default is `Info`.
    // pub(crate) log_level: log::LevelFilter,
    // #[builder(default)]
    // #[builder(setter(strip_option, into))]

    // /// Customize the log file name. If not set, the chain ID will be used.
    // pub(crate) log_file_name: Option<String>,
    #[builder(default)]
    /// Customize a trackable Ibc state. This could be the received packets on a channel.
    /// This is the state that will be logged when changes are detected
    pub(crate) ibc_state: S,
}

#[async_trait]
pub trait IbcTracker<S: LoggedState>: ChannelAccess + Send + Sync {
    /// Spawn this task in a separate thread.
    /// It will check the block height of the chain and trigger an IBC log when new blocks are produced.
    async fn cron_log(&self, config: IbcTrackerConfig<S>) -> ()
    where
        S: 'async_trait,
    {
        let node = Node::new(self.channel());
        let latest_block = node.block_info().await.unwrap();
        let block_height = latest_block.height;
        let chain_id = latest_block.chain_id;
        // let log_id = config.log_file_name.unwrap_or(chain_id.clone());

        // let encoder = Box::new(PatternEncoder::new(
        //     "{d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n}",
        // ));
        // let file_appender = FileAppender::builder()
        //     .encoder(encoder)
        //     .build(log_file_path)
        //     .unwrap();

        // let log4rs_config = Config::builder()
        //     .appender(Appender::builder().build(&log_id, Box::new(file_appender)))
        //     .build(
        //         Root::builder()
        //             .appender(chain_id.clone())
        //             .build(config.log_level),
        //     )
        //     .unwrap();

        // log4rs::init_config(log4rs_config).unwrap();

        // debug!("Logging started for chain {}", chain_id);

        let mut state = config.ibc_state;
        loop {
            let new_block_height = node.block_info().await.unwrap().height;
            // ensure to only update when a new block is produced
            if new_block_height > block_height {
                state.update_state(self.channel()).await;
                info!(target: &chain_id, "New state: {}", state);
            }
            tokio::time::sleep(config.log_interval).await;
        }
    }
}

impl<S: LoggedState> IbcTracker<S> for Channel {}

mod logged_state {
    use std::fmt::{Debug, Display};

    use diff::Diff;
    use tonic::{async_trait, transport::Channel};

    #[async_trait]
    pub trait LoggedState:
        Debug + PartialEq + Sized + Diff + Default + Display + Send + Sync
    {
        /// Retrieve the new state, is called on every update.
        async fn new_state(&self, channel: Channel) -> Self;
        /// Logs the state, only called when the state has changed.
        async fn log_state(&self, new_self: &Self) {
            let diff = self.diff(new_self);
            let mut changes_to_print = Self::identity();
            changes_to_print.apply(&diff);
            log::info!("New state: {}", new_self);
        }
        /// Top-level function that logs the state if it has changed.
        async fn update_state(&mut self, channel: Channel) {
            let new_state = self.new_state(channel).await;
            if new_state != *self {
                self.log_state(&new_state).await;
            }
            *self = new_state;
        }
    }
}

#[derive(Debug, PartialEq, Default, Diff, Clone)]
/// Store the current state of a contract's IBC connection.
pub struct CwIbcContractState {
    /// Connection over which the contract will establish channels.
    connection_id: String,
    /// The port of the contract "wasm.{contract_address}"
    port_id: String,
    /// The channels connected to the contract
    pub channel_ids: HashSet<String>,
    /// map of the received packets on a channel
    pub received_packets: HashMap<String, HashSet<u64>>,
    /// map of the acknowledged packets on a channel
    pub acknowledged_packets: HashMap<String, HashSet<u64>>,
}

impl CwIbcContractState {
    pub fn new(connection_id: impl ToString, port_id: impl ToString) -> Self {
        Self {
            connection_id: connection_id.to_string(),
            port_id: port_id.to_string(),
            ..Default::default()
        }
    }
}

#[async_trait]
impl LoggedState for CwIbcContractState {
    async fn new_state(&self, channel: Channel) -> Self {
        let ibc = Ibc::new(channel);

        let channels_over_connection = ibc.connection_channels(&self.connection_id).await.unwrap();
        let channel_ids = channels_over_connection
            .into_iter()
            .filter_map(|channel| {
                if channel.state() != State::Open || channel.port_id != self.port_id {
                    None
                } else {
                    Some(channel.channel_id)
                }
            })
            .collect::<Vec<_>>();

        // get the packets received on each channel
        let packets_per_channel =
            join_all(channel_ids.iter().map(|channel_id| {
                ibc.packet_commitments(self.port_id.clone(), channel_id.clone())
            }))
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let received_packets: HashMap<std::string::String, std::vec::Vec<u64>, RandomState> =
            HashMap::from_iter(channel_ids.clone().into_iter().zip(
                packets_per_channel.iter().map(|packets| {
                    packets
                        .iter()
                        .map(|packet| packet.sequence)
                        .collect::<Vec<_>>()
                }),
            ));

        Self {
            connection_id: self.connection_id.clone(),
            port_id: self.port_id.clone(),
            channel_ids,
            received_packets: received_packets,
            acknowledged_packets: self.acknowledged_packets.clone(),
        }
    }
}

impl Display for CwIbcContractState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            acknowledged_packets,
            channel_ids,
            received_packets,
            ..
        } = self;
        if !acknowledged_packets.is_empty() {
            write!(f, "acknowledged_packet(s): {:?}", acknowledged_packets)?;
        }
        if !received_packets.is_empty() {
            write!(f, "received_packet(s): {:?}", acknowledged_packets)?;
        }
        if !channel_ids.is_empty() {
            write!(f, "new_channel(s): {:?}", channel_ids)?;
        }
        Ok(())
    }
}
