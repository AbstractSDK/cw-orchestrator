use cw_orch::prelude::networks::{LOCAL_JUNO, LOCAL_MIGALOO, LOCAL_OSMO};
use cw_orch::prelude::*;
/// For create daemon env
use cw_orch::tokio::runtime::Runtime;
use cw_orch_interchain::{
    ChannelCreationValidator, ChannelCreator, DaemonInterchainEnv, InterchainEnv, Starship,
};
/// Others

fn create_daemon_env() -> cw_orch::anyhow::Result<(Runtime, DaemonInterchainEnv)> {
    let rt = Runtime::new()?;
    let mut interchain = DaemonInterchainEnv::new(
        rt.handle(),
        vec![(LOCAL_JUNO, None), (LOCAL_OSMO, None)],
        &ChannelCreationValidator,
    )?;

    let _local_juno: Daemon = interchain.chain("testing")?;
    let _local_osmo: Daemon = interchain.chain("localosmosis")?;

    let local_migaloo = DaemonBuilder::default()
        .handle(rt.handle())
        .chain(LOCAL_MIGALOO)
        .build()?;
    interchain.add_daemons(vec![local_migaloo]);

    Ok((rt, interchain))
}

fn create_starship_env() -> cw_orch::anyhow::Result<(Runtime, DaemonInterchainEnv<Starship>)> {
    let rt = Runtime::new()?;
    let starship = Starship::new(rt.handle(), None)?;
    let interchain = starship.interchain_env();

    let _local_juno: Daemon = interchain.chain("juno-1")?;
    let _local_osmo: Daemon = interchain.chain("osmosis-1")?;

    Ok((rt, interchain))
}

fn test() -> cw_orch::anyhow::Result<()> {
    create_daemon_env()?;
    create_starship_env()?;
    Ok(())
}

fn main() {
    test().unwrap();
}
