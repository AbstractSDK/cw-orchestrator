//! # Build Postfix Format
//! Used to specify the build-postfix for contracts in the `Uploadable` trait.

use crate::environment::{ChainInfoOwned, EnvironmentInfo, EnvironmentQuerier};

/// Build name used for building the contract.
/// See the [Abstract Optimizer](https://github.com/AbstractSDK/rust-optimizer).
pub enum BuildPostfix<'a> {
    /// Default, doesn't look for anything but the contract name.
    None,
    /// Uses the chain to figure out the chain name. I.e. "uni-6" = "juno-1" -> "juno" post-fix on build.
    ChainName(&'a ChainInfoOwned),
    /// Uses the chain name as the build-postfix. I.e. "uni-6", "juno-1", "osmosis-5", ect.
    ChainID(&'a ChainInfoOwned),
    /// Use a custom post-fix to specify the artifact.
    Custom(String),
}

impl From<BuildPostfix<'_>> for String {
    fn from(value: BuildPostfix) -> Self {
        match value {
            BuildPostfix::None => "".to_string(),
            BuildPostfix::ChainName(chain) => chain.network_info.chain_name.clone(),
            BuildPostfix::ChainID(chain) => chain.chain_id.clone(),
            BuildPostfix::Custom(postfix) => postfix,
        }
    }
}

impl EnvironmentQuerier for () {
    fn env_info(&self) -> EnvironmentInfo {
        panic!()
    }
}
