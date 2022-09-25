use boot_core::networks::juno::JUNO_DAEMON;
use boot_core::TxHandler;
use boot_core::{instantiate_daemon_env, instantiate_default_mock_env};

use boot_plus::{Cw20, CW20_BASE};
use cosmwasm_std::Addr;

// Requires a running local junod with grpc enabled

pub fn script() -> anyhow::Result<()> {
    // First we upload, instantiate and interact with a real chain
    let network = JUNO_DAEMON;
    let (_, sender, chain) = instantiate_daemon_env(network)?;
    let mut token = Cw20::new(CW20_BASE, &chain);
    // token.upload()?;
    println!("{}", token.upload_required()?);
    // token.test_generic(&sender)?;

    // Now we do the same but on a cw-multi-test environment!
    // let (_, chain) = instantiate_default_mock_env(&sender)?;
    // // The same in a cw-multi-test context
    // let sender = Addr::unchecked("test_sender");
    // let token = Cw20::new("cw-plus:cw20_base", &chain);
    // token.test_generic(&sender)?;

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
        // The backtrace is not always generated. Try to run this example
        // with `$env:RUST_BACKTRACE=1`.
        //    if let Some(backtrace) = e.backtrace() {
        //        log::debug!("backtrace: {:?}", backtrace);
        //    }

        ::std::process::exit(1);
    }
}
