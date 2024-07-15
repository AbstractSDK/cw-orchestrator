use crate::networks::{ChainInfo, ChainKind, NetworkInfo};

// ANCHOR: migaloo
pub const MIGALOO_NETWORK: NetworkInfo = NetworkInfo {
    chain_name: "migaloo-1",
    pub_address_prefix: "migaloo",
    coin_type: 118u32,
};

pub const LOCAL_MIGALOO: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "migaloo-chain",
    gas_denom: "uwhale",
    gas_price: 0.1,
    grpc_urls: &["http://localhost:9090"],
    rpc_urls: &[],
    network_info: MIGALOO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

/// <https://docs.migaloo.zone/validators/testnet>
pub const NARWHAL_1: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "narwhal-1",
    gas_denom: "uwhale",
    gas_price: 0.1,
    grpc_urls: &["migaloo-testnet-grpc.polkachu.com:20790"],
    rpc_urls: &[],
    network_info: MIGALOO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

/// <https://docs.migaloo.zone/validators/mainnet>
pub const MIGALOO_1: ChainInfo = ChainInfo {
    kind: ChainKind::Mainnet,
    chain_id: "migaloo-1",
    gas_denom: "uwhale",
    gas_price: 0.1,
    grpc_urls: &["migaloo-grpc.polkachu.com:20790"],
    rpc_urls: &[],
    network_info: MIGALOO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
// ANCHOR_END: migaloo
