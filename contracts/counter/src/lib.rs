pub mod contract;
mod error;
pub(crate) mod execute;
pub mod msg;
pub(crate) mod query;
pub mod state;

pub use crate::error::ContractError;
// ANCHOR: interface_reexport
#[cfg(feature = "interface")]
pub use crate::contract::CounterContract;
// ANCHOR_END: interface_reexport
// ANCHOR: fn_re_export
#[cfg(feature = "interface")]
pub use crate::msg::{ExecuteMsgFns as CounterExecuteMsgFns, QueryMsgFns as CounterQueryMsgFns};
// ANCHOR_END: fn_re_export

// ANCHOR: custom_interface
#[cfg(feature = "interface")]
mod interface {
    use cosmwasm_std::Empty;

    use crate::CounterContract;

    impl cw_orch_cli::CwCliAddons<cosmwasm_std::Empty> for CounterContract<cw_orch::prelude::Daemon> {
        fn addons(&mut self, _context: Empty) -> cw_orch::anyhow::Result<()>
        where
            Self: cw_orch::prelude::ContractInstance<cw_orch::prelude::Daemon>,
        {
            Ok(())
        }
    }
}
// ANCHOR_END: custom_interface
