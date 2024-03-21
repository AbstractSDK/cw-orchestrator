// ANCHOR: full_counter_example
use counter_contract::{
    msg::InstantiateMsg, CounterContract, CounterExecuteMsgFns, AsyncCounterQueryMsgFns,
};
use cw_orch::{anyhow, prelude::*, tokio};
use tokio::runtime::Runtime;

const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    std::env::set_var("TEST_MNEMONIC", LOCAL_MNEMONIC);
    // ANCHOR: chain_construction
    dotenv::dotenv().ok(); // Used to load the `.env` file if any
    pretty_env_logger::init(); // Used to log contract and chain interactions

    let network = networks::UNI_6;
    let chain = DaemonAsyncBuilder::default()
        .chain(network)
        .build().await?;
    // ANCHOR_END: chain_construction

    // ANCHOR: contract_interaction

    let counter = CounterContract::new(chain);

    let count = counter.get_count().await?;
    assert_eq!(count.count, 1);
    // ANCHOR_END: clean_example
    // ANCHOR_END: contract_interaction

    Ok(())
}
// ANCHOR_END: full_counter_example
