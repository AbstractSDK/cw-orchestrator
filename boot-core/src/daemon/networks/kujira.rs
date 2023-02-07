use crate::networks::{ChainInfo, NetworkInfo, NetworkKind};

pub const KUJIRA_CHAIN: ChainInfo = ChainInfo {
    chain_id: "kujira",
    pub_address_prefix: "kujira",
    coin_type: 118u32,
};

pub const HARPOON_4: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Testnet,
    id: "harpoon-4",
    gas_denom: "ukuji",
    gas_price: 0.025,
    grpc_urls: &["https://kujira-testnet-grpc.polkachu.com:11890"],
    chain_info: KUJIRA_CHAIN,
    lcd_url: None,
    fcd_url: None,
};
