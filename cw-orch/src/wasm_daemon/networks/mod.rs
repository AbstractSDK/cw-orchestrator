#![allow(missing_docs)]
//! # Cosmos blockchain networks
//! Contains information and helpers for different blockchain networks
//! See [parse_network] to easily retrieve this static network information
pub mod juno;

pub use crate::wasm_daemon::chain_info::{ChainInfo, ChainKind, NetworkInfo};
pub use juno::{JUNO_1, LOCAL_JUNO, UNI_6};

/// A helper function to retrieve a [`ChainInfo`] struct for a given chain-id.
///
/// ## Example
/// ```rust,no_run
/// use cw_orch::prelude::networks::{parse_network, ChainInfo};
/// let juno_mainnet: ChainInfo = parse_network("juno-1");
/// ```
/// ---
/// supported chains are: UNI_6, JUNO_1, LOCAL_JUNO, PISCO_1, PHOENIX_1, LOCAL_TERRA, INJECTIVE_888, CONSTANTINE_1, BARYON_1, INJECTIVE_1, HARPOON_4, OSMO_4, LOCAL_OSMO
pub fn parse_network(net_id: &str) -> ChainInfo {
    let networks = vec![
        UNI_6,
        JUNO_1,
        LOCAL_JUNO,
    ];
    for net in networks {
        if net.chain_id == net_id {
            return net;
        }
    }
    panic!("Network not found: {}", net_id);
}
