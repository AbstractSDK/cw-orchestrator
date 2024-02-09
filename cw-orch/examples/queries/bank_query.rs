use anyhow::Result as AnyResult;
use cw_orch::daemon::Daemon;
use cw_orch::prelude::BankQuerier;
use cw_orch::prelude::QuerierGetter;
use cw_orch_daemon::queriers::Bank;
use tokio::runtime::Runtime;
pub fn main() {
    // We start by creating a runtime, which is required for a sync daemon.
    let runtime = Runtime::new().unwrap();

    // We can now create a daemon. This daemon will be used to interact with the chain.
    let daemon = Daemon::builder()
        .chain(cw_orch::daemon::networks::LOCAL_JUNO) // chain parameter
        .handle(runtime.handle()) // handler parameter
        .build()
        .unwrap();

    test_queries(&daemon).unwrap()
}

pub fn test_queries(daemon: &Daemon) -> AnyResult<()> {
    // ANCHOR: daemon_balance_query
    let bank_query_client: Bank = daemon.querier();

    let sender = "valid_sender_addr";
    let balance_result = bank_query_client.balance(sender, None)?;
    println!("Balance of {} : {:?}", sender, balance_result);

    // ANCHOR_END: daemon_balance_query

    Ok(())
}
