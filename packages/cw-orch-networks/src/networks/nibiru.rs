use cw_orch_core::environment::{ChainInfo, ChainKind, NetworkInfo};

// ANCHOR: nibiru
pub const NIBIRU_NETWORK: NetworkInfo = NetworkInfo {
    id: "nibiru",
    pub_address_prefix: "nibi",
    coin_type: 118u32,
};

pub const NIBIRU_ITN_2: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "nibiru-itn-2",
    gas_denom: "unibi",
    gas_price: 0.025,
    grpc_urls: &["https://nibiru-testnet.grpc.kjnodes.com:443"],
    network_info: NIBIRU_NETWORK,
    lcd_url: None,
    fcd_url: None,
};
// ANCHOR_END: nibiru
