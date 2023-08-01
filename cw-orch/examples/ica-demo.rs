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

use cw_orch::interchain::interchain_env::contract_port;
use cw_orch::prelude::networks::osmosis::OSMO_2;
use cw_orch::prelude::*;

use crate::daemon::networks::*;
use crate::prelude::interchain_channel_builder::InterchainChannelBuilder;
use crate::prelude::queriers::Bank;
use crate::prelude::InterchainEnv;
use crate::prelude::Uploadable;
use cosmwasm_std::{CosmosMsg, Empty};
use cw_orch::prelude::ContractInstance;
use cw_orch::prelude::CwOrchExecute;
use cw_orch::prelude::CwOrchUpload;
use cw_orch::prelude::TxHandler;
use cw_orch::state::ChainState;
use cw_orch::{prelude::WasmPath, *};
use simple_ica_controller::msg::{self as controller_msgs};
use simple_ica_host::msg::{self as host_msgs};
use speculoos::assert_that;

use crate::starship::Starship;
use cw_orch_interchain::ibc_tracker::IbcTracker;
use tokio::runtime::Handle;

const CRATE_PATH: &str = env!("CARGO_MANIFEST_DIR");
const JUNO_MNEMONIC: &str = "dilemma imitate split detect useful creek cart sort grow essence fish husband seven hollow envelope wedding host dry permit game april present panic move";
const OSMOSIS_MNEMONIC: &str = "settle gas lobster judge silk stem act shoulder pluck waste pistol word comfort require early mouse provide marine butter crowd clock tube move wool";
const JUNO: &str = "juno-1";
const OSMOSIS: &str = "osmosis-1";

pub fn script() -> anyhow::Result<()> {
    log::info!("test");
    let rt: tokio::runtime::Runtime = tokio::runtime::Runtime::new()?;

    // We create the test starship object
    let starship = Starship::new(rt.handle().to_owned(), None)?;

    let interchain: InterchainEnv = starship.interchain_env();

    let juno = interchain.daemon(JUNO)?;
    let osmosis = interchain.daemon(OSMOSIS)?;

    let cw1 = Cw1::new("cw1", juno.clone());
    let host = Host::new("host", juno.clone());
    let controller = Controller::new("controller", osmosis.clone());

    // ### SETUP ###
    deploy_contracts(&cw1, &host, &controller)?;

    let interchain_channel = rt.block_on(
        InterchainChannelBuilder::default()
            .from_contracts(&controller, &host)
            .create_channel(starship.client(), "simple-ica-v2"),
    )?;

    // test the ica implementation
    test_ica(rt.handle().clone(), &interchain, &controller, &juno)?;

    Ok(())
}

fn main() {
    dotenv().ok();
    use dotenv::dotenv;
    env_logger::init();

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

/// Test the cw-ica contract
fn test_ica(
    rt: Handle,
    interchain: &InterchainEnv,
    // controller on osmosis
    controller: &Controller<Daemon>,
    juno: &Daemon,
) -> anyhow::Result<()> {
    // get the information about the remote account
    let remote_accounts: controller_msgs::ListAccountsResponse =
        controller.query(&controller_msgs::QueryMsg::ListAccounts {})?;
    assert_that!(remote_accounts.accounts.len()).is_equal_to(1);

    // get the account information
    let remote_account = remote_accounts.accounts[0].clone();
    let remote_addr = remote_account.remote_addr.unwrap();
    let channel = remote_account.channel_id;

    // send some funds to the remote account
    rt.block_on(
        juno.wallet()
            .bank_send(&remote_addr, vec![cosmwasm_std::coin(100u128, "ujuno")]),
    )?;

    // assert that the remote account got funds
    let balance = rt.block_on(juno.query_client::<Bank>().balance(&remote_addr, "ujuno"))?;
    assert_that!(&balance.amount).is_equal_to(100u128.to_string());

    // burn the juno remotely
    let burn_response = controller.execute(
        &controller_msgs::ExecuteMsg::SendMsgs {
            channel_id: channel,
            msgs: vec![CosmosMsg::Bank(cosmwasm_std::BankMsg::Burn {
                amount: vec![cosmwasm_std::coin(100u128, "ujuno")],
            })],
            callback_id: None,
        },
        None,
    )?;

    // Folow the transaction execution
    rt.block_on(
        interchain.await_ibc_execution(
            controller
                .get_chain()
                .state()
                .0
                .chain_data
                .chain_id
                .to_string(),
            burn_response.txhash,
        ),
    )?;

    // check that the balance became 0
    let balance = rt.block_on(juno.query_client::<Bank>().balance(&remote_addr, "ujuno"))?;
    assert_that!(&balance.amount).is_equal_to(0u128.to_string());
    Ok(())
}

// Contract interface definitions

#[interface(
    controller_msgs::InstantiateMsg,
    controller_msgs::ExecuteMsg,
    controller_msgs::QueryMsg,
    Empty
)]
struct Controller;

impl Uploadable for Controller<Daemon> {
    fn wasm(&self) -> <Daemon as TxHandler>::ContractSource {
        WasmPath::new(format!(
            "{CRATE_PATH}/examples/wasms/simple_ica_controller.wasm"
        ))
        .unwrap()
    }
}

#[interface(host_msgs::InstantiateMsg, Empty, host_msgs::QueryMsg, Empty)]
struct Host;
impl Uploadable for Host<Daemon> {
    fn wasm(&self) -> <Daemon as TxHandler>::ContractSource {
        WasmPath::new(format!("{CRATE_PATH}/examples/wasms/simple_ica_host.wasm")).unwrap()
    }
}

// just for uploading
#[interface(Empty, Empty, Empty, Empty)]
struct Cw1;

impl Uploadable for Cw1<Daemon> {
    fn wasm(&self) -> <Daemon as TxHandler>::ContractSource {
        WasmPath::new(format!("{CRATE_PATH}/examples/wasms/cw1_whitelist.wasm")).unwrap()
    }
}
