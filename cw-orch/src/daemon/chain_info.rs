use std::env;

use serde::{Deserialize, Serialize};

use crate::prelude::CwOrchError;
use ibc_chain_registry::chain::{Apis, ChainData as RegistryChainInfo, FeeToken, FeeTokens, Grpc};

#[allow(clippy::from_over_into)]
impl Into<RegistryChainInfo> for ChainInfo<'_> {
    fn into(self) -> RegistryChainInfo {
        RegistryChainInfo {
            chain_name: self.network_info.id.to_string(),
            chain_id: self.chain_id.to_string().into(),
            bech32_prefix: self.network_info.pub_address_prefix.into(),
            fees: FeeTokens {
                fee_tokens: vec![FeeToken {
                    fixed_min_gas_price: self.gas_price,
                    denom: self.gas_denom.to_string(),
                    ..Default::default()
                }],
            },
            network_type: self.kind.to_string(),
            apis: Apis {
                grpc: self
                    .grpc_urls
                    .iter()
                    .map(|url| Grpc {
                        address: url.to_string(),
                        ..Default::default()
                    })
                    .collect(),
                ..Default::default()
            },
            slip44: self.network_info.coin_type,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChainInfo<'a> {
    /// Identifier for the network ex. columbus-2
    pub chain_id: &'a str,
    /// Max gas and denom info
    // #[serde(with = "cosm_denom_format")]
    pub gas_denom: &'a str,
    /// gas price
    pub gas_price: f64,
    pub grpc_urls: &'a [&'a str],
    /// Optional urls for custom functionality
    pub lcd_url: Option<&'a str>,
    pub fcd_url: Option<&'a str>,
    /// Underlying network details (coin type, address prefix, etc)
    pub network_info: NetworkInfo<'a>,
    pub kind: ChainKind,
}

#[derive(Clone, Debug)]
pub struct ChainInfoOwned {
    /// Identifier for the network ex. columbus-2
    pub chain_id: String,
    /// Max gas and denom info
    // #[serde(with = "cosm_denom_format")]
    pub gas_denom: String,
    /// gas price
    pub gas_price: f64,
    pub grpc_urls: Vec<String>,
    /// Optional urls for custom functionality
    pub lcd_url: Option<String>,
    pub fcd_url: Option<String>,
    /// Underlying network details (coin type, address prefix, etc)
    pub network_info: NetworkInfoOwned,
    pub kind: ChainKind,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct NetworkInfo<'a> {
    pub id: &'a str,
    /// address prefix
    pub pub_address_prefix: &'a str,
    /// coin type for key derivation
    pub coin_type: u32,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct NetworkInfoOwned {
    pub id: String,
    /// address prefix
    pub pub_address_prefix: String,
    /// coin type for key derivation
    pub coin_type: u32,
}

impl From<NetworkInfo<'_>> for NetworkInfoOwned {
    fn from(info: NetworkInfo<'_>) -> Self {
        Self {
            id: info.id.to_owned(),
            pub_address_prefix: info.pub_address_prefix.to_owned(),
            coin_type: info.coin_type,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChainKind {
    Local,
    Mainnet,
    Testnet,
}

impl ChainKind {
    pub fn new() -> Result<Self, CwOrchError> {
        let network_id = env::var("NETWORK").expect("NETWORK is not set");
        let network = match network_id.as_str() {
            "testnet" => ChainKind::Testnet,
            "mainnet" => ChainKind::Mainnet,
            _ => ChainKind::Local,
        };
        Ok(network)
    }

    pub fn mnemonic_name(&self) -> &str {
        match *self {
            ChainKind::Local => "LOCAL_MNEMONIC",
            ChainKind::Testnet => "TEST_MNEMONIC",
            ChainKind::Mainnet => "MAIN_MNEMONIC",
        }
    }

    pub fn multisig_name(&self) -> &str {
        match *self {
            ChainKind::Local => "LOCAL_MULTISIG",
            ChainKind::Testnet => "TEST_MULTISIG",
            ChainKind::Mainnet => "MAIN_MULTISIG",
        }
    }
}

impl ToString for ChainKind {
    fn to_string(&self) -> String {
        match *self {
            ChainKind::Local => "local",
            ChainKind::Testnet => "testnet",
            ChainKind::Mainnet => "mainnet",
        }
        .into()
    }
}

impl From<String> for ChainKind {
    fn from(str: String) -> Self {
        match str.as_str() {
            "local" => ChainKind::Local,
            "testnet" => ChainKind::Testnet,
            "mainnet" => ChainKind::Mainnet,
            _ => ChainKind::Local,
        }
    }
}
