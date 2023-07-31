//! `Daemon` and `DaemonAsync` execution environments.
//!
//! The `Daemon` type is a synchronous wrapper around the `DaemonAsync` type and can be used as a contract execution environment.

pub mod builder;
pub mod channel;
pub mod core;
pub mod error;
pub(crate) mod json_file;
/// Proto types for different blockchains
pub mod proto;

pub mod sender;
pub mod state;
pub mod sync;
pub mod tx_resp;
// Tracking IBC state
mod ibc_tracker;
// expose these as mods as they can grow
pub mod keys;
pub mod queriers;
mod traits;
pub mod tx_builder;

pub use self::{
    builder::*, channel::*, chain_info::*, core::*, error::*, state::*, sync::*, traits::*, ibc_tracker::*, tx_resp::*,
};
pub use cw_orch_environment::chain_info::*;
pub use cw_orch_environment::networks;
pub use sender::Wallet;
pub use tx_builder::TxBuilder;

pub(crate) mod cosmos_modules {
    pub use cosmrs::proto::{
        cosmos::{
            auth::v1beta1 as auth,
            authz::v1beta1 as authz,
            bank::v1beta1 as bank,
            base::{abci::v1beta1 as abci, tendermint::v1beta1 as tendermint, v1beta1 as base},
            crisis::v1beta1 as crisis,
            distribution::v1beta1 as distribution,
            evidence::v1beta1 as evidence,
            feegrant::v1beta1 as feegrant,
            gov::v1beta1 as gov,
            mint::v1beta1 as mint,
            params::v1beta1 as params,
            slashing::v1beta1 as slashing,
            staking::v1beta1 as staking,
            tx::v1beta1 as tx,
            vesting::v1beta1 as vesting,
        },
        cosmwasm::wasm::v1 as cosmwasm,
        ibc::{
            applications::transfer::v1 as ibc_transfer,
            core::{
                channel::v1 as ibc_channel, client::v1 as ibc_client,
                connection::v1 as ibc_connection,
            },
        },
        tendermint::abci as tendermint_abci,
    };
}
