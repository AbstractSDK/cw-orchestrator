use crate::networks::{ChainInfo, ChainKind, NetworkInfo};

// ANCHOR: injective
pub const INJECTIVE_NETWORK: NetworkInfo = NetworkInfo {
    chain_name: "injective",
    pub_address_prefix: "inj",
    coin_type: 60u32,
};

/// <https://docs.injective.network/develop/public-endpoints/#mainnet>
/// <https://www.mintscan.io/injective/parameters>
/// <https://status.injective.network/>
pub const INJECTIVE_1: ChainInfo = ChainInfo {
    kind: ChainKind::Mainnet,
    chain_id: "injective-1",
    gas_denom: "inj",
    gas_price: 500_000_000.0,
    grpc_urls: &["https://sentry.chain.grpc.injective.network:443"],
    network_info: INJECTIVE_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

/// <https://docs.injective.network/develop/public-endpoints/#testnet>
/// <https://testnet.status.injective.network/>
pub const INJECTIVE_888: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "injective-888",
    gas_denom: "inj",
    gas_price: 500_000_000.0,
    grpc_urls: &["https://testnet.sentry.chain.grpc.injective.network:443"],
    network_info: INJECTIVE_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
// ANCHOR_END: injective
