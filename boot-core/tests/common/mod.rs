use std::{env, sync::Mutex, thread::sleep, time::Duration};

use ctor::{ctor, dtor};
use duct::cmd;
use once_cell::sync::Lazy;

static mut DOCKER_CONTAINER_ID: Lazy<String> = Lazy::new(|| String::new());
static INIT: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

const STATE_FILE: &str = "/tmp/boot_test.json";
const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

#[ctor]
fn docker_container_setup() {
    let mut init = INIT.lock().unwrap();
    env_logger::init();

    // Set environment variables
    // this does not seems to be working in this case
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug");
    }

    if env::var("STATE_FILE").is_err() {
        env::set_var("STATE_FILE", STATE_FILE);
    }

    if env::var("LOCAL_MNEMONIC").is_err() {
        env::set_var("LOCAL_MNEMONIC", LOCAL_MNEMONIC);
    }

    log::info!("using RUST_LOG: {}", env::var("RUST_LOG").unwrap());
    log::info!("using STATE_FILE: {}", env::var("STATE_FILE").unwrap());
    log::info!(
        "using LOCAL_MNEMONIC: {}",
        env::var("LOCAL_MNEMONIC").unwrap()
    );

    if !*init {
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
fn docker_container_stop() {
    unsafe {
        if !DOCKER_CONTAINER_ID.is_empty() {
            log::info!("stopping container: {}", &*DOCKER_CONTAINER_ID);
            let _ = cmd!("docker", "container", "stop", &*DOCKER_CONTAINER_ID).read();

            log::info!("removing container: {}", &*DOCKER_CONTAINER_ID);
            let _ = cmd!("docker", "container", "rm", &*DOCKER_CONTAINER_ID).read();
        }
    }
}
