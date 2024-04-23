use crate::{ChainInfoConst, ChainKind, NetworkInfoConst};

// ANCHOR: rollkit
pub const ROLLKIT_NETWORK: NetworkInfoConst = NetworkInfoConst {
    id: "rollkit",
    pub_address_prefix: "wasm",
    coin_type: 118u32,
};

pub const LOCAL_ROLLKIT: ChainInfoConst = ChainInfoConst {
    kind: ChainKind::Local,
    chain_id: "celeswasm",
    gas_denom: "uwasm",
    gas_price: 0.025,
    grpc_urls: &["http://localhost:9290"],
    network_info: ROLLKIT_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
// ANCHOR_END: rollkit
