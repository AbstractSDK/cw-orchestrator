use cw_orch_core::environment::{ChainInfo, ChainKind, NetworkInfo};

// ANCHOR: osmosis
pub const OSMO_NETWORK: NetworkInfo = NetworkInfo {
    chain_name: "osmosis",
    pub_address_prefix: "osmo",
    coin_type: 118u32,
};

pub const OSMOSIS_1: ChainInfo = ChainInfo {
    kind: ChainKind::Mainnet,
    chain_id: "osmosis-1",
    gas_denom: "uosmo",
    gas_price: 0.025,
    grpc_urls: &["https://grpc.osmosis.zone:443"],
    rpc_urls: &[],
    network_info: OSMO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

pub const OSMO_5: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "osmo-test-5",
    gas_denom: "uosmo",
    gas_price: 0.025,
    grpc_urls: &["https://grpc.osmotest5.osmosis.zone:443"],
    rpc_urls: &[],
    network_info: OSMO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

pub const LOCAL_OSMO: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "localosmosis",
    gas_denom: "uosmo",
    gas_price: 0.0026,
    grpc_urls: &["http://65.108.235.46:9094"],
    rpc_urls: &[],
    network_info: OSMO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
// ANCHOR_END: osmosis
