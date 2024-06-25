// ANCHOR: full_counter_example
use counter_contract::CounterContract;
use cw_orch::{
    anyhow,
    daemon::senders::manual_sender::{ManualDaemon, ManualSenderOptions},
    prelude::*,
};

// This is a test with a manual sender, to verify everything works, nothing is broadcasted

// From https://github.com/CosmosContracts/juno/blob/32568dba828ff7783aea8cb5bb4b8b5832888255/docker/test-user.env#L2
pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok(); // Used to load the `.env` file if any
    pretty_env_logger::init(); // Used to log contract and chain interactions

    let network = networks::JUNO_1;
    let sender = "juno1xjf5xscdk08c5es2m7epmerrpqmkmc3n98650t";
    let chain = ManualDaemon::builder()
        .options(ManualSenderOptions {
            sender_address: Some(sender.to_string()),
        })
        .chain(network)
        .build()?;

    let counter = CounterContract::new(chain.clone());

    // Example tx hash that succeed (correspond to a code upload tx)
    // 58AA802705BEE4597A560FBC67F6C86400E66F5FCBD0F08AA37FB140BCD65B6D
    // If not found, try to find the latests juno code uploaded (4380+)
    // https://www.mintscan.io/juno/wasm/code/4380
    counter.upload()?;

    Ok(())
}
