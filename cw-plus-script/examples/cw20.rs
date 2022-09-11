use std::cell::RefCell;
use std::rc::Rc;

use cosm_script::networks::juno::JUNO_DAEMON;
use cosm_script::{instantiate_daemon_env, Mock, MockState};

use cosmwasm_std::Addr;
use cw_multi_test::{BasicApp, ContractWrapper};
use cw_plus_script::Cw20;
// Requires a running local junod with grpc enabled

pub fn script() -> anyhow::Result<()> {
    for network in [JUNO_DAEMON] {
        let (_runtime, sender, chain) = instantiate_daemon_env(network)?;

        // run contract on a particular chain with a particular sender.
        let token = Cw20::new("cw20", &chain);
        let _token2 = Cw20::new("raw", &chain);
        let _token3 = Cw20::new("test", &chain);

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
    let sender = Addr::unchecked("testing");
    let mock_state = Rc::new(RefCell::new(MockState::new()));
    let mock_app = Rc::new(RefCell::new(BasicApp::new(|_, _, _| {})));

    let mock_chain = Mock::new(&sender, &mock_state, &mock_app)?;
    let mock_token = Cw20::new("testing", &mock_chain);
    mock_token.upload(
        cosm_script::contract::ContractCodeReference::ContractEndpoints(Box::new(
            ContractWrapper::new_with_empty(
                cw20_base::contract::execute,
                cw20_base::contract::instantiate,
                cw20_base::contract::query,
            ),
        )),
    )?;

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
