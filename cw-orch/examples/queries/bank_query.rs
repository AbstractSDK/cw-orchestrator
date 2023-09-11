use anyhow::Result as AnyResult;
use cw_orch::daemon::Daemon;
use cw_orch::prelude::queriers::Bank;
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

    runtime.block_on(test_queries(&daemon)).unwrap()
}

pub async fn test_queries(daemon: &Daemon) -> AnyResult<()> {
    // ANCHOR: daemon_balance_query
    let bank_query_client: Bank = daemon.query_client();

    let sender = "valid_sender_addr";
    let balance_result = bank_query_client.balance(sender, None).await?;
    println!("Balance of {} : {:?}", sender, balance_result);

    // ANCHOR_END: daemon_balance_query

    Ok(())
}
