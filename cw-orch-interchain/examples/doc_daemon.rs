use cw_orch::prelude::networks::{LOCAL_JUNO, LOCAL_MIGALOO, LOCAL_OSMO};
use cw_orch::prelude::*;
use cw_orch_interchain::{
    ChannelCreationValidator, ChannelCreator, DaemonInterchainEnv, InterchainEnv, Starship,
};
/// Others

fn create_daemon_env() -> cw_orch::anyhow::Result<DaemonInterchainEnv> {
    let mut interchain = DaemonInterchainEnv::new(
        vec![(LOCAL_JUNO, None), (LOCAL_OSMO, None)],
        &ChannelCreationValidator,
    )?;

    let local_juno: Daemon = interchain.get_chain("testing")?;
    let _local_osmo: Daemon = interchain.get_chain("localosmosis")?;

    let local_migaloo = DaemonBuilder::new(LOCAL_MIGALOO)
        .state(local_juno.state())
        .handle(rt.handle())
        .build()?;
    interchain.add_daemons(vec![local_migaloo]);

    Ok(interchain)
}

fn create_starship_env() -> cw_orch::anyhow::Result<DaemonInterchainEnv<Starship>> {
    let starship = Starship::new(None)?;
    let interchain = starship.interchain_env();

    let _local_juno: Daemon = interchain.get_chain("juno-1")?;
    let _local_osmo: Daemon = interchain.get_chain("osmosis-1")?;

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
