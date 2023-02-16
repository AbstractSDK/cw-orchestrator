pub mod juno;
pub mod osmosis;
pub mod terra;
pub mod injective;
pub mod kujira;
pub mod archway;
pub mod neutron;

pub use crate::daemon::state::{ChainInfo, NetworkInfo, NetworkKind};
pub use juno::{JUNO_1, LOCAL_JUNO, UNI_5};
pub use osmosis::{LOCAL_OSMO, OSMO_4};
pub use terra::{LOCAL_TERRA, PISCO_1, PHOENIX_1};
pub use injective::{INJECTIVE_888, INJECTIVE_1};
pub use kujira::HARPOON_4;
pub use archway::CONSTANTINE_1;
pub use neutron::BARYON_1;

/// Returns a [`NetworkInfo`] given its id
pub fn parse_network(net_id: &str) -> NetworkInfo {
    let networks = vec![
        UNI_5,
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
        LOCAL_OSMO
    ];
    for net in networks {
        if net.id == net_id {
            return net;
        }
    }
    panic!("Network not found: {}", net_id);
}
