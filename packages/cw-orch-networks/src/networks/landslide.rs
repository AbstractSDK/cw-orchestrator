use cw_orch_core::environment::{ChainInfo, ChainKind, NetworkInfo};

// ANCHOR: landslide
pub const LANDSLIDE_NETWORK: NetworkInfo = NetworkInfo {
    chain_name: "landslide",
    pub_address_prefix: "wasm",
    coin_type: 118u32,
};

pub const LOCAL_LANDSLIDE: ChainInfo = ChainInfo {
    kind: ChainKind::Local,
    chain_id: "landslide-test",
    gas_denom: "stake",
    gas_price: 1_f64,
    grpc_urls: &["http://127.0.0.1:9090"],
    network_info: LANDSLIDE_NETWORK,
    lcd_url: None,
    fcd_url: None,
};

// ANCHOR_END: landslide
