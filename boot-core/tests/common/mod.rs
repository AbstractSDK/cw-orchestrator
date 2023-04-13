use std::{env, fs, sync::Mutex, thread::sleep, time::Duration};

use ctor::{ctor, dtor};
use duct::cmd;
use once_cell::sync::Lazy;

static mut DOCKER_CONTAINER_ID: Lazy<String> = Lazy::new(|| String::new());
static INIT: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

// Defaults for env vars
const CONTAINER_NAME: &str = "juno_node_1";
const STATE_FILE: &str = "/tmp/boot_test.json";
const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

fn find_container(name: &String) -> Option<String> {
    cmd!("docker", "container", "ls", "--all")
        .pipe(cmd!("grep", name))
        .pipe(cmd!("rev"))
        .pipe(cmd!("cut", "-d", r#" "#, "-f1"))
        .pipe(cmd!("rev"))
        .read()
        .ok()
}

fn remove_container(container: &String) -> duct::Expression {
    cmd!("docker", "container", "rm", container)
}

fn ensure_container_removal(container: &String) {
    let container_exists = find_container(&container);

    if container_exists.is_some() {
        log::info!("Container {} exists", container);
        log::info!("Removing container: {}", container);
        let _ = remove_container(&container).read();
    }
}

#[ctor]
fn docker_container_setup() {
    let mut init = INIT.lock().unwrap();

    env_logger::init();

    // Set environment variables
    // this does not seems to be working in this case
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug");
    }

    if env::var("CONTAINER_NAME").is_err() {
        env::set_var("CONTAINER_NAME", CONTAINER_NAME);
    }
    let container = env::var("CONTAINER_NAME").unwrap();

    if env::var("STATE_FILE").is_err() {
        env::set_var("STATE_FILE", STATE_FILE);
    }

    if env::var("LOCAL_MNEMONIC").is_err() {
        env::set_var("LOCAL_MNEMONIC", LOCAL_MNEMONIC);
    }

    log::info!("Using RUST_LOG: {}", env::var("RUST_LOG").unwrap());
    log::info!("Using CONTAINER_NAME: {}", container);
    log::info!("Using STATE_FILE: {}", env::var("STATE_FILE").unwrap());
    log::info!(
        "Using LOCAL_MNEMONIC: {}",
        env::var("LOCAL_MNEMONIC").unwrap()
    );

    if !*init {
        ensure_container_removal(&container);

        // Start Docker with the appropriate ports and the provided environment variables and script
        let container_id = cmd!(
            "docker",
            "run",
            "-d", // Run the container in detached mode
            "--name",
            container,
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
            // If container_id is already present skip this unsafe enclosure
            let Some(container_id) = container_id.as_ref() else {
                return;
            };

            let id = container_id.trim().to_string();
            log::info!("Started Docker container with ID: {}", id);
            *DOCKER_CONTAINER_ID = id;
        }
    }

    // Wait for the daemon to start
    sleep(Duration::from_secs(6));
}

#[dtor]
fn docker_container_stop() {
    log::info!("running docker_container_stop");

    unsafe {
        if !DOCKER_CONTAINER_ID.is_empty() {
            log::info!(
                "Stopping container: {}:{}",
                env::var("CONTAINER_NAME").unwrap(),
                &*DOCKER_CONTAINER_ID
            );
            let _ = cmd!("docker", "container", "stop", &*DOCKER_CONTAINER_ID).read();
        }
    }

    ensure_container_removal(&env::var("CONTAINER_NAME").unwrap());

    // we need to use /tmp/boot_test_local.json instead of /tmp/boot_test.json
    // because the state file gets renamed when the network is local
    log::info!("Removing STATE_FILE: {}", "/tmp/boot_test_local.json");
    let _ = fs::remove_file("/tmp/boot_test_local.json");
}
