//! This allows to log chain specific logs to separate files
//! This is incompatible with env_logger. Don't use them simultaneously or you will get errors at runtime
use std::path::PathBuf;

use log::LevelFilter;
use log4rs::{
    append::file::FileAppender,
    config::{runtime::ConfigBuilder, Appender, Logger, Root},
    encode::pattern::PatternEncoder,
    Config,
};

/// Store the log configuration of the daemon environment
#[derive(Clone)]
pub struct InterchainLog {
    handle: log4rs::Handle,
    chain_ids: Vec<String>,
}

impl InterchainLog {
    fn get_encoder() -> Box<PatternEncoder> {
        Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n}",
        ))
    }

    fn builder() -> ConfigBuilder {
        let encoder = InterchainLog::get_encoder();
        let main_log_path = generate_log_file_path("main");
        std::fs::create_dir_all(main_log_path.parent().unwrap()).unwrap();

        let main_appender = FileAppender::builder()
            .encoder(encoder)
            .build(&main_log_path)
            .unwrap();
        Config::builder().appender(Appender::builder().build("main", Box::new(main_appender)))
    }

    fn build_logger(config: ConfigBuilder) -> log4rs::Config {
        config
            .build(Root::builder().appender("main").build(LevelFilter::Info))
            .unwrap()
    }

    fn add_logger(&self, config: ConfigBuilder, chain_id: String) -> ConfigBuilder {
        // We create the log file and register in the log config
        let log_path = generate_log_file_path(&chain_id);
        let daemon_appender = FileAppender::builder()
            .encoder(InterchainLog::get_encoder())
            .build(log_path)
            .unwrap();

        config
            .appender(Appender::builder().build(&chain_id, Box::new(daemon_appender)))
            .logger(
                Logger::builder()
                    .appender(&chain_id)
                    .build(&chain_id, LevelFilter::Info),
            )
    }

    /// Initiates an interchain log setup
    /// This will log the different chain interactions and updates on separate files for each chain.
    /// This is useful for tracking operations happenning on IBC chains
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds chains to log for in the environment.
    /// This resets logging for the whole structure to be able to add a chain to the config
    pub fn add_chains(&mut self, chain_ids: &Vec<String>) {
        // We restart the config with the older builders
        let mut config_builder = InterchainLog::builder();
        for chain_id in &self.chain_ids {
            config_builder = self.add_logger(config_builder, chain_id.to_string());
        }

        // And then we add the new builders
        for chain_id in chain_ids {
            // We verify the log setup is not already created for the chain id
            // We silently continue if we already have a log setup for the daemon
            if self.chain_ids.contains(chain_id) {
                continue;
            }
            self.chain_ids.push(chain_id.clone());
            config_builder = self.add_logger(config_builder, chain_id.clone());
            // log startup to each daemon log
            log::info!("Starting specific log: {chain_id}");
        }
        self.handle
            .set_config(InterchainLog::build_logger(config_builder));
    }
}

impl Default for InterchainLog {
    fn default() -> Self {
        // ensure dir exists
        // add main appender to config
        let config_builder = InterchainLog::builder();
        let config = InterchainLog::build_logger(config_builder);

        let handle = log4rs::init_config(config).unwrap();

        Self {
            handle,
            chain_ids: vec![],
        }
    }
}

/// Get the file path for the log target
fn generate_log_file_path(file: &str) -> PathBuf {
    let file_name = format!("{}.log", file);

    let mut log_path = std::env::current_dir().unwrap();
    log_path.push("logs");
    log_path.push(file_name);

    log_path
}
