use cosmrs::proto::ibc::core::channel::v1::State;
use diff::Diff;
use log::*;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::time::Duration;
use std::{collections::HashMap, path::PathBuf};
use tonic::{async_trait, transport::Channel};

use crate::{
    queriers::{DaemonQuerier, Ibc, Node},
    DaemonError,
};

use self::ibc_state::LoggedState;

use super::channel::ChannelAccess;

#[derive(derive_builder::Builder)]
pub struct IbcTrackerConfig<S: LoggedState> {
    #[builder(default = "Duration::from_secs(4)")]
    /// Customize the log interval. If not set, the default is 4 seconds.
    pub(crate) log_interval: Duration,
    #[builder(default = "log::LevelFilter::Info")]
    /// Customize the log level. If not set, the default is `Info`.
    pub(crate) log_level: log::LevelFilter,
    #[builder(default)]
    /// Customize the log file name. If not set, the chain ID will be used.
    pub(crate) log_file_name: Option<String>,
    #[builder(default)]
    /// Customize a trackable Ibc state. This could be the received packets on a channel.
    /// If not set, no IBC state will be tracked.
    pub(crate) ibc_state: Option<S>,
}

#[async_trait(?Send)]
pub trait IbcTracker<S: LoggedState>: ChannelAccess + Send + Sync {
    /// Spawn this task in a separate thread.
    /// It will check the block height of the chain and trigger an IBC log when new blocks are produced.
    async fn cron_log(&self, config: IbcTrackerConfig<S>) -> Result<(), DaemonError> {
        let node = Node::new(self.channel());
        let latest_block = node.block_info().await.unwrap();
        let block_height = latest_block.height;
        let chain_id = latest_block.chain_id;
        let log_id = config.log_file_name.unwrap_or(chain_id);

        let log_file_path = generate_log_file_path(&log_id);
        std::fs::create_dir_all(log_file_path.parent().unwrap()).unwrap();

        let encoder = Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n}",
        ));
        let file_appender = FileAppender::builder()
            .encoder(encoder)
            .build(log_file_path)
            .unwrap();

        let log4rs_config = Config::builder()
            .appender(Appender::builder().build(&log_id, Box::new(file_appender)))
            .build(Root::builder().appender(chain_id).build(config.log_level))
            .unwrap();

        log4rs::init_config(log4rs_config).unwrap();

        loop {
            let new_block_height = node.block_info().await.unwrap().height;
            // ensure to only update when a new block is produced
            if new_block_height > block_height {
                self.log_ibc_events().await?;
            }
            tokio::time::sleep(config.log_interval).await;
        }
    }

    async fn log_ibc_events(&self) -> Result<(), DaemonError> {
        let ibc = Ibc::new(self.channel());
        log::info!("Logging IBC events");
        let connections = ibc
            .open_connections("osmosis-1")
            .await?
            .into_iter()
            .map(|con| con.id)
            .collect::<Vec<_>>();

        log::info!("Osmosis connection: {:?}", connections);

        Ok(())
    }
}

fn generate_log_file_path(chain_id: &str) -> PathBuf {
    let file_name = format!("{}.log", chain_id);

    let mut log_path = std::env::current_dir().unwrap();
    log_path.push("logs");
    log_path.push(file_name);

    log_path
}

impl<S: LoggedState> IbcTracker<S> for Channel {}

mod ibc_state {
    use std::fmt::Debug;

    use diff::Diff;
    use tonic::{async_trait, transport::Channel};

    #[async_trait(?Send)]
    pub trait LoggedState: Debug + PartialEq + Sized + Diff {
        /// Top-level function that logs the state if it has changed.
        async fn update_state(&mut self, channel: Channel) {
            let new_state = self.new_state(channel).await;
            log::trace!("new ibc state: {:?}", new_state);
            if new_state != *self {
                self.log_state(&new_state).await;
            }
            *self = new_state;
        }
        /// Retrieve the new state, is called on every update.
        async fn new_state(&self, channel: Channel) -> Self;
        /// Logs the state, only called when the state has changed.
        async fn log_state(&self, new_self: &Self) {
            let diff = self.diff(new_self);
            log::info!("new ibc state: {:?}", diff.ch);
        }
    }
}

#[derive(Debug, PartialEq, Default, Diff)]
/// Store the current state of a contract's IBC connection.
pub struct CwIbcContractState {
    /// Connection over which the contract will establish channels.
    connection_id: String,
    /// The port of the contract "wasm.{contract_address}"
    port_id: String,
    /// The channels connected to the contract
    pub channel_ids: Vec<String>,
    /// map of the received packets on a channel
    pub received_packets: HashMap<String, u32>,
    /// map of the acknowledged packets on a channel
    pub acknowledged_packets: HashMap<String, u32>,
}

#[async_trait(?Send)]
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

        Self {
            connection_id: self.connection_id.clone(),
            port_id: self.port_id.clone(),
            channel_ids: self.channel_ids.clone(),
            received_packets: self.received_packets.clone(),
            acknowledged_packets: self.acknowledged_packets.clone(),
        }
    }

    async fn log_state(&self) {
        log::info!("new state: {:?}", self);
    }
}
