use cw_orch_core::environment::{ChainInfo, ChainKind, NetworkInfo};

// https://notional.ventures/resources/endpoints#juno

// ANCHOR: juno
pub const DORAVOTA_NETWORK: NetworkInfo = NetworkInfo {
    id: "doravota",
    pub_address_prefix: "dora",
    coin_type: 118u32,
};

pub const VOTA_ASH: ChainInfo = ChainInfo {
    kind: ChainKind::Mainnet,
    chain_id: "vota-ash",
    gas_denom: "peaka",
    gas_price: 100000000000f64,
    grpc_urls: &["https://vota-grpc.dorafactory.org:443"],
    network_info: DORAVOTA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

pub const VOTA_TESTNET: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "vota-testnet",
    gas_denom: "peaka",
    gas_price: 100000000000f64,
    grpc_urls: &["https://vota-testnet-grpc.dorafactory.org:443"],
    network_info: DORAVOTA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
