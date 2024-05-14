use counter_contract::{
    msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg},
    CounterContract, CounterExecuteMsgFns, CounterQueryMsgFns,
};
use cw_orch::{daemon::networks::ChainInfo, environment::ChainKind};
use cw_orch::{
    environment::NetworkInfo,
    prelude::{
        ContractInstance, CwOrchExecute, CwOrchInstantiate, CwOrchQuery, CwOrchUpload, Daemon,
        TxHandler,
    },
};

pub const UNION: NetworkInfo = NetworkInfo {
    chain_name: "union",
    pub_address_prefix: "union",
    coin_type: 118,
};

pub const UNION_TESTNET: ChainInfo = ChainInfo {
    chain_id: "union-testnet-8",
    gas_denom: "muno",
    gas_price: 0.0025,
    grpc_urls: &["https://grpc.testnet.bonlulu.uno"],
    lcd_url: None,
    fcd_url: None,
    network_info: UNION,
    kind: ChainKind::Testnet,
};

/// In order to use this script, you need to set the following env variables
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
    let daemon = Daemon::builder()
        // set the network to use
        .chain(UNION_TESTNET)
        .build()
        .unwrap();

    // You create a contract interface to be able to identify what you are interacting with
    let counter = CounterContract::new(daemon.clone());

    // Uploading a contract is very simple
    let upload_res = counter.upload().unwrap();

    let init_res = counter.instantiate(
        &InstantiateMsg { count: 0 },
        Some(&counter.get_chain().sender()),
        None,
    );
    assert!(init_res.is_ok());

    // You can execute a message using actual message types
    let exec_res = counter.execute(&ExecuteMsg::Increment {}, None);
    assert!(exec_res.is_ok());

    let query_res = counter.query::<GetCountResponse>(&QueryMsg::GetCount {});
    assert!(query_res.is_ok());

    // Or you can use even simpler syntax !
    let exec_res = counter.increment();
    assert!(exec_res.is_ok());

    let query_res = counter.get_count();
    assert!(query_res.is_ok());
}
