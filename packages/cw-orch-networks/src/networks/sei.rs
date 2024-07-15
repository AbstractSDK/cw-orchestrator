use crate::networks::{ChainInfo, ChainKind, NetworkInfo};

// ANCHOR: sei
pub const SEI_NETWORK: NetworkInfo = NetworkInfo {
    chain_name: "sei",
    pub_address_prefix: "sei",
    coin_type: 118u32,
};

pub const LOCAL_SEI: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "sei-chain",
    gas_denom: "usei",
    gas_price: 0.1,
    grpc_urls: &["http://localhost:9090"],
    rpc_urls: &[],
    network_info: SEI_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

pub const SEI_DEVNET_3: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "sei-devnet-3",
    gas_denom: "usei",
    gas_price: 0.1,
    grpc_urls: &["http://sei_devnet-testnet-grpc.polkachu.com:11990"],
    rpc_urls: &[],
    network_info: SEI_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

pub const ATLANTIC_2: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "atlantic-2",
    gas_denom: "usei",
    gas_price: 0.1,
    grpc_urls: &["http://sei-testnet-grpc.polkachu.com:11990"],
    rpc_urls: &[],
    network_info: SEI_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

pub const PACIFIC_1: ChainInfo = ChainInfo {
    kind: ChainKind::Mainnet,
    chain_id: "pacific-1",
    gas_denom: "usei",
    gas_price: 0.1,
    grpc_urls: &["http://sei-grpc.polkachu.com:11990"],
    network_info: SEI_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
// ANCHOR_END: sei
