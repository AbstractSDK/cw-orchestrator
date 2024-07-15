use cw_orch::prelude::*;
use cw_orch_interchain::{InterchainEnv, MockInterchainEnv};
use ibc_relayer_types::core::ics24_host::identifier::PortId;

fn crate_mock_env() -> cw_orch::anyhow::Result<MockInterchainEnv> {
    let sender = "sender";
    let mut interchain = MockInterchainEnv::new(vec![("juno-1", sender), ("osmosis-1", sender)]);

    let _test_juno: Mock = interchain.get_chain("juno-1")?;
    let _test_osmo: Mock = interchain.get_chain("osmosis-1")?;

    let test_migaloo = Mock::new(sender);
    interchain.add_mocks(vec![test_migaloo]);

    Ok(interchain)
}

fn create_channel(interchain: MockInterchainEnv) -> cw_orch::anyhow::Result<()> {
    let src_chain = "juno-1".to_string();
    let dst_chain = "osmosis-1".to_string();
    let port_id = PortId::transfer();
    interchain.create_channel(
        &src_chain,
        &dst_chain,
        &port_id,
        &port_id,
        "ics20-1",
        Some(cosmwasm_std::IbcOrder::Unordered),
    )?;
    Ok(())
}

fn test() -> cw_orch::anyhow::Result<()> {
    let interchain = crate_mock_env()?;
    create_channel(interchain)?;
    Ok(())
}

fn main() {
    test().unwrap();
}
