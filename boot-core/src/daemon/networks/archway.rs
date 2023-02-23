use crate::networks::{ChainInfo, NetworkInfo, NetworkKind};

pub const ARCHWAY_CHAIN: ChainInfo = ChainInfo {
    chain_id: "archway",
    pub_address_prefix: "archway",
    coin_type: 118u32,
};

/// https://docs.archway.io/docs/overview/network
pub const CONSTANTINE_1: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Testnet,
    id: "constantine-1",
    gas_denom: "uconst",
    gas_price: 0.025,
    grpc_urls: &["https://grpc.constantine-1.archway.tech:443"],
    chain_info: ARCHWAY_CHAIN,
    lcd_url: Some("https://api.constantine-1.archway.tech"),
    fcd_url: None,
};
