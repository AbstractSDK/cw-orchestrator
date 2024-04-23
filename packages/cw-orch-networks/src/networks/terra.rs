use cw_orch_core::environment::{ChainInfoConst, ChainKind, NetworkInfoConst};

// ANCHOR: terra
pub const TERRA_NETWORK: NetworkInfoConst = NetworkInfoConst {
    id: "terra2",
    pub_address_prefix: "terra",
    coin_type: 330u32,
};

/// Terra testnet network.
/// <https://docs.terra.money/develop/endpoints>
pub const PISCO_1: ChainInfoConst = ChainInfoConst {
    kind: ChainKind::Testnet,
    chain_id: "pisco-1",
    gas_denom: "uluna",
    gas_price: 0.015,
    grpc_urls: &["http://terra-testnet-grpc.polkachu.com:11790"],
    network_info: TERRA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

/// Terra mainnet network.
///<https://docs.terra.money/develop/endpoints>
pub const PHOENIX_1: ChainInfoConst = ChainInfoConst {
    kind: ChainKind::Mainnet,
    chain_id: "phoenix-1",
    gas_denom: "uluna",
    gas_price: 0.015,
    grpc_urls: &["http://terra-grpc.polkachu.com:11790"],
    network_info: TERRA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

/// Terra local network.
/// <https://docs.terra.money/develop/guides/initial/#next-steps-localterra-or-testnet>
pub const LOCAL_TERRA: ChainInfoConst = ChainInfoConst {
    kind: ChainKind::Local,
    chain_id: "localterra",
    gas_denom: "uluna",
    gas_price: 0.15,
    grpc_urls: &["http://localhost:9090"],
    network_info: TERRA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
// ANCHOR_END: terra
