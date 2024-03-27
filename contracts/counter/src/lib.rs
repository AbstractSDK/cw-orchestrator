pub mod contract;
mod error;
pub(crate) mod execute;
pub mod msg;
pub(crate) mod query;
pub mod state;

pub use crate::error::ContractError;
// ANCHOR: fn_re_export
pub use crate::msg::{ExecuteMsgFns as CounterExecuteMsgFns, QueryMsgFns as CounterQueryMsgFns};
// ANCHOR_END: fn_re_export

// ANCHOR: custom_interface
#[cfg(not(target_arch = "wasm32"))]
mod interface;
// ANCHOR_END: custom_interface

// ANCHOR: interface_reexport
#[cfg(not(target_arch = "wasm32"))]
pub use crate::interface::CounterContract;
// ANCHOR_END: interface_reexport
