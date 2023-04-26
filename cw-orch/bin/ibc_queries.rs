use cw_orch::channel::ChannelAccess;
use cw_orch::ibc_tracker::{
    CwIbcContractState, IbcTracker, IbcTrackerConfig, IbcTrackerConfigBuilder,
};
use cw_orch::{queriers::Ibc, *};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::select;
use tokio::time::sleep;

pub fn script() -> anyhow::Result<()> {
    // create the tokio runtime
    let rt = Arc::new(Runtime::new().unwrap());
    let options = DaemonOptionsBuilder::default()
        // or provide `chain_data`
        .network(networks::JUNO_1)
        // specify a custom deployment ID
        .deployment_id("v0.1.0")
        .build()?;

    // get sender form .env file mnemonic
    let (_sender, chain) = instantiate_daemon_env(&rt, options)?;

    let juno_channel = chain.channel();

    let tracker = IbcTrackerConfigBuilder::default()
        .ibc_state(CwIbcContractState::new("connection-0", "transfer"))
        .log_level(log::LevelFilter::Info)
        .build()?;

    // spawn juno logging on a different thread.
    rt.spawn(async move {
        juno_channel.cron_log(tracker).await;
    });

    rt.block_on(async { sleep(Duration::from_secs(100)).await });

    // rt.block_on(async {
    //     let osmosis_connections = ibc.open_connections("osmosis-1").await.unwrap();
    //     println!("osmosis_connections: {:#?}", osmosis_connections);

    // });

    Ok(())
}

fn main() {
    dotenv().ok();
    // env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = script() {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));
        ::std::process::exit(1);
    }
}
