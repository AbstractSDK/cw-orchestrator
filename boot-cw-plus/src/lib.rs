// TODO: Update this code
//! Easy to use CosmWasm-plus scripting with BOOT
//!
//! This crate provides a set of contracts that can be used to deploy and interact with the
//! [CosmWasm-plus]() contracts.
//!
//! //!
//! ### Usage
//!
//! ```ignore
//! use boot_cw_plus::CwPlus;
//! fn setup() {
//!     let sender = Addr::unchecked("test_sender");
//!     let cw_plus = CwPlus::new();
//!     let (_, chain) = instantiate_default_mock_env(&sender)?;
//!     cw_plus.deploy(chain.clone(), Empty {});
//!     // The same in a cw-multi-test context
//!     let mut token = Cw20Base::new("cw-plus:cw20_base", chain);
//!     token.upload()?;
//! }
//! ```

pub const CW1_SUBKEYS: &str = "cw-plus:cw1_subkeys";
pub const CW1_WHITELIST: &str = "cw-plus:cw1_whitelist";
pub const CW20_BASE: &str = "cw-plus:cw20_base";
pub const CW20_ICS20: &str = "cw-plus:cw20_ics20";
pub const CW3_FIXED_MULTISIG: &str = "cw-plus:cw3_fixed_multisig";
pub const CW3_FLEX_MULTISIG: &str = "cw-plus:cw3_flex_multisig";
pub const CW4_GROUP: &str = "cw-plus:cw4_group";
pub const CW4_STAKE: &str = "cw-plus:cw4_stake";

mod contracts;
mod cw_plus;
pub use crate::cw_plus::CwPlus;
pub use contracts::*;
