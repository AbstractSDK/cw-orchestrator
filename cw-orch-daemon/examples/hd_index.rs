use cosmwasm_std::coins;
use cw_orch::{anyhow, prelude::*};
use cw_orch_daemon::CosmosOptions;

// From https://github.com/CosmosContracts/juno/blob/32568dba828ff7783aea8cb5bb4b8b5832888255/docker/test-user.env#L1
pub const LOCAL_JUNO_SENDER: &str = "juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y";
pub const LOCAL_JUNO_GRANTER: &str = "juno1afhtjur8js4589xymu346ca7a5y5293xpuv6ry";

pub fn main() -> anyhow::Result<()> {
    pretty_env_logger::init(); // Used to log contract and chain interactions

    let network = networks::LOCAL_JUNO;
    // The mnemonic is read from environment variables automatically, no need to specify in this builders
    let sender = CosmosOptions::default()
        .hd_index(5)
        .authz_granter(&Addr::unchecked(LOCAL_JUNO_GRANTER));
    let chain = Daemon::builder(network).build_sender(sender)?;

    chain
    .bank_send(&Addr::unchecked(LOCAL_JUNO_GRANTER), &coins(10000, "ujuno"))?;
    

    Ok(())
}
