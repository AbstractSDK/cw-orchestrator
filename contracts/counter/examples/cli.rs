use cosmwasm_std::Empty;
use counter_contract::CounterContract;
use cw_orch::{anyhow, prelude::Daemon, tokio::runtime::Runtime};
use cw_orch_contract_cli::{ContractCli, DaemonFromCli};

pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let rt = Runtime::new()?;
    let chain = Daemon::from_cli(rt.handle())?;
    let counter = CounterContract::new(chain);

    counter.select_action(Empty {})?;

    Ok(())
}
