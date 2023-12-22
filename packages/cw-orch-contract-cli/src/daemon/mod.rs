use cw_orch::{
    anyhow::anyhow,
    prelude::{networks::parse_network, Daemon, DaemonBuilder},
    tokio::runtime::Handle,
};

pub trait DaemonFromCli {
    fn from_cli(handle: &Handle) -> cw_orch::anyhow::Result<Daemon> {
        let network_str = inquire::Text::new("Chain id")
            .with_placeholder("uni-6")
            .prompt()?;
        let network = parse_network(&network_str).map_err(|e| anyhow!(e))?;
        let chain = DaemonBuilder::default()
            .handle(handle)
            .chain(network)
            .build()?;
        Ok(chain)
    }
}

impl DaemonFromCli for Daemon {}
