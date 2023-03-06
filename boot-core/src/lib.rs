mod contract;
#[cfg(feature = "daemon")]
mod daemon;
#[cfg(feature = "daemon")]
mod keys;
// TODO: make private
pub mod deploy;
mod error;
mod index_response;
#[cfg(feature = "interchain")]
pub mod interchain;
///
pub mod interface;
mod mock;
pub mod prelude;
pub mod state;
mod tx_handler;

// pub mod traits;
pub use boot_contract_derive::boot_contract;
pub use boot_fns_derive::{ExecuteFns, QueryFns};
pub use contract::Contract;

#[cfg(feature = "daemon")]
pub use daemon::{
    core::Daemon,
    error::DaemonError,
    networks,
    state::{DaemonOptions, DaemonOptionsBuilder},
};

pub use deploy::Deploy;
pub use error::BootError;
pub use index_response::IndexResponse;
pub use mock::{core::Mock, state::MockState};
pub use tx_handler::{TxHandler, TxResponse};
/// Signals a supported execution environment
pub trait BootEnvironment: TxHandler + Clone {}

impl<T: TxHandler + Clone> BootEnvironment for T {}

#[cfg(feature = "daemon")]
pub(crate) mod cosmos_modules {
    pub use cosmrs::proto::cosmos::auth::v1beta1 as auth;
    pub use cosmrs::proto::cosmos::authz::v1beta1 as authz;
    pub use cosmrs::proto::cosmos::bank::v1beta1 as bank;
    pub use cosmrs::proto::cosmos::base::abci::v1beta1 as abci;
    pub use cosmrs::proto::cosmos::base::tendermint::v1beta1 as tendermint;
    pub use cosmrs::proto::cosmos::base::v1beta1 as base;
    pub use cosmrs::proto::cosmos::crisis::v1beta1 as crisis;
    pub use cosmrs::proto::cosmos::distribution::v1beta1 as distribution;
    pub use cosmrs::proto::cosmos::evidence::v1beta1 as evidence;
    pub use cosmrs::proto::cosmos::feegrant::v1beta1 as feegrant;
    pub use cosmrs::proto::cosmos::gov::v1beta1 as gov;
    pub use cosmrs::proto::cosmos::mint::v1beta1 as mint;
    pub use cosmrs::proto::cosmos::params::v1beta1 as params;
    pub use cosmrs::proto::cosmos::slashing::v1beta1 as slashing;
    pub use cosmrs::proto::cosmos::staking::v1beta1 as staking;
    pub use cosmrs::proto::cosmos::tx::v1beta1 as tx;
    pub use cosmrs::proto::cosmos::vesting::v1beta1 as vesting;
    pub use cosmrs::proto::cosmwasm::wasm::v1 as cosmwasm;
    pub use cosmrs::proto::ibc::core::channel::v1 as ibc_channel;
    pub use cosmrs::proto::tendermint::abci as tendermint_abci;
}
