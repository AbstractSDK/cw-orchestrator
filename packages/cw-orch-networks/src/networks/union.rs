use cw_orch_core::environment::{ChainInfo, ChainKind, NetworkInfo};

// ANCHOR: union
pub const UNION_NETWORK: NetworkInfo = NetworkInfo {
    chain_name: "union",
    pub_address_prefix: "union",
    coin_type: 118,
};

pub const UNION_TESTNET: ChainInfo = UNION_TESTNET_8;

pub const UNION_TESTNET_8: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "union-testnet-8",
    gas_denom: "muno",
    gas_price: 0.000025,
    grpc_urls: &["https://grpc.testnet-8.union.build:443"],
    network_info: UNION_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

// ANCHOR_END: union
