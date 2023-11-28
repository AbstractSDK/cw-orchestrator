use std::str::FromStr;

use cosmrs::{tx::Msg, AccountId, Coin, Denom};
use cosmwasm_std::coins;
// ANCHOR: full_counter_example
use cw_orch_daemon::DaemonBuilder;
use cw_orch_networks::networks;
use tokio::runtime::Runtime;

const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";
pub fn main() -> anyhow::Result<()> {
    std::env::set_var("LOCAL_MNEMONIC", LOCAL_MNEMONIC);

    let rt = Runtime::new()?;
    let network = networks::LOCAL_JUNO;
    let daemon = DaemonBuilder::default()
        .handle(rt.handle())
        .chain(network)
        .build()?;

    // We commit the tx (also resimulates the tx)
    // ANCHOR: send_tx
    let wallet = daemon.wallet();
    rt.block_on(wallet.bank_send("<address-of-my-sister>", coins(345, "ujunox")))?;
    // ANCHOR_END: send_tx

    // ANCHOR: cosmrs_tx
    let tx_msg = cosmrs::staking::MsgBeginRedelegate {
        // Delegator's address.
        delegator_address: AccountId::from_str("<my-address>").unwrap(),

        // Source validator's address.
        validator_src_address: AccountId::from_str("<my-least-favorite-validator>").unwrap(),

        // Destination validator's address.
        validator_dst_address: AccountId::from_str("<my-favorite-validator>").unwrap(),

        // Amount to UnDelegate
        amount: Coin {
            amount: 100_000_000_000_000u128,
            denom: Denom::from_str("ujuno").unwrap(),
        },
    };
    rt.block_on(wallet.commit_tx(vec![tx_msg.clone()], None))?;
    // ANCHOR_END: cosmrs_tx

    // ANCHOR: any_tx
    rt.block_on(wallet.commit_tx_any(
        vec![cosmrs::Any {
            type_url: "/cosmos.staking.v1beta1.MsgBeginRedelegate".to_string(),
            value: tx_msg.to_any().unwrap().value,
        }],
        None,
    ))?;
    // ANCHOR_END: any_tx

    // ANCHOR: simulate_tx
    let (gas_needed, fee_needed) =
        rt.block_on(wallet.simulate(vec![tx_msg.to_any().unwrap()], None))?;

    log::info!(
        "Submitting this transaction will necessitate: 
            - {gas_needed} gas
            - {fee_needed} for the tx fee"
    );
    // ANCHOR_END: simulate_tx

    Ok(())
}
// ANCHOR_END: full_counter_example
