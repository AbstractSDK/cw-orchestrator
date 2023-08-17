use crate::networks::{ChainInfo, ChainKind, NetworkInfo};

pub const ARCHWAY_NETWORK: NetworkInfo = NetworkInfo {
    id: "archway",
    pub_address_prefix: "archway",
    coin_type: 118u32,
};

/// <https://docs.archway.io/docs/overview/network>
pub const CONSTANTINE_1: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "constantine-1",
    gas_denom: "uconst",
    gas_price: 0.025,
    grpc_urls: &["https://grpc.constantine-1.archway.tech:443"],
    network_info: ARCHWAY_NETWORK,
    lcd_url: Some("https://api.constantine-1.archway.tech"),
    fcd_url: None,
};
