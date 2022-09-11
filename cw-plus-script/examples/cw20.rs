use std::rc::Rc;

use crate::Cw20 as cWWW;
use cosm_script::networks::juno::JUNO_DAEMON;
use cosm_script::{
    sender::{Sender, Wallet},
    Daemon, DaemonState,
};
use cw_plus_script::Cw20;
// Requires a running local junod with grpc enabled

pub async fn script() -> anyhow::Result<()> {
    let state = &DaemonState::new(JUNO_DAEMON).await?;
    let sender = Sender::new(state)?;
    let wallet = &Rc::new(sender);
    let chain = Daemon::new(wallet, state)?;

    let token = Cw20::new("cw20", chain);

    token.upload(token.source())?;

    let resp = token.create_new(&sender.address()?, 642406u128)?;
    resp.gas_used;

    token.execute(
        &cw20::Cw20ExecuteMsg::Burn {
            amount: 700u128.into(),
        },
        None,
    );
    let _token_info: cw20::TokenInfoResponse =
        token.query(cw20_base::msg::QueryMsg::TokenInfo {}).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = script().await {
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
