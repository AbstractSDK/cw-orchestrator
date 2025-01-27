use anyhow::Result as AnyResult;
use cw_orch::daemon::Daemon;
use cw_orch::prelude::BankQuerier;
use cw_orch::prelude::QuerierGetter;
use cw_orch::prelude::TxHandler;
use cw_orch_daemon::queriers::Bank;
pub fn main() {
    dotenv::dotenv().unwrap();
    env_logger::init();
    // We can now create a daemon. This daemon will be used to interact with the chain.
    let daemon = Daemon::builder(cw_orch::daemon::networks::LOCAL_JUNO) // chain parameter
        .build()
        .unwrap();

    test_queries(&daemon).unwrap()
}

pub fn test_queries(daemon: &Daemon) -> AnyResult<()> {
    // ANCHOR: daemon_balance_query
    let bank_query_client: Bank = daemon.querier();

    let sender = daemon.sender_addr();
    let balance_result = bank_query_client.balance(&sender, None)?;
    println!("Balance of {} : {:?}", sender, balance_result);

    // ANCHOR_END: daemon_balance_query

    Ok(())
}
