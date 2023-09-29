use counter_contract::{msg::InstantiateMsg, CounterContract};
use cw_orch::{anyhow, prelude::*, tokio};
use tokio::runtime::Runtime;

const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";
/// Script that registers the first Account in abstract (our Account)
pub fn main() -> anyhow::Result<()> {
    std::env::set_var("LOCAL_MNEMONIC", LOCAL_MNEMONIC);
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
