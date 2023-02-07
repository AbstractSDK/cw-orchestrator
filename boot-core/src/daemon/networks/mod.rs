mod juno;
mod osmosis;
pub mod terra;
pub mod injective;
pub mod kujira;
pub use crate::daemon::state::{ChainInfo, NetworkInfo, NetworkKind};
pub use juno::{JUNO_1, LOCAL_JUNO, UNI_5};
pub use osmosis::{LOCAL_OSMO, OSMO_4};
pub use terra::{LOCAL_TERRA, PISCO_1, PHOENIX_1};
pub use injective::INJECTIVE_888;
pub use kujira::HARPOON_4;

// https://polkachu.com/testnet_public_grpc
// https://polkachu.com/public_grpc

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

impl TryFrom<&str> for NetworkInfo {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let networks = vec![
            UNI_5,
            JUNO_1,
            LOCAL_JUNO,
            PISCO_1,
            PHOENIX_1,
            LOCAL_TERRA,
            INJECTIVE_888,
            HARPOON_4,
            OSMO_4,
            LOCAL_OSMO
        ];
        for net in networks {
            if net.id == value {
                return Ok(net);
            }
        }
        Err(format!("Network not found: {}", value))
    }
}
