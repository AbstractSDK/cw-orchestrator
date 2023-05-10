use tokio::runtime::Runtime;

use uid::Id as IdT;

#[derive(Copy, Clone, Eq, PartialEq)]
struct DeployId(());

type Id = IdT<DeployId>;

use cw_orch::{daemon::networks::LOCAL_JUNO, prelude::Daemon};

/// Get the test-daemon object (local juno)
pub fn daemon(runtime: &Runtime) -> (cosmwasm_std::Addr, Daemon) {
    let id = Id::new();
    let daemon = Daemon::builder()
        .chain(LOCAL_JUNO)
        .handle(runtime.handle())
        .deployment_id(id.to_string())
        .build()
        .unwrap();

    let sender = daemon.sender.address().unwrap();

    (sender, daemon)
}
