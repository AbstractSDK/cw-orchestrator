use cosmwasm_std::{coin, CosmosMsg, IbcMsg, IbcTimeout, IbcTimeoutBlock};
use cw_orch::{
    environment::{QueryHandler, TxHandler},
    mock::cw_multi_test::Executor,
};
use cw_orch_interchain_core::InterchainEnv;
use cw_orch_interchain_mock::MockInterchainEnv;
use ibc_relayer_types::core::ics24_host::identifier::PortId;

#[test]
fn mock_transfer() -> cw_orch::anyhow::Result<()> {
    pretty_env_logger::init();

    let interchain = MockInterchainEnv::new(vec![("juno-1", "sender"), ("stargaze-1", "sender")]);

    let channel = interchain
        .create_channel(
            "juno-1",
            "stargaze-1",
            &PortId::transfer(),
            &PortId::transfer(),
            "ics20-1",
            None,
        )
        .unwrap();
    let juno = interchain.get_chain("juno-1").unwrap();
    let stargaze = interchain.get_chain("stargaze-1").unwrap();

    let channel = channel
        .interchain_channel
        .get_ordered_ports_from("juno-1")
        .unwrap();

    juno.add_balance(juno.sender_addr().to_string(), vec![coin(100_000, "ujuno")])
        .unwrap();
    let tx_resp = juno
        .app
        .borrow_mut()
        .execute(
            juno.sender_addr(),
            CosmosMsg::Ibc(IbcMsg::Transfer {
                channel_id: channel.0.channel.unwrap().to_string(),
                to_address: stargaze.sender_addr().to_string(),
                amount: coin(100_000, "ujuno"),
                timeout: IbcTimeout::with_block(IbcTimeoutBlock {
                    revision: 1,
                    height: stargaze.block_info().unwrap().height + 1,
                }),
                memo: None,
            }),
        )
        .unwrap();

    // This makes sure that the packets arrive successfully and present a success ack
    interchain
        .await_and_check_packets("juno-1", tx_resp)
        .unwrap();

    Ok(())
}
