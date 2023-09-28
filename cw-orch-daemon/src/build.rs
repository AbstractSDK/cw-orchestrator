//! # Build Postfix Format
//! Used to specify the build-postfix for contracts in the `Uploadable` trait.

use crate::Daemon;

/// Build name used for building the contract.
/// See the [Abstract Optimizer](https://github.com/AbstractSDK/rust-optimizer).
pub enum BuildPostfix {
    /// Default, doesn't look for anything but the contract name.
    None,
    /// Uses the chain to figure out the network name. I.e. "uni-6" = "juno-1" -> "juno" post-fix on build.
    Network(Daemon),
    /// Uses the chain name as the build-postfix. I.e. "uni-6", "juno-1", "osmosis-5", ect.
    Chain(Daemon),
    /// Use a custom post-fix to specify the artifact.
    Custom(String),
}

impl From<BuildPostfix> for String {
    fn from(value: BuildPostfix) -> Self {
        match value {
            BuildPostfix::None => "".to_string(),
            BuildPostfix::Network(chain) => chain.network_id(),
            BuildPostfix::Chain(chain) => chain.chain_id(),
            BuildPostfix::Custom(postfix) => postfix,
        }
    }
}
