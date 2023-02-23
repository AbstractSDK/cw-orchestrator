use crate::networks::{ChainInfo, NetworkInfo, NetworkKind};

pub const NEUTRON_CHAIN: ChainInfo = ChainInfo {
    chain_id: "neutron",
    pub_address_prefix: "neutron",
    coin_type: 118u32,
};

/// https://github.com/neutron-org/cosmos-testnets/tree/master/replicated-security/baryon-1
pub const BARYON_1: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Testnet,
    id: "baryon-1",
    gas_denom: "untrn",
    gas_price: 0.001,
    grpc_urls: &["http://grpc.baryon.ntrn.info:80"],
    chain_info: NEUTRON_CHAIN,
    lcd_url: Some("https://rest.baryon.ntrn.info"),
    fcd_url: None,
};
