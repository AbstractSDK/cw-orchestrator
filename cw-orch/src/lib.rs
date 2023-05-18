#![doc(html_logo_url = "https://raw.githubusercontent.com/AbstractSDK/assets/mainline/logo.svg")]
// macros
pub use cw_orch_contract_derive::{interface, interface_entry_point};
pub use cw_orch_fns_derive::{ExecuteFns, QueryFns};

// Re-export anyhow for the macro
pub extern crate anyhow;

// prelude
pub mod prelude;

pub mod contract;
#[cfg(feature = "daemon")]
pub mod daemon;

pub mod deploy;
pub mod environment;
mod error;
mod index_response;
#[cfg(feature = "interchain")]
mod interchain;
mod interface_traits;
#[cfg(feature = "daemon")]
mod keys;
mod mock;
mod paths;
mod state;

pub use contract::Contract;
pub use deploy::Deploy;
pub use error::CwOrchError;
pub use index_response::IndexResponse;
pub use interface_traits::{
    CallAs, ContractInstance, CwOrcExecute, CwOrcInstantiate, CwOrcMigrate, CwOrcQuery,
    CwOrcUpload, ExecutableContract, InstantiableContract, MigratableContract, QueryableContract,
    Uploadable,
};

#[allow(deprecated)]
pub use mock::{core::Mock, state::MockState};
pub use state::{ChainState, StateInterface};
// re-export as it is used in the public API
pub use cosmwasm_std::{Addr, Coin, Empty};
pub use cw_multi_test::{custom_app, BasicApp, ContractWrapper};

#[cfg(feature = "daemon")]
pub use daemon::{
    builder::DaemonBuilder,
    error::DaemonError,
    ibc_tracker, networks, queriers,
    sync::core::Daemon,
    traits::{MigrateHelpers, UploadHelpers},
    tx_resp::CosmTxResponse,
    Wallet,
};

pub use environment::{ChainUpload, CwEnv, TxHandler, TxResponse};

#[cfg(feature = "daemon")]
pub mod channel {
    pub use crate::daemon::channel::ChannelAccess;
}

#[cfg(feature = "interchain")]
pub use interchain::{
    follow_ibc_execution, hermes::Hermes, infrastructure::InterchainInfrastructure,
};

#[cfg(feature = "daemon")]
pub use ibc_chain_registry::{chain::ChainData as RegistryChainData, fetchable::Fetchable};
