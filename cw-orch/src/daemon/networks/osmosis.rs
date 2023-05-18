use crate::daemon::chain_info::{ChainInfo, ChainKind, NetworkInfo};

pub const OSMO_NETWORK: NetworkInfo = NetworkInfo {
    id: "osmosis",
    pub_address_prefix: "osmo",
    coin_type: 118u32,
};

pub const OSMO_4: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "osmo-test-4",
    gas_denom: "uosmo",
    gas_price: 0.025,
    grpc_urls: &["http://grpc-test.osmosis.zone:443"],
    network_info: OSMO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

// pub const JUNO_1: NetworkInfo = NetworkInfo {
//     kind: ChainKind::Mainnet,
//     id: "juno-1",
//     gas_denom: "ujuno",
//     gas_price: 0.0026,
//     grpc_url: "http://65.108.235.46:26090",
//     network_info: JUNO_CHAIN,
//     lcd_url: None,
//     fcd_url: None,
// };

pub const LOCAL_OSMO: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "localosmosis",
    gas_denom: "uosmo",
    gas_price: 0.0026,
    grpc_urls: &["http://65.108.235.46:9094"],
    network_info: OSMO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
