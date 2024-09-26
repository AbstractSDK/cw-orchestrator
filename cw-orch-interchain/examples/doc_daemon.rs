use cw_orch::prelude::networks::{LOCAL_JUNO, LOCAL_MIGALOO, LOCAL_OSMO};
use cw_orch::prelude::*;
use cw_orch_interchain::{
    ChannelCreationValidator, ChannelCreator, DaemonInterchain, InterchainEnv, Starship,
};

fn create_daemon_env() -> cw_orch::anyhow::Result<DaemonInterchain> {
    // ANCHOR: DAEMON_INTERCHAIN_CREATION
    // This will create `Daemon` structures associated with chains `LOCAL_JUNO` and `LOCAL_OSMO`
    let mut interchain = DaemonInterchain::new(
        vec![(LOCAL_JUNO, None), (LOCAL_OSMO, None)],
        &ChannelCreationValidator,
    )?;

    let local_juno: Daemon = interchain.get_chain("testing")?;
    let _local_osmo: Daemon = interchain.get_chain("localosmosis")?;

    // You can also create your own daemon and add it manually
    let local_migaloo = DaemonBuilder::new(LOCAL_MIGALOO)
        .state(local_juno.state())
        .build()?;

    interchain.add_daemons(vec![local_migaloo]);
    // ANCHOR_END: DAEMON_INTERCHAIN_CREATION
    Ok(interchain)
}

fn create_starship_env() -> cw_orch::anyhow::Result<DaemonInterchain<Starship>> {
    // ANCHOR: STARSHIP_INTERCHAIN_CREATION
    let starship = Starship::new(None)?;
    let interchain = starship.interchain_env();

    let _local_juno: Daemon = interchain.get_chain("juno-1")?;
    let _local_osmo: Daemon = interchain.get_chain("osmosis-1")?;
    // ANCHOR_END: STARSHIP_INTERCHAIN_CREATION

    Ok(interchain)
}

fn test() -> cw_orch::anyhow::Result<()> {
    create_daemon_env()?;
    create_starship_env()?;
    Ok(())
}

fn main() {
    test().unwrap();
}
