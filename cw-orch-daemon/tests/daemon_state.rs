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

    let mut handles = vec![];
    for _ in 0..25 {
        let daemon_state = daemon_state.clone();
        let handle = std::thread::spawn(move || daemon_state.get("test").unwrap());
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn simultaneous_write() {
    let runtime = Runtime::new().unwrap();

    let chain_data: ChainRegistryData = JUNO_1.into();
    std::env::set_var(STATE_FILE_ENV_NAME, "./tests/test.json");

    let daemon_state = runtime
        .block_on(DaemonState::new(chain_data, "test".to_owned(), false))
        .unwrap();

    let mut handles = vec![];
    for i in 0..25 {
        let daemon_state = daemon_state.clone();
        let handle = std::thread::spawn(move || {
            daemon_state
                .set("test", &format!("test{i}"), format!("test-{i}"))
                .unwrap();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
