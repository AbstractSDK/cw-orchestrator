use counter_contract::{
    msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg},
    CounterContract,
};
use cw_orch::prelude::{
    ContractInstance, CwOrchExecute, CwOrchInstantiate, CwOrchQuery, CwOrchUpload, Daemon,
    TxHandler,
};
use cw_orch_daemon::{ChainInfo, ChainKind, NetworkInfo};

// ANCHOR: neutron
pub const CELESTIA_NETWORK: NetworkInfo = NetworkInfo {
    id: "rollkit",
    pub_address_prefix: "wasm",
    coin_type: 118u32,
};

/// <https://github.com/cosmos/chain-registry/blob/master/testnets/neutrontestnet/chain.json>
pub const ROLLKIT_1: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "celeswasm",
    gas_denom: "uwasm",
    gas_price: 0.025,
    grpc_urls: &["http://localhost:9290"],
    network_info: CELESTIA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

pub fn main() {
    // There are two types of daemon, sync and async. Sync daemons can be used is generic code. Async daemons can be used
    // in async code (e.g. tokio), which enables multi-threaded and non-blocking code.
    dotenv::dotenv().unwrap();
    env_logger::init();

    // ANCHOR: daemon_creation

    // We start by creating a daemon. This daemon will be used to interact with the chain.
    let daemon = Daemon::builder()
        // set the network to use
        .chain(ROLLKIT_1) // chain parameter
        .build()
        .unwrap();

    // ANCHOR_END: daemon_creation

    // ANCHOR: daemon_usage
    let counter = CounterContract::new(daemon.clone());

    let upload_res = counter.upload();
    assert!(upload_res.is_ok());

    let init_res = counter.instantiate(
        &InstantiateMsg { count: 0 },
        Some(&counter.get_chain().sender()),
        None,
    );
    assert!(init_res.is_ok());
    // ANCHOR_END: daemon_usage

    let exec_res = counter.execute(&ExecuteMsg::Increment {}, None);
    assert!(exec_res.is_ok());

    let query_res = counter.query::<GetCountResponse>(&QueryMsg::GetCount {});
    assert!(query_res.is_ok());
}
