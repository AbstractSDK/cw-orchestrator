use anyhow::Result as AnyResult;
use cosmwasm_std::Addr;
use cw_orch::daemon::Daemon;
use cw_orch::prelude::BankQuerier;
use cw_orch::prelude::QuerierGetter;
use cw_orch_daemon::queriers::Ibc;
use cw_orch_daemon::queriers::{Bank, Staking};
pub const TEST_MNEMONIC: &str="scare silent genuine cheese monitor industry item cloth pet gather cruise long confirm van lunar tomato scrub silk guide eight truly rural remember swim";

pub fn main() -> AnyResult<()> {
    // We start by creating a daemon. This daemon will be used to interact with the chain.
    let daemon = Daemon::builder(cw_orch::daemon::networks::JUNO_1) // chain parameter
        .mnemonic(TEST_MNEMONIC)
        .build()?;

    // We do an actual bank query on MAINNET
    let bank_query_client: Bank = daemon.querier();
    let sender = Addr::unchecked("juno185hgkqs8q8ysnc8cvkgd8j2knnq2m0ah6ae73gntv9ampgwpmrxqc5vwdr");
    let balance_result = bank_query_client.balance(&sender, None)?;
    println!("Balance of {} : {:?}", sender, balance_result);

    // We do an actual Staking query on MAINNET
    let staking_query_client: Staking = daemon.querier();
    let validator =
        Addr::unchecked("junovaloper185hgkqs8q8ysnc8cvkgd8j2knnq2m0ah6ae73gntv9ampgwpmrxqlfzywn");
    let validator_result = daemon
        .rt_handle
        .block_on(staking_query_client._validator(&validator))?;
    println!("Validator info of {} : {:?}", sender, validator_result);

    // We do an actual IBC query on MAINNET
    let ibc_query_client: Ibc = daemon.querier();
    let port_id = "transfer";
    let channel_id = "channel-0";
    let channel_result = daemon
        .rt_handle
        .block_on(ibc_query_client._channel(port_id, channel_id))?;
    println!(
        "Channel info of {port_id}:{channel_id} : {:?}",
        channel_result
    );

    Ok(())
}
