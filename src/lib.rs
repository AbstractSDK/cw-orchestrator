#![allow(dead_code)]
pub mod contract;
pub mod error;
pub mod helpers;
pub mod multisig;
pub mod sender;
pub mod traits;

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
