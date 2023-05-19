pub mod contract;
mod error;
pub mod msg;
pub mod state;

mod integration_tests;

pub use crate::error::ContractError;

#[cfg(feature = "interface")]
pub use crate::contract::CounterContract;
#[cfg(feature = "interface")]
pub use crate::msg::{ExecuteMsgFns as CounterExecuteMsgFns, QueryMsgFns as CounterQueryMsgFns};
