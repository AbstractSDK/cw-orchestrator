use crate::networks::{ChainInfo, ChainKind, NetworkInfo};

// ANCHOR: archway
pub const ARCHWAY_NETWORK: NetworkInfo = NetworkInfo {
    id: "archway",
    pub_address_prefix: "archway",
    coin_type: 118u32,
};

/// Archway Docs: <https://docs.archway.io/resources/networks>
/// Parameters: <https://testnet.mintscan.io/archway-testnet/parameters>
pub const CONSTANTINE_3: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "constantine-3",
    gas_denom: "aconst",
    gas_price: 1000000000000.0,
    grpc_urls: &["https://grpc.constantine.archway.tech:443"],
    rpc_urls: &[],
    network_info: ARCHWAY_NETWORK,
    lcd_url: Some("https://api.constantine.archway.tech"),
    fcd_url: None,
};

/// Archway Docs: <https://docs.archway.io/resources/networks>
/// Parameters <https://www.mintscan.io/archway/parameters>
pub const ARCHWAY_1: ChainInfo = ChainInfo {
    kind: ChainKind::Mainnet,
    chain_id: "archway-1",
    gas_denom: "aarch",
    gas_price: 1000000000000.0,
    grpc_urls: &["https://grpc.mainnet.archway.io:443"],
    rpc_urls: &["https://rpc.mainnet.archway.io"],
    network_info: ARCHWAY_NETWORK,
    lcd_url: Some("https://api.mainnet.archway.io"),
    fcd_url: None,
};
// ANCHOR_END: archway

#[deprecated(
    since = "0.6.1",
    note = "Constantine-1 does not exist anymore. Use Constantine-3 instead."
)]
pub const CONSTANTINE_1: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "constantine-1",
    gas_denom: "uconst",
    gas_price: 0.025,
    grpc_urls: &["https://grpc.constantine-1.archway.tech:443"],
    rpc_urls: &[],
    network_info: ARCHWAY_NETWORK,
    lcd_url: Some("https://api.constantine-1.archway.tech"),
    fcd_url: None,
};
