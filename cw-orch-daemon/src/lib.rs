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
// expose these as mods as they can grow
pub mod keys;
pub mod live_mock;
mod log;
pub mod queriers;
mod traits;
pub mod tx_broadcaster;
pub mod tx_builder;
pub use self::{builder::*, channel::*, core::*, error::*, state::*, sync::*, tx_resp::*};
pub use cw_orch_networks::chain_info::*;
pub use cw_orch_networks::networks;
pub use sender::Wallet;
pub use tx_builder::TxBuilder;

pub(crate) mod cosmos_modules {
    pub use cosmrs::proto::{
        cosmos::{
            auth::v1beta1 as auth,
            authz::v1beta1 as authz,
            bank::v1beta1 as bank,
            base::{abci::v1beta1 as abci, tendermint::v1beta1 as tendermint},
            feegrant::v1beta1 as feegrant,
            gov::v1beta1 as gov,
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
        tendermint::v0_34::abci as tendermint_abci,
    };
}

/// Re-export trait and data required to fetch daemon data from chain-registry
pub use ibc_chain_registry::{chain::ChainData as ChainRegistryData, fetchable::Fetchable};
