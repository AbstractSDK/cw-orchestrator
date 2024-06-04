use common::ica_demo::full_ica_test;
use cw_orch_interchain_core::InterchainEnv;
use cw_orch_interchain_mock::MockInterchainEnv;
mod common;
use cosmwasm_std::coins;

pub fn logger_test_init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

pub const JUNO: &str = "juno-1";
pub const OSMOSIS: &str = "osmosis-1";
pub const JUNO_FUNDS_DENOM: &str = "ujuno";

#[test]
fn mock_ica_demo() -> cw_orch::anyhow::Result<()> {
    // logger_test_init();
    env_logger::init();
    let common_sender = "sender";
    let interchain = MockInterchainEnv::new(vec![(JUNO, common_sender), (OSMOSIS, common_sender)]);

    // We need to add balance to the JUNO chain
    interchain
        .chain(JUNO)?
        .add_balance(common_sender, coins(100_000, JUNO_FUNDS_DENOM))?;

    full_ica_test(&interchain, JUNO, OSMOSIS, JUNO_FUNDS_DENOM)?;

    Ok(())
}
