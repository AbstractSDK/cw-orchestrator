#[cfg(feature = "node-tests")]
pub use node::*;

#[cfg(feature = "node-tests")]
mod node {
    use libc_print::libc_println;
    use std::{env, fs, path::Path, thread::sleep, time::Duration};

    use ctor::{ctor, dtor};

    use cw_orch_daemon::env::DaemonEnvVars;
    use duct::cmd;

    // Config
    const JUNO_IMAGE: &str = "ghcr.io/cosmoscontracts/juno:v12.0.0";
    #[allow(unused)]
    pub const STAKE_TOKEN: &str = "ujunox";

    // Defaults for env vars
    const CONTAINER_NAME: &str = "juno_node_1";
    // From https://github.com/CosmosContracts/juno/blob/32568dba828ff7783aea8cb5bb4b8b5832888255/docker/test-user.env#L2
    const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

    pub mod state_file {
        use super::*;
        pub fn exists(file: &str) -> bool {
            if Path::new(file).exists() {
                libc_println!("File found: {}", file);
                true
            } else {
                libc_println!("File not found: {}", file);
                false
            }
        }

        pub fn remove(file: &str) {
            if self::exists(file) {
                libc_println!("Removing state file: {}", file);
                let _ = fs::remove_file(file);
            }
        }
    }

    pub mod container {
        use super::cmd;
        use super::*;
        use crate::common::STAKE_TOKEN;

        pub fn find(name: &String) -> bool {
            let read = cmd!("docker", "container", "ls", "--all")
                .pipe(cmd!("grep", name))
                .pipe(cmd!("rev"))
                .pipe(cmd!("cut", "-d", r#" "#, "-f1"))
                .pipe(cmd!("rev"))
                .read();

            match read {
                Ok(val) => {
                    libc_println!("Container found: {}", name);
                    val == *name
                }
                Err(_) => false,
            }
        }

        pub fn start(name: &String, image: &String) -> bool {
            if self::find(name) {
                return false;
            }

            // Start Docker with the appropriate ports and the provided environment variables and script
            cmd!(
                "docker",
                "run",
                "-d", // Run the container in detached mode
                "--name",
                name,
                "-p",
                "1317:1317",
                "-p",
                "26656:26656",
                "-p",
                "9090:9090",
                "-e",
                format!("STAKE_TOKEN={}", STAKE_TOKEN),
                "-e",
                "UNSAFE_CORS=true",
                image,
                "./setup_and_run.sh",
                "juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y"
            )
            .read()
            .is_ok()
        }

        pub fn stop(name: &String) -> bool {
            if !self::find(name) {
                return true;
            }

            libc_println!("Stopping container: {}", name);

            let res = cmd!("docker", "container", "stop", name)
                .read()
                .ok()
                .unwrap();

            res == *name
        }

        pub fn remove(name: &String) -> bool {
            if !self::find(name) {
                return true;
            }

            libc_println!("Removing container: {}", name);

            let res = cmd!("docker", "container", "rm", name).read().ok().unwrap();

            res == *name
        }

        pub fn ensure_removal(name: &String) {
            if self::stop(name) {
                self::remove(name);
            }
        }
    }

    pub fn docker_container_start() {
        libc_println!("Running docker_container_start");

        // Set environment variables
        // this does not seems to be working in this case
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "debug");
        }

        if env::var("CONTAINER_NAME").is_err() {
            env::set_var("CONTAINER_NAME", CONTAINER_NAME);
        }
        let container = env::var("CONTAINER_NAME").unwrap();

        if env::var("JUNO_IMAGE").is_err() {
            env::set_var("JUNO_IMAGE", JUNO_IMAGE);
        }
        let image = env::var("JUNO_IMAGE").unwrap();

        if DaemonEnvVars::local_mnemonic().is_none() {
            env::set_var("LOCAL_MNEMONIC", LOCAL_MNEMONIC);
        }

        libc_println!("Using RUST_LOG: {}", env::var("RUST_LOG").unwrap());
        libc_println!("Using CONTAINER_NAME: {}", &container);
        libc_println!(
            "Using STATE_FILE: {}",
            DaemonEnvVars::state_file().display()
        );
        libc_println!(
            "Using LOCAL_MNEMONIC: {:?}",
            DaemonEnvVars::local_mnemonic()
        );

        container::start(&container, &image);

        // Wait for docker to start
        sleep(Duration::from_secs(10));
    }

    pub fn docker_container_stop() {
        libc_println!("Running docker_container_stop");
        container::ensure_removal(&env::var("CONTAINER_NAME").unwrap());
        let temp_dir = env::temp_dir();
        let expected_state_file = temp_dir.join("cw_orch_test_local.json");
        if let Some(state_file) = expected_state_file.to_str() {
            state_file::remove(state_file);
        }
    }

    #[ctor]
    fn common_start() {
        docker_container_start();
        libc_println!("Finish start");
    }

    #[dtor]
    fn common_stop() {
        docker_container_stop()
    }

    #[cfg(test)]
    #[allow(unused)]
    pub fn enable_logger() {
        let _ = env_logger::Builder::new()
            .filter_level(log::LevelFilter::Debug)
            .try_init();
    }
}
