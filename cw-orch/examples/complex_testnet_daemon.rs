use cosmwasm_std::coins;
use counter_contract::{
    msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg},
    CounterContract, CounterExecuteMsgFns, CounterQueryMsgFns,
};
use cw_orch::prelude::*;
use cw_orch_traits::Stargate;
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin,
    osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgMint},
};
use prost::Message;
use prost_types::Any;

pub const SUBDENOM: &str = "complex-test";

/// In order to use this script, you need to set the following env variables
///
/// RUST_LOG (recommended value `info`) to see the app logs
/// TEST_MNEMONIC to be able to sign and broadcast a transaction on UNI testnet
pub fn main() {
    // We start by loading environment variables from a .env file.
    // You can use a .env file to specify environment variables.
    // You have an overview of all supported environment variables here : https://orchestrator.abstract.money/contracts/env-variable.html
    dotenv::dotenv().unwrap();

    // We initialize the env logger to be able to see what's happening during the script execution
    // Remember to set the `RUST_LOG` env variable to be able to see the execution
    env_logger::init();

    // We can now create a daemon. This daemon will be used to interact with the chain.
    // In the background, the `build` function uses the `TEST_MNEMONIC` variable, don't forget to set it !
    let daemon = Daemon::builder(cw_orch::daemon::networks::UNI_6) // set the network to use
        .build()
        .unwrap();

    // You create a contract interface to be able to identify what you are interacting with
    let counter = CounterContract::new(daemon.clone());

    // Uploading a contract is very simple
    let upload_res = counter.upload();
    assert!(upload_res.is_ok());

    let init_res = counter.instantiate(
        &InstantiateMsg { count: 0 },
        Some(&counter.environment().sender_addr()),
        &[],
    );
    assert!(init_res.is_ok());

    // You can execute a message using actual message types
    let exec_res = counter.execute(&ExecuteMsg::Increment {}, &[]);
    assert!(exec_res.is_ok());

    let query_res = counter.query::<GetCountResponse>(&QueryMsg::GetCount {});
    assert!(query_res.is_ok());

    // Or you can use even simpler syntax !
    let exec_res = counter.increment();
    assert!(exec_res.is_ok());

    let query_res = counter.get_count();
    assert!(query_res.is_ok());

    let sender_addr = daemon.sender_addr().to_string();
    // We create a denom
    daemon
        .commit_any(
            vec![Any {
                type_url: MsgCreateDenom::TYPE_URL.to_string(),
                value: MsgCreateDenom {
                    sender: sender_addr.clone(),
                    subdenom: SUBDENOM.to_string(),
                }
                .encode_to_vec(),
            }],
            None,
        )
        .unwrap();
    let denom = format!("factory/{}/{}", sender_addr, SUBDENOM);
    // We mint some tokens
    daemon
        .commit_any(
            vec![Any {
                type_url: MsgMint::TYPE_URL.to_string(),
                value: MsgMint {
                    sender: sender_addr.clone(),
                    amount: Some(Coin {
                        amount: "100000".to_string(),
                        denom: denom.clone(),
                    }),
                    mint_to_address: sender_addr.clone(),
                }
                .encode_to_vec(),
            }],
            None,
        )
        .unwrap();
    // We send some funds to the counter contract
    let contract_addr = counter.address().unwrap();

    daemon
        .bank_send(&contract_addr, &coins(50_000, denom.clone()))
        .unwrap();

    // We verify they have received their funds
    assert_eq!(
        daemon
            .bank_querier()
            .balance(&contract_addr, Some(denom.clone()))
            .unwrap(),
        coins(50_000, denom.clone())
    );
}
