mod contract;
#[cfg(feature = "daemon")]
mod daemon;
mod deploy;
mod error;
mod index_response;
mod interface_traits;
#[cfg(feature = "daemon")]
mod keys;
mod mock;
mod state;
mod tx_handler;

pub use contract::Contract;
pub use cw_orc_contract_derive::{contract, interface};
pub use cw_orc_fns_derive::{ExecuteFns, QueryFns};
pub use deploy::Deploy;
pub use error::CwOrcError;
pub use index_response::IndexResponse;
pub use interface_traits::{
    CallAs, ContractInstance, CwOrcExecute, CwOrcInstantiate, CwOrcMigrate, CwOrcQuery,
    CwOrcUpload, ExecuteableContract, InstantiateableContract, MigrateableContract,
    QueryableContract, Uploadable,
};

#[allow(deprecated)]
pub use mock::{
    core::{instantiate_custom_mock_env, instantiate_default_mock_env, Mock},
    state::MockState,
};
pub use state::{ChainState, StateInterface};
pub use tx_handler::{TxHandler, TxResponse};
// re-export as it is used in the public API
pub use cosmwasm_std::{Addr, Coin, Empty};
pub use cw_multi_test::{custom_app, BasicApp, ContractWrapper};

#[cfg(feature = "daemon")]
pub use daemon::{
    artifacts_dir::ArtifactsDir,
    builder::DaemonBuilder,
    channel::DaemonChannel,
    core::Daemon,
    error::DaemonError,
    networks, queriers,
    traits::{MigrateHelpers, UploadHelpers},
    wasm_path::WasmPath,
    Wallet,
};

#[cfg(feature = "daemon")]
pub use ibc_chain_registry::{chain::ChainData as RegistryChainData, fetchable::Fetchable};

/// Signals a supported execution environment for CosmWasm contracts
pub trait CwEnv: TxHandler + Clone {}
impl<T: TxHandler + Clone> CwEnv for T {}
