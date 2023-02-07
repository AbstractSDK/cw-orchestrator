use crate::networks::{ChainInfo, NetworkInfo, NetworkKind};

pub const INJECTIVE_CHAIN: ChainInfo = ChainInfo {
    chain_id: "injective",
    pub_address_prefix: "inj",
    coin_type: 60u32,
};

// https://testnet.status.injective.network/
pub const INJECTIVE_888: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Testnet,
    id: "injective-888",
    gas_denom: "inj",
    gas_price: 0.025,
    grpc_urls: &["https://testnet.grpc.injective.network:443"],
    chain_info: INJECTIVE_CHAIN,
    lcd_url: None,
    fcd_url: None,
};
