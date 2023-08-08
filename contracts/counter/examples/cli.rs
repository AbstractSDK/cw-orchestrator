use counter_contract::{
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    ContractError, CounterContract,
};
use cw_orch::{anyhow, tokio::runtime::Runtime, prelude::{networks, DaemonBuilder}};
use cw_orch_cli::ContractCli;

pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

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
