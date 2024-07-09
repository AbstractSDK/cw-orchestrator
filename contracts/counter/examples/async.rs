// ANCHOR: full_async_example
use counter_contract::AsyncCounterQueryMsgFns;
use counter_contract::CounterContract;
use cw_orch::{anyhow, prelude::*, tokio};

// From https://github.com/CosmosContracts/juno/blob/32568dba828ff7783aea8cb5bb4b8b5832888255/docker/test-user.env#L2
const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    std::env::set_var("LOCAL_MNEMONIC", LOCAL_MNEMONIC);
    dotenv::dotenv().ok(); // Used to load the `.env` file if any
    pretty_env_logger::init(); // Used to log contract and chain interactions

    let network = networks::LOCAL_JUNO;
    let chain = DaemonAsyncBuilder::new(network).build().await?;

    let counter = CounterContract::new(chain);

    let count = counter.get_count_async().await?;
    assert_eq!(count.count, 1);

    Ok(())
}

// ANCHOR_END: full_async_example
