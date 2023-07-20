use crate::networks::{ChainInfo, ChainKind, NetworkInfo};

pub const NEUTRON_NETWORK: NetworkInfo = NetworkInfo {
    id: "neutron",
    pub_address_prefix: "neutron",
    coin_type: 118u32,
};

/// <https://github.com/cosmos/chain-registry/blob/master/testnets/neutrontestnet/chain.json>
pub const PION_1: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "pion-1",
    gas_denom: "untrn",
    gas_price: 0.001,
    grpc_urls: &["http://grpc-palvus.pion-1.ntrn.tech:80"],
    network_info: NEUTRON_NETWORK,
    lcd_url: Some("https://rest-palvus.pion-1.ntrn.tech"),
    fcd_url: None,
};

/// <https://github.com/cosmos/chain-registry/blob/master/neutron/chain.json>
pub const NEUTRON_1: ChainInfo = ChainInfo {
    kind: ChainKind::Mainnet,
    chain_id: "neutron-1",
    gas_denom: "untrn",
    gas_price: 0.001,
    grpc_urls: &["grpc-kralum.neutron-1.neutron.org:80"],
    network_info: NEUTRON_NETWORK,
    lcd_url: Some("https://rest-kralum.neutron-1.neutron.org"),
    fcd_url: None,
};

pub const LOCAL_NEUTRON: ChainInfo = ChainInfo {
    kind: ChainKind::Mainnet,
    chain_id: "test-1",
    gas_denom: "untrn",
    gas_price: 0.0025,
    grpc_urls: &["http://localhost:8090"],
    network_info: NEUTRON_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
