use cw_orch::{
    daemon::networks::{PION_1, XION_TESTNET_1},
    tokio::runtime::Runtime,
};
use cw_orch_interchain::prelude::*;
use hermes_relayer::relayer::HermesRelayer;
use old_ibc_relayer_types::core::ics24_host::identifier::PortId;

pub fn main() -> cw_orch::anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();
    let rt = Runtime::new()?;

    let relayer = HermesRelayer::new(
        rt.handle(),
        vec![
            (
                PION_1,
                None,
                true,
                "https://rpc-falcron.pion-1.ntrn.tech/".to_string(),
            ),
            (
                XION_TESTNET_1,
                None,
                false,
                "https://xion-testnet-rpc.polkachu.com".to_string(),
            ),
        ],
        vec![(
            (
                XION_TESTNET_1.chain_id.to_string(),
                PION_1.chain_id.to_string(),
            ),
            "connection-63".to_string(),
        )]
        .into_iter()
        .collect(),
    )?;

    let interchain = relayer.interchain_env();

    interchain.create_channel(
        "xion-testnet-1",
        "pion-1",
        &PortId::transfer(),
        &PortId::transfer(),
        "ics20-1",
        None,
    )?;

    Ok(())
}
