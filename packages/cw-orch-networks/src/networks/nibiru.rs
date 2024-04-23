use crate::chain_info::{ChainInfoConst, ChainKind, NetworkInfoConst};

// ANCHOR: nibiru
pub const NIBIRU_NETWORK: NetworkInfoConst = NetworkInfoConst {
    id: "nibiru",
    pub_address_prefix: "nibi",
    coin_type: 118u32,
};

pub const NIBIRU_ITN_2: ChainInfoConst = ChainInfoConst {
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
