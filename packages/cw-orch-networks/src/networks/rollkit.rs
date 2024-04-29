use cw_orch_core::environment::{ChainInfo, ChainKind, NetworkInfo};

// ANCHOR: rollkit
pub const ROLLKIT_NETWORK: NetworkInfo = NetworkInfo {
    id: "rollkit",
    pub_address_prefix: "wasm",
    coin_type: 118u32,
};

pub const LOCAL_ROLLKIT: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "celeswasm",
    gas_denom: "uwasm",
    gas_price: 0.025,
    grpc_urls: &["http://localhost:9290"],
    network_info: ROLLKIT_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

pub const ROLLKIT_TESTNET: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "rosm",
    gas_denom: "urosm",
    gas_price: 0.025,
    grpc_urls: &["http://grpc.rosm.rollkit.dev:9290"],
    network_info: ROLLKIT_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
// ANCHOR_END: rollkit
