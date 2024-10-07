use cw_orch_core::environment::{ChainInfo, ChainKind, NetworkInfo};

// ANCHOR: cosmos
pub const COSMOS_HUB_NETWORK: NetworkInfo = NetworkInfo {
    chain_name: "cosmoshub",
    pub_address_prefix: "cosmos",
    coin_type: 118,
};

pub const COSMOS_HUB_TESTNET: ChainInfo = THETA_TESTNET_001;

pub const THETA_TESTNET_001: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "theta-testnet-001",
    gas_denom: "uatom",
    gas_price: 0.0025,
    grpc_urls: &["https://grpc-t.cosmos.nodestake.top:443"],
    network_info: COSMOS_HUB_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

// ANCHOR_END: cosmos
