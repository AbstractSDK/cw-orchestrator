use counter_contract::{
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    ContractError,
};
use cw_orch::anyhow;
use cw_orch_cli::ContractCli;

type CounterCli = ContractCli<ContractError, InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg>;

pub fn main() -> anyhow::Result<()> {
    CounterCli::select_action()?;
    Ok(())
}
