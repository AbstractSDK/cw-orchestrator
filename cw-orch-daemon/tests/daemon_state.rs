use std::sync::Arc;

use cw_orch_core::environment::ChainState;
use cw_orch_daemon::{
    env::STATE_FILE_ENV_NAME,
    networks::{JUNO_1, NEUTRON_1},
    Daemon, DaemonBuilder, DaemonError, DaemonStateFile,
};

pub const DUMMY_MNEMONIC:&str = "chapter wrist alcohol shine angry noise mercy simple rebel recycle vehicle wrap morning giraffe lazy outdoor noise blood ginger sort reunion boss crowd dutch";

#[test]
#[serial_test::serial]
fn simultaneous_read() {
    let daemon = DaemonBuilder::new(JUNO_1)
        .mnemonic(DUMMY_MNEMONIC)
        .is_test(true)
        .build()
        .unwrap();

    // Write to state something
    let mut daemon_state = daemon.state();
    daemon_state.set("test", "test", "test").unwrap();
    drop(daemon_state);

    let mut handles = vec![];
    for _ in 0..25 {
        let daemon_state = daemon.state();
        let handle = std::thread::spawn(move || {
            if let DaemonStateFile::FullAccess { json_file_state } = &daemon_state.json_state {
                // Just make sure it outputs > 2 so we know state is shared
                let strong_count = Arc::strong_count(json_file_state);
                dbg!(strong_count);
            } else {
                unreachable!("It's full access daemon");
            }
            daemon_state.get("test").unwrap()
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
    std::env::remove_var(STATE_FILE_ENV_NAME);
}

#[test]
#[serial_test::serial]
fn simultaneous_write() {
    let daemon = DaemonBuilder::new(JUNO_1)
        .mnemonic(DUMMY_MNEMONIC)
        .is_test(true)
        .build()
        .unwrap();

    let mut handles = vec![];
    for i in 0..25 {
        let mut daemon_state = daemon.state();
        let handle = std::thread::spawn(move || {
            if let DaemonStateFile::FullAccess { json_file_state } = &daemon_state.json_state {
                // Just make sure it outputs > 2 so we know state is shared
                let strong_count = Arc::strong_count(json_file_state);
                dbg!(strong_count);
            } else {
                unreachable!("It's full access daemon");
            }
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
    maybe_err.unwrap();
    std::env::remove_var(STATE_FILE_ENV_NAME);
}

#[test]
#[serial_test::serial]
fn simultaneous_write_rebuilt() {
    let daemon = DaemonBuilder::new(JUNO_1)
        .mnemonic(DUMMY_MNEMONIC)
        .is_test(true)
        .build()
        .unwrap();

    let options = daemon.sender().options().clone();

    let mut handles = vec![];
    // Note this one has lower iterations since rebuild is pretty long process
    for i in 0..10 {
        let daemon: Daemon = daemon.rebuild().build_sender(options.clone()).unwrap();
        let mut daemon_state = daemon.state();
        let handle = std::thread::spawn(move || {
            if let DaemonStateFile::FullAccess { json_file_state } = &daemon_state.json_state {
                // Just make sure it outputs > 2 so we know state is shared
                let strong_count = Arc::strong_count(json_file_state);
                dbg!(strong_count);
            } else {
                unreachable!("It's full access daemon");
            }
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
    maybe_err.unwrap();
    std::env::remove_var(STATE_FILE_ENV_NAME);
}

#[test]
#[serial_test::serial]
fn error_when_another_daemon_holds_it() {
    let state_path = std::env::temp_dir()
        .join("daemon_state_test_file")
        .into_os_string();
    std::env::set_var(STATE_FILE_ENV_NAME, state_path);

    let _daemon = DaemonBuilder::new(JUNO_1)
        .mnemonic(DUMMY_MNEMONIC)
        .build()
        .unwrap();

    let daemon_res = DaemonBuilder::new(JUNO_1).mnemonic(DUMMY_MNEMONIC).build();

    assert!(matches!(
        daemon_res,
        Err(DaemonError::StateAlreadyLocked(_))
    ));
    std::env::remove_var(STATE_FILE_ENV_NAME);
}

#[test]
#[serial_test::serial]
fn does_not_error_when_previous_daemon_dropped_state() {
    let daemon = DaemonBuilder::new(JUNO_1)
        .mnemonic(DUMMY_MNEMONIC)
        .is_test(true)
        .build()
        .unwrap();

    drop(daemon);

    let daemon_res = DaemonBuilder::new(JUNO_1)
        .mnemonic(DUMMY_MNEMONIC)
        .is_test(true)
        .build();

    assert!(daemon_res.is_ok());
    std::env::remove_var(STATE_FILE_ENV_NAME);
}

#[test]
#[serial_test::serial]
fn does_not_error_when_using_different_files() {
    let _daemon = DaemonBuilder::new(JUNO_1)
        .mnemonic(DUMMY_MNEMONIC)
        .is_test(true)
        .build()
        .unwrap();

    // Different file
    let daemon_res = DaemonBuilder::new(JUNO_1)
        .mnemonic(DUMMY_MNEMONIC)
        // is test will produce new file every time
        .is_test(true)
        .build();

    assert!(daemon_res.is_ok());
    std::env::remove_var(STATE_FILE_ENV_NAME);
}

#[test]
#[serial_test::serial]
fn reuse_same_state_multichain() {
    let daemon = DaemonBuilder::new(JUNO_1)
        .mnemonic(DUMMY_MNEMONIC)
        .is_test(true)
        .build()
        .unwrap();

    let daemon_res = DaemonBuilder::new(NEUTRON_1)
        .state(daemon.state())
        .mnemonic(DUMMY_MNEMONIC)
        .build();

    assert!(daemon_res.is_ok());
    std::env::remove_var(STATE_FILE_ENV_NAME);
}

// #[test]
// #[serial_test::serial]
// #[should_panic]
// #[ignore = "Serial don't track forks for some reason, run it manually"]
// fn panic_when_someone_holds_json_file() {
//     match unsafe { nix::unistd::fork() } {
//         Ok(nix::unistd::ForkResult::Child) => {
//             // Occur lock for file for 100 millis
//             let _state = JsonLockedState::new(TEST_STATE_FILE);
//             std::thread::sleep(std::time::Duration::from_millis(100));
//         }
//         Ok(nix::unistd::ForkResult::Parent { .. }) => {
//             // Wait a bit for child to occur lock and try to lock already locked file by child
//             std::thread::sleep(std::time::Duration::from_millis(50));
//             let _state = JsonLockedState::new(TEST_STATE_FILE);
//         }
//         Err(_) => (),
//     }
// }
