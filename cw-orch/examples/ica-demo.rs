//! # ICA Demo
//!
//! Uses the cosmwasm IBC demo repo to showcase cw-orch's IBC capabilities.
//! repo: https://github.com/confio/cw-ibc-demo
//!
//! ## Setup
//!
//! Clone interchaintest (used to spin up the nodes and relayer)
//! ```bash
//! git clone https://github.com/AbstractSDK/interchaintest.git
//! ```
//!
//! Now spin up the environment:
//! ```bash
//! cd interchaintest
//! go test examples/ibc/cw_ibc_test.go
//! ```
//! Wait a minute for the environment to be spun up.
//! Then run this script
//!
//! ```bash
//! cargo run --example ica-demo
//! ```
//!
//! ## What it does
//! This script starts by creating an `Interchain` object that connects with the locally running blockchain nodes. These nodes are spun up by interchaintest as a preparation for the test.
//!
//! ## Resources
//!
//! [Python/notebook ibc relayer](https://github.com/bear-market-labs/pybc-relayer)
//! [Cosmwasm IBC demo repo](https://github.com/confio/cw-ibc-demo)
//! [Hermes](https://hermes.informal.systems/)
//! [Interchaintest](https://github.com/strangelove-ventures/interchaintest)

use cosmwasm_std::{Empty};
use cw_orch::{
    channel::ChannelAccess,
    ibc_tracker::{CwIbcContractState, IbcTracker, IbcTrackerConfigBuilder},
    networks::{osmosis::OSMO_2, JUNO_1},
    *,
};
use simple_ica_controller::msg::{self as controller_msgs};
use simple_ica_host::msg::{self as host_msgs};
use std::sync::Arc;

const CRATE_PATH: &str = env!("CARGO_MANIFEST_DIR");
const JUNO_MNEMONIC: &str = "dilemma imitate split detect useful creek cart sort grow essence fish husband seven hollow envelope wedding host dry permit game april present panic move";
const OSMOSIS_MNEMONIC: &str = "settle gas lobster judge silk stem act shoulder pluck waste pistol word comfort require early mouse provide marine butter crowd clock tube move wool";
const JUNO: &str = "juno-1";
const OSMOSIS: &str = "osmosis-2";

#[contract(
    controller_msgs::InstantiateMsg,
    controller_msgs::ExecuteMsg,
    controller_msgs::QueryMsg,
    Empty
)]
struct Controller;

impl<Chain: CwEnv> Controller<Chain> {
    pub fn new(name: &str, chain: Chain) -> Self {
        let contract = Contract::new(name, chain);
        Self(contract)
    }
}

impl Uploadable<Daemon> for Controller<Daemon> {
    fn source(&self) -> <Daemon as TxHandler>::ContractSource {
        WasmPath::new(format!(
            "{CRATE_PATH}/examples/wasms/simple_ica_controller.wasm"
        ))
        .unwrap()
    }
}

#[contract(host_msgs::InstantiateMsg, Empty, host_msgs::QueryMsg, Empty)]
struct Host;
impl<Chain: CwEnv> Host<Chain> {
    pub fn new(name: &str, chain: Chain) -> Self {
        let contract = Contract::new(name, chain);
        Self(contract)
    }
}

impl Uploadable<Daemon> for Host<Daemon> {
    fn source(&self) -> <Daemon as TxHandler>::ContractSource {
        WasmPath::new(format!("{CRATE_PATH}/examples/wasms/simple_ica_host.wasm")).unwrap()
    }
}

// just for uploading
#[contract(Empty, Empty, Empty, Empty)]
struct Cw1;
impl<Chain: CwEnv> Cw1<Chain> {
    pub fn new(name: &str, chain: Chain) -> Self {
        let contract = Contract::new(name, chain);
        Self(contract)
    }
}

impl Uploadable<Daemon> for Cw1<Daemon> {
    fn source(&self) -> <Daemon as TxHandler>::ContractSource {
        WasmPath::new(format!("{CRATE_PATH}/examples/wasms/cw1_whitelist.wasm")).unwrap()
    }
}

