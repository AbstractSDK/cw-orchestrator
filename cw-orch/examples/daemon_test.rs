use counter_contract::{
    contract::CounterContract,
    msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg},
};
use cw_orch::prelude::{
    networks, ContractInstance, CwOrchExecute, CwOrchInstantiate, CwOrchQuery, CwOrchUpload,
    Daemon, TxHandler,
};
use tokio::runtime::Runtime;
const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

pub fn main() {
    // There are two types of daemon, sync and async. Sync daemons can be used is generic code. Async daemons can be used
    // in async code (e.g. tokio), which enables multi-threaded and non-blocking code.

    env_logger::init();
    // We start by creating a runtime, which is required for a sync daemon.
    let runtime = Runtime::new().unwrap();

    // We can now create a daemon. This daemon will be used to interact with the chain.
    let daemon = Daemon::builder()
        // set the network to use
        .chain(networks::LOCAL_JUNO)
        .handle(runtime.handle())
        .mnemonic(LOCAL_MNEMONIC)
        .build()
        .unwrap();

    let counter = CounterContract::new("local:counter", daemon.clone());

    let upload_res = counter.upload();
    assert!(upload_res.is_ok());

    let init_res = counter.instantiate(
        &InstantiateMsg { count: 0 },
        Some(&counter.get_chain().sender()),
        None,
    );
    assert!(init_res.is_ok());

    let exec_res = counter.execute(&ExecuteMsg::Increment {}, None);
    assert!(exec_res.is_ok());

    let query_res = counter.query::<GetCountResponse>(&QueryMsg::GetCount {});
    assert!(query_res.is_ok());
}
