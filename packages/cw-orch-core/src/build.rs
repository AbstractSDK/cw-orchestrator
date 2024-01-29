//! # Build Postfix Format
//! Used to specify the build-postfix for contracts in the `Uploadable` trait.

use crate::environment::{EnvironmentInfo, EnvironmentQuerier};

/// Build name used for building the contract.
/// See the [Abstract Optimizer](https://github.com/AbstractSDK/rust-optimizer).
pub enum BuildPostfix<'a, T: EnvironmentQuerier = ()> {
    /// Default, doesn't look for anything but the contract name.
    None,
    /// Uses the chain to figure out the chain name. I.e. "uni-6" = "juno-1" -> "juno" post-fix on build.
    ChainName(&'a T),
    /// Uses the chain name as the build-postfix. I.e. "uni-6", "juno-1", "osmosis-5", ect.
    ChainID(&'a T),
    /// Use a custom post-fix to specify the artifact.
    Custom(String),
}

impl<T: EnvironmentQuerier> From<BuildPostfix<'_, T>> for String {
    fn from(value: BuildPostfix<T>) -> Self {
        match value {
            BuildPostfix::None => "".to_string(),
            BuildPostfix::ChainName(chain) => chain.env_info().chain_name,
            BuildPostfix::ChainID(chain) => chain.env_info().chain_id,
            BuildPostfix::Custom(postfix) => postfix,
        }
    }
}

impl EnvironmentQuerier for () {
    fn env_info(&self) -> EnvironmentInfo {
        panic!()
    }
}
