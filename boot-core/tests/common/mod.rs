// Code that is run when the test suite is started
// Initializes the [wasmd](https://github.com/CosmWasm/wasmd) daemon to perform tests against a mock blockchain.
// requires dockerd to be running

use std::{env, sync::Mutex, thread::sleep, time::Duration};

use ctor::{ctor, dtor};
use duct::cmd;
use once_cell::sync::Lazy;

// Use a global variable to store the container ID
static mut DOCKER_CONTAINER_ID: Lazy<String> = Lazy::new(|| String::new());
static INIT: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

#[ctor]
fn daemon_setup() {
    let mut init = INIT.lock().unwrap();
    env_logger::init();

    // Set environment variables
    env::set_var("STATE_FILE", "/tmp/boot_test.json");
    env::set_var("RUST_LOG", "debug");
    env::set_var(
        "LOCAL_MNEMONIC",
        "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose",
    );

    if !*init {
        // Remove existing Docker container
        let _ = cmd!("docker", "container", "rm", "juno_node_1").read();

        sleep(Duration::from_secs(5));

        // Start Docker with the appropriate ports and the provided environment variables and script
        let container_id = cmd!(
            "docker",
            "run",
            "-d", // Run the container in detached mode
            "--name",
            "juno_node_1",
            "-p",
            "1317:1317",
            "-p",
            "26656:26656",
            "-p",
            "9090:9090",
            "-e",
            "STAKE_TOKEN=ujunox",
            "-e",
            "UNSAFE_CORS=true",
            "ghcr.io/cosmoscontracts/juno:v12.0.0",
            "./setup_and_run.sh",
            "juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y"
        )
        .read()
        .ok();

        // Mark initialization as complete
        *init = true;

        // Save the container ID to the global variable
        unsafe {
            let Some(container_id) = container_id.as_ref() else {
                return;
            };
            let id = container_id.trim().to_string();
            // libc_eprintln!("Started Docker container with ID: {}", id);
            *DOCKER_CONTAINER_ID = id;
        }
    }

    // Wait for the daemon to start
    sleep(Duration::from_secs(8));
}

#[dtor]
fn shutdown_daemon() {
    unsafe {
        if !DOCKER_CONTAINER_ID.is_empty() {
            let _ = cmd!("docker", "container", "stop", &*DOCKER_CONTAINER_ID).read();
        }
    }
}

pub mod cw20 {
    use boot_core::{contract, Contract, ContractWrapper, CwEnv};
    use cw20_base::msg::*;

    #[contract(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
    pub struct Cw20Base;

    // Implement chain-generic functions
    impl<Chain: CwEnv> Cw20Base<Chain> {
        pub fn new(chain: Chain) -> Self {
            let crate_path = env!("CARGO_MANIFEST_DIR");
            let file_path = &format!("{}{}", crate_path, "/tests/artifacts/cw20_base.wasm");
            Self(
                Contract::new("cw-plus:cw20_base", chain)
                    .with_mock(Box::new(
                        ContractWrapper::new_with_empty(
                            cw20_base::contract::execute,
                            cw20_base::contract::instantiate,
                            cw20_base::contract::query,
                        )
                        .with_migrate(cw20_base::contract::migrate),
                    ))
                    .with_wasm_path(file_path),
            )
        }
    }
}
