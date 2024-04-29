use cw_orch_core::environment::{ChainInfo, ChainKind, NetworkInfo};

// ANCHOR: xion
pub const XION_NETWORK: NetworkInfo = NetworkInfo {
    chain_name: "xion",
    pub_address_prefix: "xion",
    coin_type: 118u32,
};

pub const XION_TESTNET_1: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "xion-testnet-1",
    gas_denom: "uxion",
    gas_price: 0.025,
    grpc_urls: &["http://xion-testnet-grpc.polkachu.com:22390"],
    network_info: XION_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

// ANCHOR_END: xion
