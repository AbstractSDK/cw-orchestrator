use crate::networks::{ChainInfoConst, ChainKind, NetworkInfoConst};

// ANCHOR: kujira
pub const KUJIRA_NETWORK: NetworkInfoConst = NetworkInfoConst {
    id: "kujira",
    pub_address_prefix: "kujira",
    coin_type: 118u32,
};

pub const HARPOON_4: ChainInfoConst = ChainInfoConst {
    kind: ChainKind::Testnet,
    chain_id: "harpoon-4",
    gas_denom: "ukuji",
    gas_price: 0.025,
    grpc_urls: &["http://kujira-testnet-grpc.polkachu.com:11890"],
    network_info: KUJIRA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
// ANCHOR_END: kujira
