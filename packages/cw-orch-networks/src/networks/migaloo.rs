use crate::networks::{ChainInfoConst, ChainKind, NetworkInfoConst};

// ANCHOR: migaloo
pub const MIGALOO_NETWORK: NetworkInfoConst = NetworkInfoConst {
    id: "migaloo-1",
    pub_address_prefix: "migaloo",
    coin_type: 118u32,
};

pub const LOCAL_MIGALOO: ChainInfoConst = ChainInfoConst {
    kind: ChainKind::Local,
    chain_id: "migaloo-chain",
    gas_denom: "uwhale",
    gas_price: 0.1,
    grpc_urls: &["http://localhost:9090"],
    network_info: MIGALOO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

/// <https://docs.migaloo.zone/validators/testnet>
pub const NARWHAL_1: ChainInfoConst = ChainInfoConst {
    kind: ChainKind::Testnet,
    chain_id: "narwhal-1",
    gas_denom: "uwhale",
    gas_price: 0.1,
    grpc_urls: &["migaloo-testnet-grpc.polkachu.com:20790"],
    network_info: MIGALOO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

/// <https://docs.migaloo.zone/validators/mainnet>
pub const MIGALOO_1: ChainInfoConst = ChainInfoConst {
    kind: ChainKind::Mainnet,
    chain_id: "migaloo-1",
    gas_denom: "uwhale",
    gas_price: 0.1,
    grpc_urls: &["migaloo-grpc.polkachu.com:20790"],
    network_info: MIGALOO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
// ANCHOR_END: migaloo
