use counter_contract::msg::InstantiateMsg;
use counter_contract::CounterContract;
use cw_orch::anyhow;
use cw_orch::prelude::*;
use cw_orch::tokio;
use tokio::runtime::Runtime;

/// Script that registers the first Account in abstract (our Account)
pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let rt = Runtime::new()?;
    let network = networks::LOCAL_JUNO;
    let chain = DaemonBuilder::default()
        .handle(rt.handle())
        .chain(network)
        .build()?;

    let counter = CounterContract::new("counter_contract", chain);

    counter.upload()?;
    counter.instantiate(&InstantiateMsg { count: 0 }, None, None)?;
    Ok(())
}
