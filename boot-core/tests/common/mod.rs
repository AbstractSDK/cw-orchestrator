// Code that is run when the test suite is started
// Initializes the [wasmd](https://github.com/CosmWasm/wasmd) daemon to perform tests against a mock blockchain.
// requires dockerd to be running

use std::{sync::Mutex, thread::sleep, time::Duration};

use ctor::{ctor, dtor};
use duct::{cmd, ReaderHandle};
use libc_print::libc_eprintln;
use once_cell::sync::Lazy;

// Use a global variable to store the container ID
static mut DOCKER_CONTAINER_ID: Lazy<String> = Lazy::new(|| String::new());
static INIT: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

#[ctor]
fn daemon_setup() {
    let mut init = INIT.lock().unwrap();
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
        .read().ok();

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
}

#[dtor]
fn shutdown_daemon() {
    unsafe{
        if !DOCKER_CONTAINER_ID.is_empty() {
            let _ = cmd!("docker", "container", "stop", &*DOCKER_CONTAINER_ID).read();
        }
    }
}