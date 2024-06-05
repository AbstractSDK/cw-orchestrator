#![allow(unused)]
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

use cosmwasm_std::DepsMut;
use cosmwasm_std::Response;
use cosmwasm_std::StdError;
use cw_orch::interface;
use cw_orch::prelude::*;

use cosmwasm_std::{CosmosMsg, Empty, Env, MessageInfo, StdResult};
use cw_orch_interchain_core::IbcQueryHandler;
use cw_orch_interchain_core::InterchainEnv;
use cw_orch_interchain_core::InterchainError;
use simple_ica_controller::msg::{self as controller_msgs};
use simple_ica_host::msg::{self as host_msgs};

use speculoos::assert_that;

use super::bank::BankModule;

const CRATE_PATH: &str = env!("CARGO_MANIFEST_DIR");

pub fn full_ica_test<Chain: IbcQueryHandler + BankModule, IBC: InterchainEnv<Chain>>(
    interchain: &IBC,
    host_chain_id: &str,
    controller_chain_id: &str,
    host_funds_denom: &str,
) -> cw_orch::anyhow::Result<()> {
    let host_chain = interchain.chain(host_chain_id)?;
    let controller_chain = interchain.chain(controller_chain_id)?;

    let cw1 = Cw1::new("cw1", host_chain.clone());
    let host = Host::new("host", host_chain.clone());
    let controller = Controller::new("controller", controller_chain.clone());

    // ### SETUP ###
    deploy_contracts(&cw1, &host, &controller)?;

    interchain.create_contract_channel(
        &controller,
        &host,
        "simple-ica-v2",
        Some(cosmwasm_std::IbcOrder::Unordered),
    )?;

    // test the ica implementation
    test_ica(interchain, &controller, &host_chain, host_funds_denom)?;

    Ok(())
}

fn deploy_contracts<Chain: CwEnv>(
    cw1: &Cw1<Chain>,
    host: &Host<Chain>,
    controller: &Controller<Chain>,
) -> cw_orch::anyhow::Result<()> {
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
fn test_ica<Chain: IbcQueryHandler + BankModule, IBC: InterchainEnv<Chain>>(
    interchain: &IBC,
    // controller on osmosis
    controller: &Controller<Chain>,
    juno: &Chain,
    host_funds_denom: &str,
) -> Result<(), InterchainError> {
    // get the information about the remote account
    let remote_accounts: controller_msgs::ListAccountsResponse =
        controller.query(&controller_msgs::QueryMsg::ListAccounts {})?;
    assert_that!(remote_accounts.accounts.len()).is_equal_to(1);

    // get the account information
    let remote_account = remote_accounts.accounts[0].clone();
    let remote_addr = remote_account.remote_addr.unwrap();
    let channel = remote_account.channel_id;

    // send some funds to the remote account
    juno.send(
        &remote_addr,
        vec![cosmwasm_std::coin(100u128, host_funds_denom)],
    )
    .map_err(Into::into)?;

    // assert that the remote account got funds
    let balance = juno
        .bank_querier()
        .balance(&remote_addr, Some(host_funds_denom.to_string()))
        .map_err(Into::into)?;

    assert_that!(&balance[0].amount.u128()).is_equal_to(100);

    // burn the juno remotely
    let burn_response = controller.execute(
        &controller_msgs::ExecuteMsg::SendMsgs {
            channel_id: channel,
            msgs: vec![CosmosMsg::Bank(cosmwasm_std::BankMsg::Burn {
                amount: vec![cosmwasm_std::coin(100u128, host_funds_denom)],
            })],
            callback_id: None,
        },
        None,
    )?;

    let chain_id = controller
        .get_chain()
        .block_info()
        .map_err(Into::into)?
        .chain_id
        .to_string();

    // Follow the transaction execution
    interchain.check_ibc(&chain_id, burn_response)?;

    // check that the balance became 0
    let balance = juno
        .bank_querier()
        .balance(&remote_addr, Some(host_funds_denom.to_string()))
        .map_err(Into::into)?;
    assert_that!(&balance[0].amount.u128()).is_equal_to(0);
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

impl<Chain: CwEnv> Uploadable for Controller<Chain> {
    fn wasm(_chain: &ChainInfoOwned) -> <Daemon as TxHandler>::ContractSource {
        WasmPath::new(format!("{CRATE_PATH}/wasms/simple_ica_controller.wasm")).unwrap()
    }

    fn wrapper() -> Box<dyn MockContract<Empty, Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                simple_ica_controller::contract::execute,
                simple_ica_controller::contract::instantiate,
                simple_ica_controller::contract::query,
            )
            .with_ibc(
                simple_ica_controller::ibc::ibc_channel_open,
                simple_ica_controller::ibc::ibc_channel_connect,
                simple_ica_controller::ibc::ibc_channel_close,
                simple_ica_controller::ibc::ibc_packet_receive,
                simple_ica_controller::ibc::ibc_packet_ack,
                simple_ica_controller::ibc::ibc_packet_timeout,
            ),
        )
    }
}

pub fn host_execute(_: DepsMut, _: Env, _: MessageInfo, _: Empty) -> StdResult<Response> {
    Err(StdError::generic_err("Execute not implemented for host"))
}

#[interface(host_msgs::InstantiateMsg, Empty, host_msgs::QueryMsg, Empty)]
struct Host;
impl<Chain: CwEnv> Uploadable for Host<Chain> {
    fn wasm(_chain: &ChainInfoOwned) -> <Daemon as TxHandler>::ContractSource {
        WasmPath::new(format!("{CRATE_PATH}/wasms/simple_ica_host.wasm")).unwrap()
    }

    fn wrapper() -> Box<dyn MockContract<Empty, Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                host_execute,
                simple_ica_host::contract::instantiate,
                simple_ica_host::contract::query,
            )
            .with_reply(simple_ica_host::contract::reply)
            .with_ibc(
                simple_ica_host::contract::ibc_channel_open,
                simple_ica_host::contract::ibc_channel_connect,
                simple_ica_host::contract::ibc_channel_close,
                simple_ica_host::contract::ibc_packet_receive,
                simple_ica_host::contract::ibc_packet_ack,
                simple_ica_host::contract::ibc_packet_timeout,
            ),
        )
    }
}

// just for uploading
#[interface(Empty, Empty, Empty, Empty)]
struct Cw1;

impl<Chain: CwEnv> Uploadable for Cw1<Chain> {
    fn wasm(_chain: &ChainInfoOwned) -> <Daemon as TxHandler>::ContractSource {
        WasmPath::new(format!("{CRATE_PATH}/wasms/cw1_whitelist.wasm")).unwrap()
    }
    fn wrapper() -> Box<dyn MockContract<Empty, Empty>> {
        Box::new(ContractWrapper::new_with_empty(
            cw1_whitelist::contract::execute,
            cw1_whitelist::contract::instantiate,
            cw1_whitelist::contract::query,
        ))
    }
}
