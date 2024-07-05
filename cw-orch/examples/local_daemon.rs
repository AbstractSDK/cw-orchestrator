use counter_contract::{
    msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg},
    CounterContract,
};
use cw_orch::{
    environment::Environment,
    prelude::{CwOrchExecute, CwOrchInstantiate, CwOrchQuery, CwOrchUpload, Daemon, TxHandler},
};

// From https://github.com/CosmosContracts/juno/blob/32568dba828ff7783aea8cb5bb4b8b5832888255/docker/test-user.env#L2
const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

pub fn main() {
    // There are two types of daemon, sync and async. Sync daemons can be used is generic code. Async daemons can be used
    // in async code (e.g. tokio), which enables multi-threaded and non-blocking code.
    std::env::set_var("LOCAL_MNEMONIC", LOCAL_MNEMONIC);
    env_logger::init();

    // ANCHOR: daemon_creation

    // We start by creating a daemon. This daemon will be used to interact with the chain.
    let daemon = Daemon::builder(cw_orch::daemon::networks::LOCAL_JUNO) // chain parameter
        .build()
        .unwrap();

    // ANCHOR_END: daemon_creation

    // ANCHOR: daemon_usage
    let counter = CounterContract::new(daemon.clone());

    let upload_res = counter.upload();
    assert!(upload_res.is_ok());

    let init_res = counter.instantiate(
        &InstantiateMsg { count: 0 },
        Some(&counter.environment().sender_addr()),
        None,
    );
    assert!(init_res.is_ok());
    // ANCHOR_END: daemon_usage

    let exec_res = counter.execute(&ExecuteMsg::Increment {}, None);
    assert!(exec_res.is_ok());

    let query_res = counter.query::<GetCountResponse>(&QueryMsg::GetCount {});
    assert!(query_res.is_ok());
}
