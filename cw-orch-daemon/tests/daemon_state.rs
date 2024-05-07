use std::sync::Arc;

use cw_orch_core::environment::ChainState;
use cw_orch_daemon::{json_lock::JsonLockedState, networks::OSMOSIS_1, DaemonBuilder, DaemonError};

pub const DUMMY_MNEMONIC:&str = "chapter wrist alcohol shine angry noise mercy simple rebel recycle vehicle wrap morning giraffe lazy outdoor noise blood ginger sort reunion boss crowd dutch";
const TEST_STATE_FILE: &str = "./tests/test.json";
const TEST2_STATE_FILE: &str = "./tests/test2.json";

#[test]
#[serial_test::serial]
fn simultaneous_read() {
    let daemon = DaemonBuilder::default()
        .chain(OSMOSIS_1)
        .mnemonic(DUMMY_MNEMONIC)
        .state_path(TEST_STATE_FILE)
        .build()
        .unwrap();

    // Write to state something, don't forget to drop lock to avoid deadlock
    let mut daemon_state = daemon.daemon.state.lock().unwrap();
    daemon_state.set("test", "test", "test").unwrap();
    drop(daemon_state);

    let mut handles = vec![];
    for _ in 0..25 {
        let daemon_state = daemon.state();
        let handle = std::thread::spawn(move || {
            // Just make sure it outputs > 2 so we know state is shared
            let strong_count = Arc::strong_count(&daemon_state);
            dbg!(strong_count);

            let state_lock = daemon_state.lock().unwrap();
            state_lock.get("test").unwrap()
        });
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
#[serial_test::serial]
fn simultaneous_write() {
    let daemon = DaemonBuilder::default()
        .chain(OSMOSIS_1)
        .mnemonic(DUMMY_MNEMONIC)
        .state_path(TEST_STATE_FILE)
        .build()
        .unwrap();

    let mut handles = vec![];
    for i in 0..25 {
        let daemon_state = daemon.state();
        let handle = std::thread::spawn(move || {
            // Just make sure it outputs > 2 so we know state is shared
            let strong_count = Arc::strong_count(&daemon_state);
            dbg!(strong_count);
            let mut state_lock = daemon_state.lock().unwrap();
            state_lock
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
    maybe_err.unwrap();
}

#[test]
#[serial_test::serial]
fn simultaneous_write_rebuilt() {
    let daemon = DaemonBuilder::default()
        .chain(OSMOSIS_1)
        .mnemonic(DUMMY_MNEMONIC)
        .state_path(TEST_STATE_FILE)
        .build()
        .unwrap();

    let mut handles = vec![];
    // Note this one has lower iterations since it rebuild is pretty long process
    for i in 0..10 {
        let daemon = daemon.rebuild().build().unwrap();
        let daemon_state = daemon.state();
        let handle = std::thread::spawn(move || {
            // Just make sure it outputs > 2 so we know state is shared
            let strong_count = Arc::strong_count(&daemon_state);
            dbg!(strong_count);
            let mut state_lock = daemon_state.lock().unwrap();
            state_lock
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
#[serial_test::serial]
#[should_panic]
fn panic_when_someone_holds_json_file() {
    match unsafe { nix::unistd::fork() } {
        Ok(nix::unistd::ForkResult::Child) => {
            // Occur lock for file for 100 millis
            let _state = JsonLockedState::new(TEST_STATE_FILE);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        Ok(nix::unistd::ForkResult::Parent { .. }) => {
            // Wait a bit for child to occur lock and try to lock already locked file by child
            std::thread::sleep(std::time::Duration::from_millis(50));
            let _state = JsonLockedState::new(TEST_STATE_FILE);
        }
        Err(_) => (),
    }
}

#[test]
#[serial_test::serial]
fn error_when_another_daemon_holds_it() {
    let _daemon = DaemonBuilder::default()
        .chain(OSMOSIS_1)
        .mnemonic(DUMMY_MNEMONIC)
        .state_path(TEST_STATE_FILE)
        .build()
        .unwrap();

    let daemon_res = DaemonBuilder::default()
        .chain(OSMOSIS_1)
        .mnemonic(DUMMY_MNEMONIC)
        .state_path(TEST_STATE_FILE)
        .build();

    assert!(matches!(
        daemon_res,
        Err(DaemonError::StateAlreadyLocked(_))
    ));
}

#[test]
#[serial_test::serial]
fn does_not_error_when_previous_daemon_dropped_state() {
    let daemon = DaemonBuilder::default()
        .chain(OSMOSIS_1)
        .mnemonic(DUMMY_MNEMONIC)
        .state_path(TEST_STATE_FILE)
        .build()
        .unwrap();

    drop(daemon);

    let daemon_res = DaemonBuilder::default()
        .chain(OSMOSIS_1)
        .mnemonic(DUMMY_MNEMONIC)
        .state_path(TEST_STATE_FILE)
        .build();

    assert!(daemon_res.is_ok(),);
}

#[test]
#[serial_test::serial]
fn does_not_error_when_using_different_files() {
    let _daemon = DaemonBuilder::default()
        .chain(OSMOSIS_1)
        .mnemonic(DUMMY_MNEMONIC)
        .state_path(TEST_STATE_FILE)
        .build()
        .unwrap();

    let daemon_res = DaemonBuilder::default()
        .chain(OSMOSIS_1)
        .mnemonic(DUMMY_MNEMONIC)
        // Different file
        .state_path(TEST2_STATE_FILE)
        .build();

    assert!(daemon_res.is_ok());
}
