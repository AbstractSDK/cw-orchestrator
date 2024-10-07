use cw_orch_core::environment::{ChainInfo, ChainKind, NetworkInfo};

// ANCHOR: cosmos
pub const COSMOS_HUB_NETWORK: NetworkInfo = NetworkInfo {
    chain_name: "cosmoshub",
    pub_address_prefix: "cosmos",
    coin_type: 118,
};

pub const COSMOS_HUB_TESTNET: ChainInfo = ICS_TESTNET;

/// Cosmos Hub Testnet
/// @see https://github.com/cosmos/testnets/blob/master/interchain-security/provider/README.md
/// Use the faucet here: https://faucet.polypore.xyz
pub const ICS_TESTNET: ChainInfo = ChainInfo {
    kind: ChainKind::Testnet,
    chain_id: "provider",
    gas_denom: "uatom",
    gas_price: 0.0025,
    grpc_urls: &["https://grpc-rs.cosmos.nodestake.top:443"],
    network_info: COSMOS_HUB_NETWORK,
    lcd_url: Some("https://api-rs.cosmos.nodestake.top:443"),
    fcd_url: None,
};

// ANCHOR_END: cosmos
