pub mod networks;

pub use cw_orch_core::environment::{ChainInfo, ChainKind, NetworkInfo};

use networks::SUPPORTED_NETWORKS;

/// A helper function to retrieve a [`ChainInfo`] struct for a given chain-id.
///
/// ## Example
/// ```rust,no_run
/// use cw_orch_networks::networks::{parse_network, ChainInfo};
///
/// let juno_mainnet: ChainInfo = parse_network("juno-1").unwrap();
/// ```
/// ---
/// supported chains are defined by the `SUPPORT_NETWORKS` variable
pub fn parse_network(net_id: &str) -> Result<ChainInfo, String> {
    SUPPORTED_NETWORKS
        .iter()
        .find(|net| net.chain_id == net_id)
        .cloned()
        .ok_or(format!("Network not found: {}", net_id))
}
