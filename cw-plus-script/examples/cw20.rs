use std::rc::Rc;

use cosm_script::networks::juno::{UNI_3, JUNO_DAEMON};
use cosm_script::{
    sender::{Sender},
    Daemon, DaemonState,
};
use cw_plus_script::Cw20;
// Requires a running local junod with grpc enabled

pub fn script() -> anyhow::Result<()> {
    for network in [JUNO_DAEMON] {
        let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
        let state = &rt.block_on(DaemonState::new(network))?;
        let sender = &Rc::new(Sender::new(state)?);
        let chain = Daemon::new(sender, state, rt)?;
    
        let token = Cw20::new("cw20", chain);
    
        token.upload(token.source())?;
    
        let resp = token.create_new(&sender.address()?, 642406u128)?;
        resp.gas_used;
    
        token.execute(
            &cw20::Cw20ExecuteMsg::Burn {
                amount: 700u128.into(),
            },
            None,
        )?;
        let _token_info: cw20::TokenInfoResponse =
            token.query(&cw20_base::msg::QueryMsg::TokenInfo {})?;
        }
        
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
