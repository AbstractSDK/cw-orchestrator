mod contract_instance;
mod deploy;
pub mod interface_traits;
mod paths;
/// Cloned from cosmwasm/cw_multi_test
mod wrapper;
#[cfg(feature = "mock")]
pub use wrapper::cw_multi_test_impl::BoxedContractWrapper;
pub use wrapper::{Contract as MockContract, ContractWrapper};

pub use contract_instance::Contract;
pub use deploy::Deploy;

pub use paths::from_workspace as artifacts_dir_from_workspace;
pub use paths::{ArtifactsDir, WasmPath};
