use cosmwasm_std::IbcOrder;
use cw_orch_core::environment::QuerierGetter;
use cw_orch_daemon::queriers::Ibc;
use cw_orch_daemon::Daemon;
use cw_orch_interchain_core::{types::ChannelCreationResult, InterchainEnv};
use cw_orch_interchain_daemon::ChannelCreator;
use cw_orch_starship::Starship;
use ibc_relayer_types::core::ics24_host::identifier::PortId;

fn assert_ordering(
    channel: ChannelCreationResult<Daemon>,
    juno: &Daemon,
    order: IbcOrder,
) -> anyhow::Result<()> {
    let ibc_querier: Ibc = juno.querier();
    let channel = channel
        .interchain_channel
        .get_ordered_ports_from("juno-1")?;

    let channel_info = juno.rt_handle.block_on(ibc_querier._channel(
        channel.0.port.to_string(),
        channel.0.channel.unwrap().to_string(),
    ))?;
    match order {
        IbcOrder::Ordered => assert_eq!(channel_info.ordering, 2),
        IbcOrder::Unordered => assert_eq!(channel_info.ordering, 1),
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let starship = Starship::new(None)?;
    let interchain_env = starship.interchain_env();

    let juno = interchain_env.get_chain("juno-1")?;

    let channel_created = interchain_env.create_channel(
        "juno-1",
        "stargaze-1",
        &PortId::transfer(),
        &PortId::transfer(),
        "ics20-1",
        Some(IbcOrder::Unordered),
    )?;
    assert_ordering(channel_created, &juno, IbcOrder::Unordered)?;

    let channel_created = interchain_env.create_channel(
        "juno-1",
        "stargaze-1",
        &PortId::transfer(),
        &PortId::transfer(),
        "ics20-1",
        None,
    )?;
    assert_ordering(channel_created, &juno, IbcOrder::Unordered)?;

    Ok(())
}
