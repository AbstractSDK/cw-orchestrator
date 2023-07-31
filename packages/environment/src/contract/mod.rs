mod contract_instance;
mod deploy;
pub mod interface_traits;
mod paths;

pub use contract_instance::Contract;
pub use deploy::Deploy;

pub use paths::from_workspace as artifacts_dir_from_workspace;
pub use paths::{ArtifactsDir, WasmPath};
