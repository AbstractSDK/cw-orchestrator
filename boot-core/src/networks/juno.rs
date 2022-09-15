use crate::{
    data_structures::daemon_state::{ChainInfo, NetworkInfo},
    NetworkKind,
};

pub const JUNO_CHAIN: ChainInfo = ChainInfo {
    chain_id: "juno",
    pub_address_prefix: "juno",
    coin_type: 118u32,
};

pub const UNI_3: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Testnet,
    id: "uni-3",
    gas_denom: "ujunox",
    gas_price: 0.025,
    grpc_url: "http://ssh.ohmroger.com:9099",
    chain_info: JUNO_CHAIN,
    lcd_url: None,
    fcd_url: None,
};

pub const JUNO_1: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Mainnet,
    id: "juno-1",
    gas_denom: "ujuno",
    gas_price: 0.0026,
    grpc_url: "http://65.108.235.46:26090",
    chain_info: JUNO_CHAIN,
    lcd_url: None,
    fcd_url: None,
};

pub const JUNO_DAEMON: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Local,
    id: "testing",
    gas_denom: "ustake",
    gas_price: 0.0,
    grpc_url: "http://localhost:9090",
    chain_info: JUNO_CHAIN,
    lcd_url: None,
    fcd_url: None,
};
