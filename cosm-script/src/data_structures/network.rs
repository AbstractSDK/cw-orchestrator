use std::{env, fs::File, str::FromStr};

use cosmrs::Denom;

use crate::cosm_denom_format;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, from_value, json, to_value, value::Index};

use tonic::transport::Channel;

use crate::error::CosmScriptError;

#[derive(Clone, Debug)]
pub struct Network {
    /// What kind of network
    pub kind: NetworkKind,
    /// Identifier for the network ex. columbus-2
    pub id: String,
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

impl Network {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkInfo {
    /// Identifier for the network ex. columbus-2
    pub id: String,
    /// Max gas and denom info
    #[serde(with = "cosm_denom_format")]
    pub gas_denom: Denom,
    /// gas price
    pub gas_price: f64,
    pub grpc_url: String,
    /// Optional urls for custom functionality
    pub lcd_url: Option<String>,
    pub fcd_url: Option<String>,
}

impl Default for NetworkInfo {
    fn default() -> Self {
        Self {
            gas_denom: Denom::from_str("").unwrap(),
            id: String::default(),
            gas_price: 0f64,
            grpc_url:String::default(),
            lcd_url: None,
            fcd_url: None,
            
        }
    }
}

impl NetworkInfo {
    pub async fn into_network(
        self,
        kind: NetworkKind,
        chain: &Chain,
    ) -> Result<Network, CosmScriptError> {
        let grpc_channel = Channel::from_shared(self.grpc_url)
            .unwrap()
            .connect()
            .await?;

        Ok(Network {
            kind,
            grpc_channel,
            chain: chain.clone(),
            id: self.id,
            gas_denom: self.gas_denom,
            gas_price: self.gas_price,
            lcd_url: self.lcd_url,
            fcd_url: self.fcd_url,
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

    file_path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ChainInfo {
    /// address prefix
    pub pub_addr_prefix: String,
    /// coin type for key derivation
    pub coin_type: u32,
}

impl Chain {
    pub async fn get(chain_id: &str) -> Result<Self, CosmScriptError> {
        let chains_json_path = env::var("CHAINS")?;
        let file = File::open(&chains_json_path)
            .expect(&format!("file should be present at {}", chains_json_path));
        let mut config: serde_json::Value = from_reader(file)?;

        match config.get(chain_id) {
            Some(chain) => {
                let info: ChainInfo = from_value(chain["info"].clone())?;
                if info.pub_addr_prefix == "FILL" {
                    return Err(CosmScriptError::NewChain(chains_json_path));
                };
                return Ok(Self {
                    chain_id: chain_id.into(),
                    pub_addr_prefix: info.pub_addr_prefix,
                    coin_type: info.coin_type,
                    file_path: chains_json_path,
                });
            }
            None => {
                let info = ChainInfo {
                    coin_type: 118u32,
                    pub_addr_prefix: "FILL".into(),
                };
                config[chain_id] = json!(
                    {
                        "info": info,
                        "networks": {}
                    }
                );
                serde_json::to_writer_pretty(File::create(&chains_json_path)?, &config)?;
                Err(CosmScriptError::NewChain(chains_json_path))
            }
        }
    }

    pub async fn network(&self) -> Result<Network, CosmScriptError> {
        let file =
            File::open(&self.file_path).expect(&format!("file present at {}", self.file_path));
        let mut config: serde_json::Value = from_reader(file)?;
        
        let network_kind = NetworkKind::new()?;
        
        let network = config[&self.chain_id]["networks"].get(network_kind.to_string());
        
        match network {
            Some(network) => {
                let info: NetworkInfo = from_value(network["info"].clone())?;
                if info.grpc_url == String::default() {
                    return Err(CosmScriptError::NewNetwork(self.file_path.clone()));
                }
                info.into_network(network_kind, self).await
            }
            // Fill scaffold for user
            None => {
                let info = NetworkInfo::default();
                config[&self.chain_id]["networks"][network_kind.to_string()] = json!(
                    {
                        "info": info,
                        "code_ids": {},
                        "deployments": {
                            "default": {}
                        }
                    }
                );
                serde_json::to_writer_pretty(File::create(&self.file_path)?, &config)?;
                Err(CosmScriptError::NewNetwork(self.file_path.clone()))
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetworkKind {
    Local,
    Mainnet,
    Testnet,
}

impl NetworkKind {
    pub fn new() -> Result<Self, CosmScriptError> {
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
