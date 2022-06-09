#![allow(dead_code)]
pub mod contract;
mod data_structures;
pub mod error;
pub mod helpers;
mod keys;
mod multisig;
pub mod sender;
pub mod traits;

pub use cosmrs::{Coin, Denom};
pub use data_structures::{
    deployment::Deployment,
    network::{Chain, Network, NetworkKind},
    tx_resp::CosmTxResponse,
};
pub use error::CosmScriptError;
pub use helpers::{get_configuration, get_env_vars};

#[macro_use]
extern crate lazy_static;
pub(crate) use crate::client_types::cosm_denom_format;
use data_structures::{client_types, core_types};

pub mod cosmos_modules {
    pub use cosmos_sdk_proto::cosmos::auth::v1beta1 as auth;
    pub use cosmos_sdk_proto::cosmos::authz::v1beta1 as authz;
    pub use cosmos_sdk_proto::cosmos::bank::v1beta1 as bank;
    pub use cosmos_sdk_proto::cosmos::base::abci::v1beta1 as abci;
    pub use cosmos_sdk_proto::cosmos::base::tendermint::v1beta1 as tendermint;
    pub use cosmos_sdk_proto::cosmos::base::v1beta1 as base;
    pub use cosmos_sdk_proto::cosmos::crisis::v1beta1 as crisis;
    pub use cosmos_sdk_proto::cosmos::distribution::v1beta1 as distribution;
    pub use cosmos_sdk_proto::cosmos::evidence::v1beta1 as evidence;
    pub use cosmos_sdk_proto::cosmos::feegrant::v1beta1 as feegrant;
    pub use cosmos_sdk_proto::cosmos::gov::v1beta1 as gov;
    pub use cosmos_sdk_proto::cosmos::mint::v1beta1 as mint;
    pub use cosmos_sdk_proto::cosmos::params::v1beta1 as params;
    pub use cosmos_sdk_proto::cosmos::slashing::v1beta1 as slashing;
    pub use cosmos_sdk_proto::cosmos::staking::v1beta1 as staking;
    pub use cosmos_sdk_proto::cosmos::tx::v1beta1 as tx;
    pub use cosmos_sdk_proto::cosmwasm::wasm::v1 as cosmwasm;
    pub use cosmos_sdk_proto::tendermint::abci as tendermint_abci;
}

// mod macro_dev {
//     use terra_rust_script_derive::contract;

//     #[derive(Clone, Debug, contract)]
//     /// Updates the addressbook
//     pub enum ExecuteMsg {
//         UpdateContractAddresses {
//             to_add: Vec<(String, String)>,
//             to_remove: Vec<String>,
//         },
//         UpdateAssetAddresses {
//             to_add: Vec<(String, String)>,
//             to_remove: Vec<String>,
//         },
//         /// Sets a new Admin
//         SetAdmin {
//             admin: String,
//         },

//         Set {
//             init: InstantiateMsg,
//         },
//     }

//     #[derive(Clone, Debug, contract)]
//     pub struct InstantiateMsg {
//         /// Version control contract used to get code-ids and register OS
//         pub version_control_contract: String,
//         /// Memory contract
//         pub memory_contract: String,
//         // Creation fee in some denom (TBD)
//         pub creation_fee: u32,
//     }
// }
