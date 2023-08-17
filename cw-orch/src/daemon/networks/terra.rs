use crate::daemon::chain_info::{ChainInfo, ChainKind, NetworkInfo};

// ANCHOR: terra
pub const TERRA_NETWORK: NetworkInfo = NetworkInfo {
    id: "terra2",
    pub_address_prefix: "terra",
    coin_type: 330u32,
};

/// Terra testnet network.
/// <https://docs.terra.money/develop/endpoints>
pub const PISCO_1: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "pisco-1",
    gas_denom: "uluna",
    gas_price: 0.15,
    grpc_urls: &["http://terra-testnet-grpc.polkachu.com:11790"],
    network_info: TERRA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

/// Terra mainnet network.
///<https://docs.terra.money/develop/endpoints>
pub const PHOENIX_1: ChainInfo = ChainInfo {
    kind: ChainKind::Mainnet,
    chain_id: "phoenix-1",
    gas_denom: "uluna",
    gas_price: 0.15,
    grpc_urls: &["http://terra-grpc.polkachu.com:11790"],
    network_info: TERRA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

/// Terra local network.
/// <https://docs.terra.money/develop/guides/initial/#next-steps-localterra-or-testnet>
pub const LOCAL_TERRA: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "localterra",
    gas_denom: "uluna",
    gas_price: 0.15,
    grpc_urls: &["http://65.108.235.46:9090"],
    network_info: TERRA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
// ANCHOR_END: terra
