use std::cell::RefCell;
use std::rc::Rc;

use cosm_script::contract::{get_source, ContractSource};
use cosm_script::index_response::IndexResponse;
use cosm_script::networks::juno::JUNO_DAEMON;
use cosm_script::tx_handler::TxHandler;
use cosm_script::{instantiate_daemon_env, Mock, MockState};

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
        token.upload(token.source())?;
        // Instantiate the contract using a custom function
        let resp = token.create_new(&sender.address()?, 420u128)?;
        // Access the execution result
        println!("gas used in token creation: {}", resp.gas_used);
        // get the user balance and assert for testing purposes
        let new_balance = token.balance(&sender.address()?)?;
        // balance == mint balance
        assert_eq!(420u128, new_balance.u128());
        // BURNNNN
        token.execute(
            &cw20::Cw20ExecuteMsg::Burn {
                amount: 96u128.into(),
            },
            None,
        )?;
        let _token_info: cw20::TokenInfoResponse =
            token.query(&cw20_base::msg::QueryMsg::TokenInfo {})?;
    }

    // The same in a cw-multi-test context
    let sender = Addr::unchecked("testing");

    let mock_chain = Mock::new(&sender, &mock_state, &mock_app)?;
    let mock_token = Cw20::new("testing", &mock_chain);
    // mock_token.upload(
    //     ,
    // )?;

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
