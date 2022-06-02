use std::{env, fs::File, str::FromStr};

use cosmrs::Denom;
use serde::de::IntoDeserializer;
use serde_json::from_reader;

use crate::config::GroupConfig;
use tonic::transport::Channel;

use crate::error::TerraRustScriptError;

#[derive(Clone, Debug)]
pub struct NetworkConfig {
    /// What kind of network
    pub network: NetworkKind,
    /// Identifier for the network ex. columbus-2
    pub network_id: String,
    /// gRPC channel
    pub grpc_channel: Channel,
    /// Underlying chain details
    pub chain: Chain,
    /// Max gas and denom info
    pub gas_denom: Denom,
    /// gas price
    pub gas_price: f64,
    /// Optional urls for custom functionality
    pub lcd_url: Option<String>,
    pub fcd_url: Option<String>,
}

impl NetworkConfig {
    pub async fn new(chain: Chain) -> Result<Self, TerraRustScriptError> {
        let chains_json_path = env::var("CHAINS")?;
        let file = File::open(&chains_json_path)
            .expect(&format!("file should be present at {}", chains_json_path));
        let json: serde_json::Value = from_reader(file)?;

        let network_kind = NetworkKind::new()?;

        let network_value = json[chain.chain_id.clone()][network_kind.to_string()].clone();

        let grpc_url = network_value["grpc_url"].as_str().unwrap().to_string();
        let grpc_channel = Channel::from_shared(grpc_url).unwrap().connect().await?;

        let network_id = network_value["id"].as_str().unwrap().to_string();
        let lcd_url = network_value
            .get("lcd")
            .map(|v| v.as_str().unwrap().to_string());
        let fcd_url = network_value
            .get("fcd")
            .map(|v| v.as_str().unwrap().to_string());
        let gas_denom: Denom = Denom::from_str(network_value["gas_denom"].as_str().unwrap())?;
        let gas_price = network_value["gas_price"].as_f64().unwrap();

        Ok(Self {
            network: network_kind,
            network_id,
            grpc_channel,
            chain,
            gas_denom,
            gas_price,
            lcd_url,
            fcd_url,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Chain {
    /// Name of the chain, ex Juno, Terra, ...
    pub chain_id: String,
    /// address prefix
    pub pub_addr_prefix: String,
    /// coin type for key derivation
    pub coin_type: u32,
}

impl Chain {
    pub async fn new(chain: &str) -> Result<Self, TerraRustScriptError> {
        let chains_json_path = env::var("CHAINS")?;
        let file = File::open(&chains_json_path)
            .expect(&format!("file should be present at {}", chains_json_path));
        let json: serde_json::Value = from_reader(file)?;
        Ok(Self {
            chain_id: chain.to_string(),
            coin_type: json[chain]["coin_type"].as_u64().unwrap() as u32,
            pub_addr_prefix: json[chain]["pub_addr_prefix"].as_str().unwrap().into(),
        })
    }
}

#[derive(Clone, Debug)]
pub enum NetworkKind {
    Local,
    Mainnet,
    Testnet,
}

impl NetworkKind {
    pub fn new() -> Result<Self, TerraRustScriptError> {
        let network_id = env::var("NETWORK")?;
        let network = match network_id.as_str() {
            "testnet" => NetworkKind::Testnet,
            "mainnet" => NetworkKind::Mainnet,
            _ => NetworkKind::Local,
        };
        Ok(network)
    }

    pub fn mnemonic_name(&self) -> &str {
        match *self {
            NetworkKind::Local => "LOCAL_MNEMONIC",
            NetworkKind::Mainnet => "MAIN_MNEMONIC",
            NetworkKind::Testnet => "TEST_MNEMONIC",
        }
    }

    pub fn multisig_name(&self) -> &str {
        match *self {
            NetworkKind::Local => "LOCAL_MULTISIG",
            NetworkKind::Mainnet => "MAIN_MULTISIG",
            NetworkKind::Testnet => "TEST_MULTISIG",
        }
    }
}

impl ToString for NetworkKind {
    fn to_string(&self) -> String {
        match *self {
            NetworkKind::Local => "local",
            NetworkKind::Mainnet => "mainnet",
            NetworkKind::Testnet => "testnet",
        }
        .into()
    }
}
