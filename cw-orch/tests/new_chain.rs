// ANCHOR: NEW_NETWORK_INFO

use cw_orch::environment::{ChainInfo, ChainKind, NetworkInfo};

pub const NEW_NETWORK_INFO: NetworkInfo = NetworkInfo {
    chain_name: "osmosis",
    pub_address_prefix: "osmo",
    coin_type: 118,
};

pub const NEW_CHAIN_INFO: ChainInfo = ChainInfo {
    chain_id: "osmosis-4",
    gas_denom: "uosmo",
    gas_price: 7575.8,
    grpc_urls: &["Some GRPC URLS"],
    lcd_url: None, // Not necessary for cw-orch
    fcd_url: None, // Not necessary for cw-orch
    network_info: NEW_NETWORK_INFO,
    kind: ChainKind::Mainnet,
};
// ANCHOR_END: NEW_NETWORK_INFO
