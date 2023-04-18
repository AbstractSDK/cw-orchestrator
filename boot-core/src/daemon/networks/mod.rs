pub mod archway;
pub mod injective;
pub mod juno;
pub mod kujira;
pub mod neutron;
pub mod osmosis;
pub mod terra;

pub use crate::daemon::state::{NetworkInfo, ChainInfo, ChainKind};
pub use archway::CONSTANTINE_1;
pub use injective::{INJECTIVE_1, INJECTIVE_888};
pub use juno::{JUNO_1, LOCAL_JUNO, UNI_6};
pub use kujira::HARPOON_4;
pub use neutron::BARYON_1;
pub use osmosis::{LOCAL_OSMO, OSMO_4};
pub use terra::{LOCAL_TERRA, PHOENIX_1, PISCO_1};

/// Returns a [`NetworkInfo`] given its id
pub fn parse_network(net_id: &str) -> ChainInfo {
    let networks = vec![
        UNI_6,
        JUNO_1,
        LOCAL_JUNO,
        PISCO_1,
        PHOENIX_1,
        LOCAL_TERRA,
        INJECTIVE_888,
        CONSTANTINE_1,
        BARYON_1,
        INJECTIVE_1,
        HARPOON_4,
        OSMO_4,
        LOCAL_OSMO,
    ];
    for net in networks {
        if net.chain_id == net_id {
            return net;
        }
    }
    panic!("Network not found: {}", net_id);
}
