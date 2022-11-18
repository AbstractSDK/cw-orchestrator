use crate::daemon::state::{ChainInfo, NetworkInfo, NetworkKind};

pub const JUNO_CHAIN: ChainInfo = ChainInfo {
    chain_id: "juno",
    pub_address_prefix: "juno",
    coin_type: 118u32,
};

pub const UNI_5: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Testnet,
    id: "uni-5",
    gas_denom: "ujunox",
    gas_price: 0.026,
    grpc_url: "https://juno-testnet-grpc.polkachu.com:12690",
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

pub const LOCAL_JUNO: NetworkInfo = NetworkInfo {
    kind: NetworkKind::Local,
    id: "testing",
    gas_denom: "ustake",
    gas_price: 0.0,
    grpc_url: "http://localhost:9090",
    chain_info: JUNO_CHAIN,
    lcd_url: None,
    fcd_url: None,
};
