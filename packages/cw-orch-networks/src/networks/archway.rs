use crate::networks::{ChainInfoConst, ChainKind, NetworkInfoConst};

// ANCHOR: archway
pub const ARCHWAY_NETWORK: NetworkInfoConst = NetworkInfoConst {
    id: "archway",
    pub_address_prefix: "archway",
    coin_type: 118u32,
};

/// Archway Docs: <https://docs.archway.io/resources/networks>
/// Parameters: <https://testnet.mintscan.io/archway-testnet/parameters>
pub const CONSTANTINE_3: ChainInfoConst = ChainInfoConst {
    kind: ChainKind::Testnet,
    chain_id: "constantine-3",
    gas_denom: "aconst",
    gas_price: 1000000000000.0,
    grpc_urls: &["https://grpc.constantine.archway.tech:443"],
    network_info: ARCHWAY_NETWORK,
    lcd_url: Some("https://api.constantine.archway.tech"),
    fcd_url: None,
};

/// Archway Docs: <https://docs.archway.io/resources/networks>
/// Parameters <https://www.mintscan.io/archway/parameters>
pub const ARCHWAY_1: ChainInfoConst = ChainInfoConst {
    kind: ChainKind::Mainnet,
    chain_id: "archway-1",
    gas_denom: "aarch",
    gas_price: 1000000000000.0,
    grpc_urls: &["https://grpc.mainnet.archway.io:443"],
    network_info: ARCHWAY_NETWORK,
    lcd_url: Some("https://api.mainnet.archway.io"),
    fcd_url: None,
};
// ANCHOR_END: archway
