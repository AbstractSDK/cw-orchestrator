/*

*/
#[cfg(test)]
mod contract {
    use std::{
        env,
        time::Duration,
        sync::Mutex,
        sync::Arc,
        thread::sleep,
    };

    use boot_core::contract;
    use ctor::{ctor, dtor};
    use once_cell::sync::Lazy;
    use duct::cmd;

    use tokio::runtime::Runtime;

    use cosmwasm_std::Uint128;
    use cw_multi_test::ContractWrapper;

    use speculoos::prelude::*;

    use boot_core::{
        networks::LOCAL_JUNO,
        Contract, DaemonOptionsBuilder,
        instantiate_daemon_env,
    };

    use cw20_base::msg::*;

    static mut DOCKER_CONTAINER_ID: Lazy<String> = Lazy::new(|| String::new());
    static INIT: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

    // const CW20_CONTRACT_WASM: &str = "/../boot-cw-plus/cw-artifacts/cw20_base.wasm";
    const CW20_CONTRACT_WASM: &str = "/../boot-cw-plus/cw-artifacts/cw20_base.wasm";
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
        log::info!("using LOCAL_MNEMONIC: {}", env::var("LOCAL_MNEMONIC").unwrap());

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

    #[contract(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
    pub struct Cw20Base;

    // #[test]
    // fn querier() {
    //     let rt = Arc::new(Runtime::new().unwrap());

    //     // configure daemon options
    //     let options = DaemonOptionsBuilder::default()
    //         .network(LOCAL_JUNO)
    //         .deployment_id("v0.1.0")
    //         .build()
    //         .unwrap();

    //     // instantiate chain daemon
    //     let (sender, chain) = instantiate_daemon_env(&rt, options).unwrap();

    // }

    #[test]
    fn contract() {
        let rt = Arc::new(Runtime::new().unwrap());

        // configure daemon options
        let options = DaemonOptionsBuilder::default()
            .network(LOCAL_JUNO)
            .deployment_id("v0.1.0")
            .build()
            .unwrap();

        // instantiate chain daemon
        let (sender, chain) = instantiate_daemon_env(&rt, options).unwrap();

        log::info!("got wallet {}", sender);

        // create contract base configuration
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let wasm_path = format!("{}{}", crate_path, CW20_CONTRACT_WASM);
        log::info!("wasm path {}", wasm_path);

        let mut contract = Contract::new("cw-plus:cw20_base", chain)
            .with_mock(Box::new(
                ContractWrapper::new_with_empty(
                    cw20_base::contract::execute,
                    cw20_base::contract::instantiate,
                    cw20_base::contract::query,
                )
                .with_migrate(cw20_base::contract::migrate),
            ))
            .with_wasm_path(wasm_path);

        // let upload_if_res = contract.upload_if_needed();
        // println!("upload_if_res: {:#?}", upload_if_res);

        // let is_latest_res = contract.is_running_latest();
        // println!("is_latest_res: {:#?}", is_latest_res);

        // upload contract
        let upload_res = contract.upload();
        asserting!("upload is succesful").that(&upload_res).is_ok();

        let code_id = upload_res.unwrap().logs[0].events[1].attributes[1].value.clone();

        log::info!("using code_id {}", code_id);

        let amount = Uint128::from(10000u128);

        // init msg for contract
        let init_msg = cw20_base::msg::InstantiateMsg {
            name: "Token".to_owned(),
            symbol: "TOK".to_owned(),
            decimals: 6u8,
            initial_balances: vec![
                cw20::Cw20Coin { address: sender.to_string(), amount }
            ],
            mint: None,
            marketing: None,
        };

        // instantiate contract on chain
        let init_res = contract.instantiate(&init_msg, Some(&sender.clone()), None);
        asserting!("instantiate is successful").that(&init_res).is_ok();

        // do a query and validate its successful
        let query_res = contract.query::<
            cw20_base::msg::QueryMsg,
            cw20::BalanceResponse
        >(&cw20_base::msg::QueryMsg::Balance { address: sender.to_string() });
        asserting!("query is successful").that(&query_res).is_ok();

        // validate migrations are successful
        let migrate_res = contract.migrate(&MigrateMsg {}, code_id.parse::<u64>().unwrap());
        asserting!("migrate is successful").that(&migrate_res).is_ok();

        println!("---------------------------------------------------------------------");

        let upload_if_res = contract.upload_if_needed();
        println!("upload_if_res: {:#?}", upload_if_res);

        let is_latest_res = contract.is_running_latest();
        println!("is_latest_res: {:#?}", is_latest_res);
    }
}