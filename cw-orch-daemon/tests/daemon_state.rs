use std::{thread, time::Duration};

use cw_orch_core::env::STATE_FILE_ENV_NAME;
use cw_orch_daemon::{ChainRegistryData, DaemonState};
use cw_orch_networks::networks::JUNO_1;
use tokio::runtime::Runtime;

#[test]
fn simultaneous_read() {
    let runtime = Runtime::new().unwrap();

    let chain_data: ChainRegistryData = JUNO_1.into();
    std::env::set_var(STATE_FILE_ENV_NAME, "./tests/test.json");

    let daemon_state = runtime
        .block_on(DaemonState::new(chain_data, "test".to_owned(), false))
        .unwrap();
    daemon_state.set("test", "test", "test").unwrap();
    for _ in 0..25 {
        let daemon_state = daemon_state.clone();
        std::thread::spawn(move || daemon_state.get("test"));
    }
    thread::sleep(Duration::from_millis(500));
}

#[test]
fn simultaneous_write() {
    let runtime = Runtime::new().unwrap();

    let chain_data: ChainRegistryData = JUNO_1.into();
    std::env::set_var(STATE_FILE_ENV_NAME, "./tests/test.json");

    let daemon_state = runtime
        .block_on(DaemonState::new(chain_data, "test".to_owned(), false))
        .unwrap();

    for i in 0..25 {
        let daemon_state = daemon_state.clone();
        std::thread::spawn(move || daemon_state.set("test", &format!("test{i}"), format!("test-{i}")));
    }
    thread::sleep(Duration::from_millis(500));
}
