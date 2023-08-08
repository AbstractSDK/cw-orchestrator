use counter_contract::CounterContract;
use cw_orch::{
    anyhow,
    prelude::{networks, DaemonBuilder},
    tokio::runtime::Runtime,
};
use cw_orch_cli::ContractCli;

pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let rt = Runtime::new()?;
    let network = networks::UNI_6;
    let chain = DaemonBuilder::default()
        .handle(rt.handle())
        .chain(network)
        .build()?;

    let counter = CounterContract::new("counter_contract", chain);

    ContractCli::select_action(counter)?;

    Ok(())
}
