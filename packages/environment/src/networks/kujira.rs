use crate::networks::{ChainInfo, ChainKind, NetworkInfo};

pub const KUJIRA_NETWORK: NetworkInfo = NetworkInfo {
    id: "kujira",
    pub_address_prefix: "kujira",
    coin_type: 118u32,
};

pub const HARPOON_4: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "harpoon-4",
    gas_denom: "ukuji",
    gas_price: 0.025,
    grpc_urls: &["http://kujira-testnet-grpc.polkachu.com:11890"],
    network_info: KUJIRA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
