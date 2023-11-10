pub mod contract;
mod error;
pub(crate) mod execute;
pub mod msg;
pub(crate) mod query;
pub mod state;

pub use crate::error::ContractError;
// ANCHOR: interface_reexport
#[cfg(feature = "interface")]
pub use crate::interface::CounterContract;
// ANCHOR_END: interface_reexport
// ANCHOR: fn_re_export
#[cfg(feature = "interface")]
pub use crate::msg::{ExecuteMsgFns as CounterExecuteMsgFns, QueryMsgFns as CounterQueryMsgFns};
// ANCHOR_END: fn_re_export

// ANCHOR: custom_interface
#[cfg(feature = "interface")]
mod interface;
// ANCHOR_END: custom_interface