// Requires a running local junod with grpc enabled
pub fn script() -> anyhow::Result<()> {
    let rt: Arc<tokio::runtime::Runtime> = Arc::new(tokio::runtime::Runtime::new().unwrap());

    let interchain = InterchainInfrastructure::new(
        &rt,
        vec![(JUNO_1, JUNO_MNEMONIC), (OSMO_2, OSMOSIS_MNEMONIC)],
    )?;

    let juno = interchain.daemon(JUNO)?;
    let osmosis = interchain.daemon(OSMOSIS)?;

    let cw1 = Cw1::new("cw1", juno.clone());
    let host = Host::new("host", juno.clone());
    let controller = Controller::new("controller", osmosis.clone());

    // ### SETUP ###
    deploy_contracts(&cw1, &host, &controller)?;
    interchain
        .hermes
        .create_channel(&rt, "connection-0", "simple-ica-v2", &controller, &host);

    // Track IBC on JUNO
    let juno_channel = juno.channel();
    let tracker = IbcTrackerConfigBuilder::default()
        .ibc_state(CwIbcContractState::new(
            "connection-0",
            format!("wasm.{}", host.addr_str()?),
        ))
        // .log_level(log::LevelFilter::Info)
        .build()?;
    // spawn juno logging on a different thread.
    rt.spawn(async move {
        juno_channel.cron_log(tracker).await;
    });

    // Track IBC on OSMOSIS
    let osmosis_channel = osmosis.channel();
    let tracker = IbcTrackerConfigBuilder::default()
        .ibc_state(CwIbcContractState::new(
            "connection-0",
            format!("wasm.{}", controller.addr_str()?),
        ))
        // .log_level(log::LevelFilter::Info)
        .build()?;
    // spawn osmosis logging on a different thread.
    rt.spawn(async move {
        osmosis_channel.cron_log(tracker).await;
    });

    Ok(())
}

fn main() {
    dotenv().ok();
    // env_logger::init();

    use dotenv::dotenv;

    if let Err(ref err) = script() {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));
        ::std::process::exit(1);
    }
}

fn deploy_contracts(
    cw1: &Cw1<Daemon>,
    host: &Host<Daemon>,
    controller: &Controller<Daemon>,
) -> anyhow::Result<()> {
    cw1.upload()?;
    host.upload()?;
    controller.upload()?;
    host.instantiate(
        &host_msgs::InstantiateMsg {
            cw1_code_id: cw1.code_id()?,
        },
        None,
        None,
    )?;
    controller.instantiate(&controller_msgs::InstantiateMsg {}, None, None)?;
    Ok(())
}

// fn test_ica() {
//     // Get account information
//     // let res: controller_msgs::ListAccountsResponse =
//     //     controller.query(&controller_msgs::QueryMsg::ListAccounts {})?;
//     // println!("After channel creation: {:?}", res);
//     let remote_accounts: ListAccountsResponse =
//         host.query(&host_msgs::QueryMsg::ListAccounts {})?;
//     println!("Remote accounts: {:?}", remote_accounts);
//     let remote_account = remote_accounts.accounts[0].clone();
//     // send some funds to the remote account
//     let res = rt.block_on(juno.sender.bank_send(
//         &remote_account.account,
//         vec![cosmwasm_std::coin(100u128, "ujuno")],
//     ))?;
//     // println!("Send funds result: {:?}", res);
//     let channel = remote_account.channel_id;

//     controller.execute(
//         &controller_msgs::ExecuteMsg::SendFunds {
//             ica_channel_id: channel.clone(),
//             transfer_channel_id: "channel-0".to_string(),
//         },
//         Some(&[cosmwasm_std::coin(100u128, "uosmo")]),
//     )?;

//     // let cont_accounts: controller_msgs::ListAccountsResponse = controller.query(&controller_msgs::QueryMsg::ListAccounts {  })?;

//     // println!("Controller accounts: {:?}", cont_accounts);

//     controller.execute(
//         &controller_msgs::ExecuteMsg::SendMsgs {
//             channel_id: "channel-1".to_string(),
//             msgs: vec![CosmosMsg::Bank(cosmwasm_std::BankMsg::Burn {
//                 amount: vec![cosmwasm_std::coin(100u128, "ujuno")],
//             })],
//             callback_id: None,
//         },
//         None,
//     )?;

//     // // wait a bit
//     std::thread::sleep(std::time::Duration::from_secs(600));
//     // let balance_result: AccountResponse =
//     //     controller.query(&controller_msgs::QueryMsg::Account {
//     //         channel_id: channel,
//     //     })?;
//     // println!("Balance result: {:?}", balance_result);

// }

/*

hermes query channels  --chain juno-1
hermes query channels  --chain osmosis-2

hermes query packet pending --chain juno-1 --port wasm.juno1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqwrw37d --channel channel-1
hermes query packet pending --chain osmosis-2 --port wasm.osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9 --channel channel-1

hermes query packet commitments --chain juno-1 --port wasm.juno1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqwrw37d --channel channel-1
hermes query packet commitments --chain osmosis-2 --port wasm.osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9 --channel channel-1

hermes query packet pending-sends --chain osmosis-2 --port wasm.osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9 --channel channel-1
hermes query packet pending-sends --chain juno-1 --port wasm.juno1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqwrw37d --channel channel-1

hermes clear packets --chain juno-1 --port wasm.juno1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqwrw37d --channel channel-1
hermes clear packets --chain osmosis-2 --port wasm.osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9 --channel channel-1

hermes clear packets --chain osmosis-2 --port transfer --channel channel-0


 */
