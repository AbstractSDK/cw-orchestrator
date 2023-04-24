//! # Boot
//!
//! Get up to speed with the basics of advanced contract development for multiple CosmWasm chains.
//!
//! Boot offers a suite of tools that simplify the creation, deployment, and testing process.
//! Enabling developers to focus on building robust and efficient smart contracts.
//!
//! You'll spend less time on boilerplate code and setup, and more time on crafting secure and performant smart contracts.
//!
//! Start accelerating your smart contract development today!
//!
//! # Important parts
//!
//! * [Contract] - Here you can find how to setup your contract and starting developing it
//! * [Mock] - And here you can find how to implement testing for your contract
//!
//! # Quick start
//!
//!
//! ## Environment
//!
//! * `STATE_FILE` - holds the path for your state file, which contains, addresses and code ids for your contracts in different environments.
//! * `LOCAL_MNEMONIC` - mnemonic to use for local development, used for genesis wallets.
//!
//! ## Contract instance initialization
//!
//! In this example we are using cw20_base contract that is contained inside [boot_cw_plus].
//!
//! ```rust
//! use std::sync::Arc;
//! use tokio::runtime::Runtime;
//! use boot_core::{
//!    instantiate_daemon_env, networks::LOCAL_JUNO,
//!    Contract, ContractWrapper, Daemon, DaemonOptionsBuilder
//! };
//!
//! let runtime = Arc::new(Runtime::new().unwrap());
//!
//! let options = DaemonOptionsBuilder::default()
//!     .network(LOCAL_JUNO)
//!     .deployment_id("v0.1.0")
//!     .build()
//!     .unwrap();
//!
//! let (sender, chain) = instantiate_daemon_env(&runtime, options).unwrap();
//!
//! let mut contract = Contract::new("cw-plus:cw20_base", chain)
//!     .with_mock(Box::new(
//!         ContractWrapper::new_with_empty(
//!             cw20_base::contract::execute,
//!             cw20_base::contract::instantiate,
//!             cw20_base::contract::query,
//!         )
//!         .with_migrate(cw20_base::contract::migrate),
//!     ))
//!     .with_wasm_path("cw20_base.wasm");
//! ```
//!
//! ## Uploading and initializing
//! After the contract has been configured we can start interacting with it.
//! In the same way we would in any other scenario:
//!
//! ```rust
//! // Upload the contract
//! let upload_res = contract.upload().unwrap();
//!
//! // Once the contract is uploaded we can adquire the code_id like this
//! let code_id = contract.code_id().unwrap();
//!
//! // Prepare or instantiate message
//! let init_msg = cw20_base::msg::InstantiateMsg {
//!     name: "Token".to_owned(),
//!     symbol: "TOK".to_owned(),
//!     decimals: 6u8,
//!     initial_balances: vec![cw20::Cw20Coin {
//!         address: sender.to_string(),
//!         amount: Uint128::from(10000u128),
//!     }],
//!     mint: None,
//!     marketing: None,
//! };
//!
//! // Now we can call the instantiate method with our data
//! let init_res = contract.instantiate(&init_msg, Some(&sender.clone()), None);
//!
//! // After this point we can get our contract address using:
//! let contract_address = contract.address().unwrap();
//!
//! ```
//!
//! ## Executing
//!
//! ```rust
//! // prepare our message
//! let exec_msg = cw20_base::msg::ExecuteMsg::Burn {
//!     amount: 10u128.into()
//! };
//!
//! // execute our contract
//! let exec_res = contract.execute(&exec_msg, None);
//! ```
//!
//! ## Quering
//!
//! ```rust
//! // prepare our query
//! let query_msg = &cw20_base::msg::QueryMsg::Balance {
//!     address: sender.to_string(),
//! };
//!
//! // query our contract
//! let query_res = contract.query::<cw20_base::msg::QueryMsg, cw20::BalanceResponse>(query_msg);
//! ```
//!
//! ## Migration
//!
//! ```rust
//! // prepare our migration message
//! let migrate_msg = &MigrateMsg {};
//!
//! // execute our migration
//! let migrate_res = contract.migrate(migrate_msg, new_code_id);
//! ```

mod contract;

#[cfg(feature = "daemon")]
mod daemon;

mod deploy;
mod error;
mod index_response;
mod interface;
#[cfg(feature = "daemon")]
mod keys;
mod mock;
mod state;
mod tx_handler;

pub use boot_contract_derive::contract;
pub use boot_fns_derive::{ExecuteFns, QueryFns};
pub use contract::{Contract, ContractCodeReference};
pub use deploy::Deploy;
pub use error::BootError;
pub use index_response::IndexResponse;
pub use interface::{
    BootExecute, BootInstantiate, BootMigrate, BootQuery, BootUpload, CallAs, ContractInstance,
    CwInterface,
};
pub use mock::{
    core::{instantiate_custom_mock_env, instantiate_default_mock_env, Mock},
    state::MockState,
};
pub use state::{ChainState, StateInterface};
pub use tx_handler::{TxHandler, TxResponse};
// re-export as it is used in the public API
pub use cosmwasm_std::{Addr, Coin, Empty};
pub use cw_multi_test::ContractWrapper;

#[cfg(feature = "daemon")]
pub use daemon::{
    channel::DaemonChannel,
    core::{instantiate_daemon_env, Daemon},
    error::DaemonError,
    networks, queriers,
    state::{DaemonOptions, DaemonOptionsBuilder},
    Wallet,
};

#[cfg(feature = "daemon")]
pub use ibc_chain_registry::{chain::ChainData as RegistryChainData, fetchable::Fetchable};

#[deprecated(
    since = "0.8.1",
    note = "Phasing out the use of `BootEnvironment` in favor of `CwEnv`"
)]
/// Signals a supported execution environment
pub trait BootEnvironment: TxHandler + Clone {}
#[allow(deprecated)]
impl<T: TxHandler + Clone> BootEnvironment for T {}

/// Signals a supported execution environment for CosmWasm contracts
pub trait CwEnv: TxHandler + Clone {}
impl<T: TxHandler + Clone> CwEnv for T {}
