use log::*;
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use std::path::PathBuf;
use std::time::Duration;
use tonic::async_trait;

use crate::{
    queriers::{DaemonQuerier, Ibc},
    DaemonError, TxHandler,
};

use super::channel::ChannelAccess;

#[async_trait]
pub trait IbcTracker: TxHandler + ChannelAccess {
    /// Spawn this task in a separate thread.
    /// It will check the block height of the chain and trigger an IBC log when new blocks are produced.
    async fn cron_log(&self) -> Result<(), DaemonError> {
        let latest_block = self.block_info().unwrap();
        let block_height = latest_block.height;
        let chain_id = latest_block.chain_id;

        let log_file_path = generate_log_file_path(&chain_id);
        std::fs::create_dir_all(log_file_path.parent().unwrap()).unwrap();

        let encoder = Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n}",
        ));
        let file_appender = FileAppender::builder()
            .encoder(encoder)
            .build(log_file_path)
            .unwrap();

        let config = Config::builder()
            .appender(Appender::builder().build(&chain_id, Box::new(file_appender)))
            .build(
                Root::builder()
                    .appender(&chain_id)
                    .build(log::LevelFilter::Trace),
            )
            .unwrap();

        log4rs::init_config(config).unwrap();

        trace!("detailed tracing info");
        debug!("debug info");
        info!("relevant general info");
        warn!("warning this program doesn't do much");
        error!("error message here");
        loop {
            let new_block_height = self.block_info().unwrap().height;
            if new_block_height > block_height {
                self.log_ibc_events().await;
            }
            // wait for 4 seconds (< block time)
            tokio::time::sleep(Duration::from_secs(4)).await;
        }
    }

    async fn log_ibc_events(&self) -> Result<(), DaemonError> {
        let ibc = Ibc::new(self.channel());
        log::info!("Logging IBC events");

        Ok(())
    }
}

fn generate_log_file_path(chain_id: &str) -> PathBuf {
    let file_name = format!("log_{}.log", chain_id);

    let mut log_path = std::env::current_dir().unwrap();
    log_path.push("logs");
    log_path.push(file_name);

    log_path
}
