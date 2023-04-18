use crate::daemon::state::{NetworkInfo, ChainInfo, ChainKind};

pub const TERRA_NETWORK: NetworkInfo = NetworkInfo {
    network_id: "terra",
    pub_address_prefix: "terra",
    coin_type: 330u32,
};

pub const PISCO_1: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "pisco-1",
    gas_denom: "uluna",
    gas_price: 0.15,
    grpc_urls: &["http://terra-testnet-grpc.polkachu.com:11790"],
    chain_info: TERRA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

pub const PHOENIX_1: ChainInfo = ChainInfo {
    kind: ChainKind::Mainnet,
    chain_id: "phoenix-1",
    gas_denom: "uluna",
    gas_price: 0.15,
    grpc_urls: &["https://terra-grpc.polkachu.com:11790"],
    chain_info: TERRA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

pub const LOCAL_TERRA: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "localterra",
    gas_denom: "uluna",
    gas_price: 0.15,
    grpc_urls: &["http://65.108.235.46:9090"],
    chain_info: TERRA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
