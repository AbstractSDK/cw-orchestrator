//! `Daemon` and `DaemonAsync` execution environments.
//!
//! The `Daemon` type is a synchronous wrapper around the `DaemonAsync` type and can be used as a contract execution environment.
//!
#[cfg(all(feature = "rpc", feature = "grpc"))]
compile_error!("feature \"rpc\" and feature \"grpc\" cannot be enabled at the same time");

#[cfg(not(any(feature = "rpc", feature = "grpc")))]
compile_error!("At least one of feature \"rpc\" and feature \"grpc\" needs to be enabled");

pub mod builder;
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

pub mod queriers;

#[cfg(feature = "rpc")]
pub mod rpc_channel;
#[cfg(feature = "rpc")]
pub use self::rpc_channel::*;

#[cfg(feature = "grpc")]
pub mod grpc_channel;
#[cfg(feature = "grpc")]
pub use self::grpc_channel::*;

mod traits;
pub mod tx_builder;

pub use self::{builder::*, core::*, error::*, state::*, sync::*, traits::*, tx_resp::*};
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

/// Re-export trait and data required to fetch daemon data from chain-registry
pub use ibc_chain_registry::{chain::ChainData as ChainRegistryData, fetchable::Fetchable};
