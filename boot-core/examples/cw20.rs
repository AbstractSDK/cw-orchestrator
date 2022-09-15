use std::cell::RefCell;
use std::rc::Rc;

use boot_core::index_response::IndexResponse;
use boot_core::networks::juno::JUNO_DAEMON;
use boot_core::tx_handler::TxHandler;
use boot_core::{instantiate_daemon_env, instantiate_default_mock_env, Mock, MockState};

use cosmwasm_std::Addr;
use cw_multi_test::{BasicApp, ContractWrapper};
use boot_plus::Cw20;
// Requires a running local junod with grpc enabled

pub fn script() -> anyhow::Result<()> {
    // First we upload, instantiate and interact with a real chain
    let network = JUNO_DAEMON;
    let (_, sender, chain) = instantiate_daemon_env(network)?;
    let mut token = Cw20::new("cw20", &chain);
    token.upload()?;
    token.test_generic(&sender)?;

    // Now we do the same but on a cw-multi-test environment!
    let (_, chain) = instantiate_default_mock_env(&sender)?;
    // The same in a cw-multi-test context
    let sender = Addr::unchecked("test_sender");
    let token = Cw20::new("cw20", &chain);
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

        // The backtrace is not always generated. Try to run this example
        // with `$env:RUST_BACKTRACE=1`.
        //    if let Some(backtrace) = e.backtrace() {
        //        log::debug!("backtrace: {:?}", backtrace);
        //    }

        ::std::process::exit(1);
    }
}
