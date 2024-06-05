// prelude
#[cfg(not(target_arch = "wasm32"))]
pub mod prelude {
    pub use cw_orch_interchain_core::{IbcQueryHandler, InterchainEnv};
    pub use cw_orch_interchain_mock::{MockBech32InterchainEnv, MockInterchainEnv};

    #[cfg(feature = "daemon")]
    pub use cw_orch_interchain_daemon::{
        ChannelCreationValidator, ChannelCreator, DaemonInterchainEnv,
    };
    #[cfg(feature = "daemon")]
    pub use cw_orch_starship::Starship;
}

#[cfg(not(target_arch = "wasm32"))]
pub use cw_orch_interchain_core::*;

#[cfg(not(target_arch = "wasm32"))]
pub use cw_orch_interchain_mock::*;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "daemon")]
pub use cw_orch_interchain_daemon::*;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "daemon")]
pub use cw_orch_starship::*;
