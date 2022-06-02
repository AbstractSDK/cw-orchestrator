#![allow(dead_code)]
mod client_types;
pub mod config;
pub mod contract;
mod core_types;
pub mod error;
pub mod helpers;
mod keys;
mod multisig;
mod network;
pub mod sender;
pub mod traits;

#[macro_use]
extern crate lazy_static;

pub use crate::client_types::{
    terra_decimal_format, terra_f64_format, terra_opt_decimal_format, terra_opt_u64_format,
    terra_u64_format,
};

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
