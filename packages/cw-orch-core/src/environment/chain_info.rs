use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub type ChainInfo = ChainInfoBase<&'static str, &'static [&'static str]>;
pub type ChainInfoOwned = ChainInfoBase<String, Vec<String>>;

pub type NetworkInfo = NetworkInfoBase<&'static str>;
pub type NetworkInfoOwned = NetworkInfoBase<String>;

/// Information about a chain.
/// This is used to connect to a chain and to generate transactions.
#[derive(Clone, Debug, PartialEq)]
pub struct ChainInfoBase<StringType: Into<String> + Default, StringArrayType: AsRef<[StringType]>> {
    /// Identifier for the network ex. phoenix-2, pisco-1
    pub chain_id: StringType,
    /// Max gas and denom info
    pub gas_denom: StringType,
    /// gas price
    pub gas_price: f64,
    /// gRPC urls, used to attempt connection
    pub grpc_urls: StringArrayType,
    /// Optional urls for custom functionality
    pub lcd_url: Option<StringType>,
    /// Optional urls for custom functionality
    pub fcd_url: Option<StringType>,
    /// Underlying network details (coin type, address prefix, etc)
    pub network_info: NetworkInfoBase<StringType>,
    /// Chain kind, (local, testnet, mainnet)
    pub kind: ChainKind,
}

/// Information about the underlying network, used for key derivation
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct NetworkInfoBase<StringType: Into<String>> {
    /// network identifier (ex. juno, terra2, osmosis, etc)
    pub chain_name: StringType,
    /// address prefix
    pub pub_address_prefix: StringType,
    /// coin type for key derivation
    pub coin_type: u32,
}

impl<StringType: Into<String> + Default> Default for NetworkInfoBase<StringType> {
    fn default() -> Self {
        Self {
            chain_name: StringType::default(),
            pub_address_prefix: StringType::default(),
            // Default cosmos coin
            coin_type: 118,
        }
    }
}

impl<StringType: Into<String> + Default, StringArrayType: AsRef<[StringType]> + Default> Default
    for ChainInfoBase<StringType, StringArrayType>
{
    fn default() -> Self {
        Self {
            chain_id: Default::default(),
            gas_denom: Default::default(),
            gas_price: f64::NAN,
            grpc_urls: Default::default(),
            lcd_url: Default::default(),
            fcd_url: Default::default(),
            network_info: Default::default(),
            kind: ChainKind::Local,
        }
    }
}

impl From<ChainInfo> for ChainInfoOwned {
    fn from(value: ChainInfo) -> Self {
        ChainInfoOwned {
            chain_id: value.chain_id.to_string(),
            gas_denom: value.gas_denom.to_string(),
            gas_price: value.gas_price,
            grpc_urls: value.grpc_urls.iter().map(|url| url.to_string()).collect(),
            lcd_url: value.lcd_url.map(ToString::to_string),
            fcd_url: value.fcd_url.map(ToString::to_string),
            network_info: value.network_info.into(),
            kind: value.kind,
        }
    }
}
impl From<NetworkInfo> for NetworkInfoOwned {
    fn from(value: NetworkInfo) -> Self {
        NetworkInfoOwned {
            chain_name: value.chain_name.to_string(),
            pub_address_prefix: value.pub_address_prefix.to_string(),
            coin_type: value.coin_type,
        }
    }
}

/// Kind of chain (local, testnet, mainnet)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChainKind {
    /// A local chain, used for development
    Local,
    /// A mainnet chain
    Mainnet,
    /// A testnet chain
    Testnet,
}

impl Display for ChainKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match *self {
            ChainKind::Local => "local",
            ChainKind::Testnet => "testnet",
            ChainKind::Mainnet => "mainnet",
        };
        write!(f, "{}", str)
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

impl<StringType: Into<String> + Default, StringArrayType: AsRef<[StringType]> + Default>
    ChainInfoBase<StringType, StringArrayType>
{
    pub fn config(chain_id: StringType) -> Self {
        Self {
            chain_id,
            ..Default::default()
        }
    }
}
