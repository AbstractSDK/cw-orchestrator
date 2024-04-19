use cw_orch_core::env::STATE_FILE_ENV_NAME;
use cw_orch_daemon::{json_file::JsonFileState, ChainRegistryData, DaemonState};
use cw_orch_networks::networks::JUNO_1;
use tokio::runtime::Runtime;

const TEST_STATE_FILE: &str = "./tests/test.json";

#[test]
fn simultaneous_read() {
    let runtime = Runtime::new().unwrap();

    let chain_data: ChainRegistryData = JUNO_1.into();
    std::env::set_var(STATE_FILE_ENV_NAME, TEST_STATE_FILE);

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

    let mut maybe_err = Ok(serde_json::Value::default());

    for handle in handles {
        let result = handle.join();
        if result.is_err() {
            maybe_err = result;
        }
    }
    // Error if at least one failed
    let _ = maybe_err.unwrap();
}

#[test]
fn simultaneous_write() {
    let runtime = Runtime::new().unwrap();

    let chain_data: ChainRegistryData = JUNO_1.into();
    std::env::set_var(STATE_FILE_ENV_NAME, TEST_STATE_FILE);

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

    let mut maybe_err = Ok(());
    // Finish all handles
    for handle in handles {
        let result = handle.join();
        if result.is_err() {
            maybe_err = result;
        }
    }
    // Error if at least one failed
    maybe_err.unwrap()
}

#[test]
#[should_panic]
fn panic_when_someone_else_holds_it() {
    match unsafe { nix::unistd::fork() } {
        Ok(nix::unistd::ForkResult::Child) => {
            // Occur lock for file for 100 millis
            let _state = JsonFileState::new(TEST_STATE_FILE);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        Ok(nix::unistd::ForkResult::Parent { .. }) => {
            // Wait a bit for child to occur lock and try to lock already locked file by child
            std::thread::sleep(std::time::Duration::from_millis(50));
            let _state = JsonFileState::new(TEST_STATE_FILE);
        }
        Err(_) => (),
    }
}
