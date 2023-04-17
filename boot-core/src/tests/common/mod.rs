#[cfg(test)]
pub mod common {
    use std::{env, fs, path::Path, sync::Mutex, thread::sleep, time::Duration};

    use ctor::{ctor, dtor};
    // use ctor::ctor;
    use duct::cmd;

    use std::sync::Arc;

    use tokio::runtime::Runtime;

    use boot_core::{instantiate_daemon_env, Contract, ContractWrapper, DaemonOptionsBuilder};

    pub static mut CONTRACT_COUNTER: Mutex<u8> = Mutex::new(0);

    const CW20_CONTRACT_WASM: &str = "/../boot-cw-plus/cw-artifacts/cw20_base.wasm";

    // Defaults for env vars
    const CONTAINER_NAME: &str = "juno_node_1";
    const STATE_FILE: &str = "/tmp/boot_test.json";
    const LOCAL_MNEMONIC: &str = "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

    pub mod state_file {
        use super::{fs, Path};

        pub fn exists(file: &str) -> bool {
            Path::new(file).exists()
        }

        pub fn remove(file: &str) {
            if self::exists(file) {
                log::info!("Removing state file: {}", file);
                let _ = fs::remove_file(file);
            }
        }
    }

    pub mod container {
        use super::cmd;

        pub fn find(name: &String) -> bool {
            let read = cmd!("docker", "container", "ls", "--all")
                .pipe(cmd!("grep", name))
                .pipe(cmd!("rev"))
                .pipe(cmd!("cut", "-d", r#" "#, "-f1"))
                .pipe(cmd!("rev"))
                .read();

            match read {
                Ok(val) => {
                    log::info!("Container found: {}", name);
                    val == *name
                }
                Err(_) => false,
            }
        }

        pub fn start(name: &String) -> bool {
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
                "STAKE_TOKEN=ujunox",
                "-e",
                "UNSAFE_CORS=true",
                "ghcr.io/cosmoscontracts/juno:v14.0.0",
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

            log::info!("Stopping container: {}", name);

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

            log::info!("Removing container: {}", name);

            let res = cmd!("docker", "container", "rm", name).read().ok().unwrap();

            res == *name
        }

        pub fn ensure_removal(name: &String) {
            if self::stop(name) {
                self::remove(&name);
            }
        }
    }

    pub(crate) fn start_contract() -> (cosmwasm_std::Addr, boot_core::Contract<boot_core::Daemon>) {
        unsafe {
            let counter = CONTRACT_COUNTER.lock().unwrap().checked_add(1u8).unwrap();
            CONTRACT_COUNTER = counter.into();
            log::info!("Contract starts: {}", counter);
        }

        let runtime = Arc::new(Runtime::new().unwrap());

        let options = DaemonOptionsBuilder::default()
            .network(boot_core::networks::LOCAL_JUNO)
            .deployment_id("v0.1.0")
            .build()
            .unwrap();

        let (sender, chain) = instantiate_daemon_env(&runtime, options).unwrap();

        // create contract base configuration
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let wasm_path = format!("{}{}", crate_path, CW20_CONTRACT_WASM);
        log::info!("Using wasm path {}", wasm_path);

        let contract = Contract::new("cw-plus:cw20_base", chain)
            .with_mock(Box::new(
                ContractWrapper::new_with_empty(
                    cw20_base::contract::execute,
                    cw20_base::contract::instantiate,
                    cw20_base::contract::query,
                )
                .with_migrate(cw20_base::contract::migrate),
            ))
            .with_wasm_path(wasm_path);

        let v = crate::utils::json::read(&env::var("STATE_FILE").unwrap());
        println!("{:#?}", v);

        (sender, contract)
    }

    pub fn docker_container_start() {
        log::info!("Running docker_container_start");

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

        container::start(&container);

        // Wait for docker to start
        sleep(Duration::from_secs(6));
    }

    pub fn docker_container_stop() {
        log::info!("Running docker_container_stop");
        container::ensure_removal(&env::var("CONTAINER_NAME").unwrap());
        state_file::remove(&env::var("STATE_FILE").unwrap());
    }

    #[ctor]
    fn common_start() {
        env_logger::init();
        docker_container_start()
    }

    #[dtor]
    fn common_stop() {
        docker_container_stop()
    }
}
