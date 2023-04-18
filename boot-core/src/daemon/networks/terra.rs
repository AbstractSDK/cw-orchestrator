use crate::daemon::state::{ChainInfo, NetworkInfo, NetworkKind};

pub const TERRA_CHAIN: ChainInfo = ChainInfo {
    network_id: "terra",
    pub_address_prefix: "terra",
    coin_type: 330u32,
};

pub const PISCO_1: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Testnet,
    chain_id: "pisco-1",
    gas_denom: "uluna",
    gas_price: 0.15,
    grpc_urls: &["http://terra-testnet-grpc.polkachu.com:11790"],
    chain_info: TERRA_CHAIN,
    lcd_url: None,
    fcd_url: None,
};

pub const PHOENIX_1: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Mainnet,
    chain_id: "phoenix-1",
    gas_denom: "uluna",
    gas_price: 0.15,
    grpc_urls: &["https://terra-grpc.polkachu.com:11790"],
    chain_info: TERRA_CHAIN,
    lcd_url: None,
    fcd_url: None,
};

pub const LOCAL_TERRA: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Local,
    chain_id: "localterra",
    gas_denom: "uluna",
    gas_price: 0.15,
    grpc_urls: &["http://65.108.235.46:9090"],
    chain_info: TERRA_CHAIN,
    lcd_url: None,
    fcd_url: None,
};
