use crate::chain_info::{ChainInfo, ChainKind, NetworkInfo};

// https://notional.ventures/resources/endpoints#juno

pub const JUNO_NETWORK: NetworkInfo = NetworkInfo {
    id: "juno",
    pub_address_prefix: "juno",
    coin_type: 118u32,
};

#[deprecated(
    since = "0.6.1",
    note = "Uni-5 does not exist anymore. Use Uni-6 instead."
)]
pub const UNI_5: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "uni-5",
    gas_denom: "ujunox",
    gas_price: 0.025,
    grpc_urls: &["https://juno-testnet-grpc.polkachu.com:12690"],
    network_info: JUNO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

pub const UNI_6: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "uni-6",
    gas_denom: "ujunox",
    gas_price: 0.0251,
    grpc_urls: &["http://juno-testnet-grpc.polkachu.com:12690"],
    network_info: JUNO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

pub const JUNO_1: ChainInfo = ChainInfo {
    kind: ChainKind::Mainnet,
    chain_id: "juno-1",
    gas_denom: "ujuno",
    gas_price: 0.0750,
    grpc_urls: &[
        "https://grpc-juno-ia.cosmosia.notional.ventures",
        "http://juno-grpc.polkachu.com:12690",
    ],
    network_info: JUNO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

pub const LOCAL_JUNO: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "testing",
    gas_denom: "ujunox",
    gas_price: 0.0,
    grpc_urls: &["http://localhost:9090"],
    network_info: JUNO_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
