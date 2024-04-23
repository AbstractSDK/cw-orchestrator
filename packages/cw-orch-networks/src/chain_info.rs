use std::fmt::Display;

use serde::{Deserialize, Serialize};

use cw_orch_core::{
    env::{LOCAL_MNEMONIC_ENV_NAME, MAIN_MNEMONIC_ENV_NAME, TEST_MNEMONIC_ENV_NAME},
    CwEnvError, CwOrchEnvVars,
};

pub trait StringType {
    type StringType;
    type StringArrayType;
}

pub type ChainInfoConst = ChainInfoBase<&'static str, &'static [&'static str]>;
pub type ChainInfo = ChainInfoBase<String, Vec<String>>;

pub type NetworkInfoConst = NetworkInfoBase<&'static str>;
pub type NetworkInfo = NetworkInfoBase<String>;

/// Information about a chain.
/// This is used to connect to a chain and to generate transactions.
#[derive(Clone, Debug)]
pub struct ChainInfoBase<StringType, StringArrayType> {
    /// Identifier for the network ex. columbus-2
    pub chain_id: StringType,
    /// Max gas and denom info
    // #[serde(with = "cosm_denom_format")]
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
#[derive(Clone, Debug, Serialize, Default)]
pub struct NetworkInfoBase<StringType> {
    /// network identifier (ex. juno, terra, osmosis, etc)
    pub id: StringType,
    /// address prefix
    pub pub_address_prefix: StringType,
    /// coin type for key derivation
    pub coin_type: u32,
}

impl From<ChainInfoConst> for ChainInfo {
    fn from(value: ChainInfoConst) -> Self {
        ChainInfo {
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
impl From<NetworkInfoConst> for NetworkInfo {
    fn from(value: NetworkInfoConst) -> Self {
        NetworkInfo {
            id: value.id.to_string(),
            pub_address_prefix: value.pub_address_prefix.to_string(),
            coin_type: value.coin_type,
        }
    }
}

/// Kind of chain (local, testnet, mainnet)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChainKind {
    /// A local chain, used for development
    Local,
    /// A mainnet chain
    Mainnet,
    /// A testnet chain
    Testnet,
}

impl ChainKind {
    /// Get the mnemonic name for the chain kind
    pub fn mnemonic_env_variable_name(&self) -> &str {
        match *self {
            ChainKind::Local => LOCAL_MNEMONIC_ENV_NAME,
            ChainKind::Testnet => TEST_MNEMONIC_ENV_NAME,
            ChainKind::Mainnet => MAIN_MNEMONIC_ENV_NAME,
        }
    }

    pub fn mnemonic(&self) -> Result<String, CwEnvError> {
        let env_vars = CwOrchEnvVars::load()?;
        match *self {
            ChainKind::Local => env_vars.local_mnemonic,
            ChainKind::Testnet => env_vars.test_mnemonic,
            ChainKind::Mainnet => env_vars.main_mnemonic,
        }
        .ok_or(CwEnvError::EnvVarNotPresentNamed(
            self.mnemonic_env_variable_name().to_string(),
        ))
    }
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
