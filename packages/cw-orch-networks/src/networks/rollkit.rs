use crate::{ChainInfo, ChainKind, NetworkInfo};

pub const CELESTIA_NETWORK: NetworkInfo = NetworkInfo {
    id: "rollkit",
    pub_address_prefix: "wasm",
    coin_type: 118u32,
};

/// <https://github.com/cosmos/chain-registry/blob/master/testnets/neutrontestnet/chain.json>
pub const ROLLKIT_1: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "celeswasm",
    gas_denom: "uwasm",
    gas_price: 0.025,
    grpc_urls: &["http://localhost:9290"],
    network_info: CELESTIA_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
