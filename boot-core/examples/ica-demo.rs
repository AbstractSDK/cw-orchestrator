use boot_core::{interchain::infrastructure::InterchainInfrastructure, prelude::*};
use boot_core::{networks::LOCAL_JUNO, DaemonOptionsBuilder};
use boot_cw_plus::{Cw20, CW20_BASE};
use cosmwasm_std::{Addr, Empty};
use simple_ica_controller::msg as controller_msgs;
use simple_ica_host::msg as host_msgs;
use std::sync::Arc;

const CRATE_PATH: &str = env!("CARGO_MANIFEST_DIR");

#[boot_contract(
    controller_msgs::InstantiateMsg,
    controller_msgs::ExecuteMsg,
    controller_msgs::QueryMsg,
    Empty
)]
struct Controller;

impl<Chain: BootEnvironment> Controller<Chain> {
    pub fn new(name: &str, chain: Chain) -> Self {
        let mut contract = Contract::new(name, chain);
        contract = contract.with_wasm_path(format!(
            "{CRATE_PATH}/examples/wasms/simple_ica_controller.wasm"
        ));
        Self(contract)
    }
}

#[boot_contract(host_msgs::InstantiateMsg, Empty, host_msgs::QueryMsg, Empty)]
struct Host;
impl<Chain: BootEnvironment> Host<Chain> {
    pub fn new(name: &str, chain: Chain) -> Self {
        let mut contract = Contract::new(name, chain);
        contract =
            contract.with_wasm_path(format!("{CRATE_PATH}/examples/wasms/simple_ica_host.wasm"));
        Self(contract)
    }
}

// just for uploading
#[boot_contract(Empty, Empty, Empty, Empty)]
struct Cw1;

impl<Chain: BootEnvironment> Cw1<Chain> {
    pub fn new(name: &str, chain: Chain) -> Self {
        let mut contract = Contract::new(name, chain);
        contract =
            contract.with_wasm_path(format!("{CRATE_PATH}/examples/wasms/cw1_whitelist.wasm"));
        Self(contract)
    }
}

// Requires a running local junod with grpc enabled
pub fn script() -> anyhow::Result<()> {
    let rt = Arc::new(tokio::runtime::Runtime::new().unwrap());

    let interchain = InterchainInfrastructure::new(&rt)?;

    let mut cw1 = Cw1::new("cw1", interchain.chain_a.clone());
    let mut host = Host::new("host", interchain.chain_a.clone());
    let mut controller = Controller::new("controller", interchain.chain_b);

    // cw1.upload()?;
    // host.upload()?;
    // controller.upload()?;

    // host.instantiate(
    //     &host_msgs::InstantiateMsg {
    //         cw1_code_id: cw1.code_id()?,
    //     },
    //     None,
    //     None,
    // )?;

    // controller.instantiate(&controller_msgs::InstantiateMsg {}, None, None)?;

    let res: controller_msgs::ListAccountsResponse =
        controller.query(&controller_msgs::QueryMsg::ListAccounts {})?;
    println!("res: {:?}", res);
    Ok(())
}

fn main() {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = script() {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));
        ::std::process::exit(1);
    }
}
