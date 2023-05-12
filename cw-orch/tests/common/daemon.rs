use tokio::runtime::Runtime;

use uid::Id as IdT;

#[derive(Copy, Clone, Eq, PartialEq)]
struct DeployId(());

type Id = IdT<DeployId>;

use cw_orch::{daemon::networks::LOCAL_JUNO, environment::TxHandler, prelude::SyncDaemon};
#[allow(unused)]
/// Get the test-daemon object (local juno)
pub fn daemon(runtime: &Runtime) -> (cosmwasm_std::Addr, SyncDaemon) {
    let id = Id::new();
    let daemon = SyncDaemon::builder()
        .chain(LOCAL_JUNO)
        .handle(runtime.handle())
        .deployment_id(id.to_string())
        .build()
        .unwrap();

    let sender = daemon.sender();

    (sender, daemon)
}
