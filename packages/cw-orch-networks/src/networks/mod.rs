#![allow(missing_docs)]
//! # Cosmos blockchain networks
//! Contains information and helpers for different blockchain networks
//! See [parse_network] to easily retrieve this static network information
pub mod archway;
pub mod injective;
pub mod juno;
pub mod kujira;
pub mod migaloo;
pub mod neutron;
pub mod osmosis;
pub mod sei;
pub mod terra;

pub use crate::chain_info::{ChainInfo, ChainKind, NetworkInfo};
pub use archway::{ARCHWAY_1, CONSTANTINE_3};
pub use injective::{INJECTIVE_1, INJECTIVE_888};
pub use juno::{JUNO_1, LOCAL_JUNO, UNI_6};
pub use kujira::HARPOON_4;
pub use migaloo::{LOCAL_MIGALOO, MIGALOO_1, NARWHAL_1};
pub use neutron::{LOCAL_NEUTRON, NEUTRON_1, PION_1};
pub use osmosis::{LOCAL_OSMO, OSMO_5};
pub use sei::{ATLANTIC_2, LOCAL_SEI, PACIFIC_1, SEI_DEVNET_3};
pub use terra::{LOCAL_TERRA, PHOENIX_1, PISCO_1};

/// A helper function to retrieve a [`ChainInfo`] struct for a given chain-id.
///
/// ## Example
/// ```rust,no_run
/// use cw_orch_networks::networks::{parse_network, ChainInfo};
/// let juno_mainnet: ChainInfo = parse_network("juno-1");
/// ```
/// ---
/// supported chains are: UNI_6, JUNO_1, LOCAL_JUNO, PISCO_1, PHOENIX_1, LOCAL_TERRA, INJECTIVE_888, CONSTANTINE_1, BARYON_1, INJECTIVE_1, HARPOON_4, OSMO_4, LOCAL_OSMO
pub fn parse_network(net_id: &str) -> ChainInfo {
    let networks = vec![
        UNI_6,
        JUNO_1,
        LOCAL_JUNO,
        PISCO_1,
        PHOENIX_1,
        LOCAL_TERRA,
        INJECTIVE_888,
        CONSTANTINE_3,
        ARCHWAY_1,
        PION_1,
        NARWHAL_1,
        NEUTRON_1,
        INJECTIVE_1,
        HARPOON_4,
        OSMO_5,
        LOCAL_OSMO,
        LOCAL_MIGALOO,
        LOCAL_NEUTRON,
        MIGALOO_1,
        LOCAL_SEI,
        SEI_DEVNET_3,
        ATLANTIC_2,
        PACIFIC_1,
    ];
    for net in networks {
        if net.chain_id == net_id {
            return net;
        }
    }
    panic!("Network not found: {}", net_id);
}
