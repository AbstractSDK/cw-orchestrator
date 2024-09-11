// ANCHOR: full_counter_example

use cw_orch::{anyhow, prelude::*};
use cw_orch_daemon::senders::QueryOnlyDaemon;

// From https://github.com/CosmosContracts/juno/blob/32568dba828ff7783aea8cb5bb4b8b5832888255/docker/test-user.env#L1
pub const LOCAL_JUNO_SENDER: &str = "juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y";

pub fn main() -> anyhow::Result<()> {
    pretty_env_logger::init(); // Used to log contract and chain interactions

    let network = networks::LOCAL_JUNO;
    // QueryOnlyDaemon doesn't need a mnemonic to function
    let chain: QueryOnlyDaemon = QueryOnlyDaemon::builder(network).build_sender(())?;

    let balances = chain
        .bank_querier()
        .balance(&Addr::unchecked(LOCAL_JUNO_SENDER), None)?;
    assert!(!balances.is_empty());

    Ok(())
}
