use std::sync::Arc;

use boot_core::networks::LOCAL_JUNO;

use boot_core::prelude::*;

use boot_cw_plus::{Cw20, CW20_BASE};
use cosmwasm_std::Addr;

// Requires a running local junod with grpc enabled
pub fn script() -> anyhow::Result<()> {
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());

    // use the cosmos chain registry for gRPC url sources.
    // let chain_data = rt.block_on( ChainData::fetch("juno".into(), None))?;
    // let (sender, chain) = instantiate_daemon_env(&rt,chain_data)?;

    // First we upload, instantiate and interact with a real chain
    let network = LOCAL_JUNO;

    let (sender, chain) = instantiate_daemon_env(&rt, network)?;
    let mut token = Cw20::new(CW20_BASE, &chain);
    token.upload()?;
    token.test_generic(&sender)?;

    // Now we do the same but on a cw-multi-test environment!
    let sender = Addr::unchecked("test_sender");
    let (_, chain) = instantiate_default_mock_env(&sender)?;
    // The same in a cw-multi-test context
    let mut token = Cw20::new("cw-plus:cw20_base", &chain);
    token.upload()?;
    token.test_generic(&sender)?;

    Ok(())
}

fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = script() {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        log::error!("Ensure your environment variables are set!");
        ::std::process::exit(1);
    }
}
