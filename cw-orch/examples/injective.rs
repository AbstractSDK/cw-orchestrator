use counter_contract::{
    msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg},
    CounterContract,
};
use cw_orch::{
    environment::Environment,
    prelude::{
        networks, CwOrchExecute, CwOrchInstantiate, CwOrchQuery, CwOrchUpload, Daemon, TxHandler,
    },
};

const TESTNET_MNEMONIC: &str = "across left ignore gold echo argue track joy hire release captain enforce hotel wide flash hotel brisk joke midnight duck spare drop chronic stool";

pub fn main() {
    // There are two types of daemon, sync and async. Sync daemons can be used is generic code. Async daemons can be used
    // in async code (e.g. tokio), which enables multi-threaded and non-blocking code.

    env_logger::init();

    // We can now create a daemon. This daemon will be used to interact with the chain.
    let res = Daemon::builder()
        // set the network to use
        .chain(networks::INJECTIVE_888)
        .mnemonic(TESTNET_MNEMONIC)
        .build();

    let Some(daemon) = res.as_ref().ok() else {
        panic!("Error: {}", res.err().unwrap());
    };

    let counter = CounterContract::new(daemon.clone());

    let upload_res = counter.upload();
    assert!(upload_res.is_ok());

    let init_res = counter.instantiate(
        &InstantiateMsg { count: 0 },
        Some(&counter.environment().sender()),
        None,
    );
    assert!(init_res.is_ok());

    let exec_res = counter.execute(&ExecuteMsg::Increment {}, None);
    assert!(exec_res.is_ok());

    let query_res = counter.query::<GetCountResponse>(&QueryMsg::GetCount {});
    assert!(query_res.is_ok());
}
