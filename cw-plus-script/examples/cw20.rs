use std::cell::RefCell;
use std::rc::Rc;

use cosm_script::index_response::IndexResponse;
use cosm_script::networks::juno::JUNO_DAEMON;
use cosm_script::tx_handler::TxHandler;
use cosm_script::{instantiate_daemon_env, Mock, MockState, instantiate_default_mock_env};

use cosmwasm_std::Addr;
use cw_multi_test::{BasicApp, ContractWrapper};
use cw_plus_script::Cw20;
// Requires a running local junod with grpc enabled

pub fn script() -> anyhow::Result<()> {
    for network in [JUNO_DAEMON] {
        let (_, sender, chain) = instantiate_daemon_env(network)?;

        // run contract on a particular chain with a particular sender.
        let token = Cw20::new("cw20", &chain);
        // upload the contract over gRPC
        // token.upload(token.source())?;
        // Instantiate the contract using a custom function
        token.test_generic(sender.address()?)?
    }

    // The same in a cw-multi-test context
    let sender = Addr::unchecked("testing");
    let (state, chain) = instantiate_default_mock_env(&sender)?;
    
    let token = Cw20::new("testing", &chain);
    token.upload(token.source())?;
    token.test_generic(sender)?;
    

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
