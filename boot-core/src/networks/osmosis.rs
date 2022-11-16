use crate::{
    data_structures::daemon_state::{ChainInfo, NetworkInfo},
    NetworkKind,
};

pub const OSMO_CHAIN: ChainInfo = ChainInfo {
    chain_id: "osmosis",
    pub_address_prefix: "osmo",
    coin_type: 118u32,
};

pub const OSMO_4: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Testnet,
    id: "osmo-test-4",
    gas_denom: "uosmo",
    gas_price: 0.0031,
    grpc_url: "http://65.108.235.46:9092",
    chain_info: OSMO_CHAIN,
    lcd_url: None,
    fcd_url: None,
};

// pub const JUNO_1: NetworkInfo = NetworkInfo {
//     kind: NetworkKind::Mainnet,
//     id: "juno-1",
//     gas_denom: "ujuno",
//     gas_price: 0.0026,
//     grpc_url: "http://65.108.235.46:26090",
//     chain_info: JUNO_CHAIN,
//     lcd_url: None,
//     fcd_url: None,
// };

pub const OSMO_DAEMON: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Local,
    id: "localosmosis",
    gas_denom: "uosmo",
    gas_price: 0.0026,
    grpc_url: "http://65.108.235.46:9094",
    chain_info: OSMO_CHAIN,
    lcd_url: None,
    fcd_url: None,
};
