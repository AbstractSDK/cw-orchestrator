use crate::error::BootError;
use crate::state::StateInterface;

use cosmrs::{Denom, proto::cosmos::base::tendermint::v1beta1::{service_client::ServiceClient, GetNodeInfoRequest}};
use cosmwasm_std::Addr;
use ibc_chain_registry::chain::{Apis, ChainData as RegistryChainInfo, FeeToken, FeeTokens, Grpc};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, json, Value};
use std::{collections::HashMap, env, fs::File, rc::Rc, str::FromStr};
use tonic::{transport::Channel, client::GrpcService};
pub const DEFAULT_DEPLOYMENT: &str = "default";

#[derive(Clone, Debug)]
pub struct DaemonState {
    pub json_file_path: String,
    /// What kind of network
    pub kind: NetworkKind,
    /// Identifier for the network ex. columbus-2
    pub id: String,
    /// Deployment identifier
    pub deployment_id: String,
    /// gRPC channel
    pub grpc_channel: Channel,
    /// Underlying chain details
    pub chain: ChainInfoOwned,
    /// Max gas and denom info
    pub gas_denom: Denom,
    /// gas price
    pub gas_price: f64,
    /// Optional urls for custom functionality
    pub lcd_url: Option<String>,
    pub fcd_url: Option<String>,
}

impl DaemonState {
    pub async fn new(network: impl Into<RegistryChainInfo>) -> Result<DaemonState, BootError> {
        let network: RegistryChainInfo = network.into();
        // find working grpc channel

        let mut successful_connections = vec![];
        for grpc in network.apis.grpc.iter() {
            let endpoint = Channel::builder(grpc.address.clone().try_into().unwrap());
            let maybe_client = ServiceClient::connect(endpoint.clone()).await;
            if maybe_client.is_err() {
                continue;
            }
            let node_info = maybe_client?.get_node_info(GetNodeInfoRequest{}).await?.into_inner();
            if node_info.default_node_info.as_ref().unwrap().network != network.chain_name {
                continue;
            }

            log::error!("{:?}", node_info.default_node_info.unwrap());
            successful_connections.push(
                endpoint.connect().await?
            )
        }

        if successful_connections.is_empty() {
            return Err(BootError::StdErr("No active grpc endpoint found.".into()));
        }

        log::error!("{:?}", successful_connections[0]);

        let mut path = env::var("STATE_FILE").unwrap();
        if network.network_type == NetworkKind::Local.to_string() {
            let name = path.split('.').next().unwrap();
            path = format!("{}_local.json", name);
        }

        // Try to get the standard fee token (probably shortest denom)
        let shortest_denom_token = network.fees.fee_tokens.iter().fold(
            network.fees.fee_tokens[0].clone(),
            |acc, item| {
                if item.denom.len() < acc.denom.len() {
                    item.clone()
                } else {
                    acc
                }
            },
        );

        let state = DaemonState {
            json_file_path: path,
            kind: NetworkKind::from(network.network_type),
            deployment_id: DEFAULT_DEPLOYMENT.into(),
            grpc_channel: successful_connections[0].clone(),
            chain: ChainInfoOwned {
                chain_id: network.chain_name.to_string(),
                pub_address_prefix: network.bech32_prefix,
                coin_type: network.slip44,
            },
            id: network.chain_id.to_string(),
            gas_denom: Denom::from_str(&shortest_denom_token.denom).unwrap(),
            gas_price: shortest_denom_token.fixed_min_gas_price,
            lcd_url: None,
            fcd_url: None,
        };
        state.check_file_validity();
        Ok(state)
    }

    pub fn check_file_validity(&self) {
        let file = File::open(&self.json_file_path).unwrap_or_else(|_| {
            let file = File::create(&self.json_file_path).unwrap();
            serde_json::to_writer_pretty(&file, &json!({})).unwrap();
            File::open(&self.json_file_path).unwrap()
        });
        log::info!("Opening daemon state at {}", self.json_file_path);
        let mut json: serde_json::Value = from_reader(file).unwrap();
        if json.get(&self.chain.chain_id).is_none() {
            json[&self.chain.chain_id] = json!({});
        }
        if json[&self.chain.chain_id].get(&self.id).is_none() {
            json[&self.chain.chain_id][&self.id] = json!({
                &self.deployment_id: {},
                "code_ids": {}
            });
        }

        serde_json::to_writer_pretty(File::create(&self.json_file_path).unwrap(), &json).unwrap();
    }

    pub fn set_deployment(&mut self, deployment_id: impl Into<String>) {
        self.deployment_id = deployment_id.into();
        self.check_file_validity();
    }

    /// Get the state filepath and read it as json
    fn json(&self) -> serde_json::Value {
        let file = File::open(&self.json_file_path)
            .unwrap_or_else(|_| panic!("file should be present at {}", self.json_file_path));
        let json: serde_json::Value = from_reader(file).unwrap();
        json
    }

    /// Retrieve a stateful value using the chainId and networkId
    fn get(&self, key: &str) -> Value {
        let json = self.json();
        json[&self.chain.chain_id][&self.id.to_string()][key].clone()
    }

    /// Set a stateful value using the chainId and networkId
    fn set<T: Serialize>(&self, key: &str, contract_id: &str, value: T) {
        let mut json = self.json();

        json[&self.chain.chain_id][&self.id.to_string()][key][contract_id] = json!(value);

        serde_json::to_writer_pretty(File::create(&self.json_file_path).unwrap(), &json).unwrap();
    }
}

impl StateInterface for Rc<DaemonState> {
    fn get_address(&self, contract_id: &str) -> Result<Addr, BootError> {
        let value = self
            .get(&self.deployment_id)
            .get(contract_id)
            .ok_or_else(|| BootError::AddrNotInFile(contract_id.to_owned()))?
            .clone();
        Ok(Addr::unchecked(value.as_str().unwrap()))
    }

    fn set_address(&mut self, contract_id: &str, address: &Addr) {
        self.set(&self.deployment_id, contract_id, address.as_str());
    }

    /// Get the locally-saved version of the contract's version on this network
    fn get_code_id(&self, contract_id: &str) -> Result<u64, BootError> {
        let value = self
            .get("code_ids")
            .get(contract_id)
            .ok_or_else(|| BootError::CodeIdNotInFile(contract_id.to_owned()))?
            .clone();
        Ok(value.as_u64().unwrap())
    }

    /// Set the locally-saved version of the contract's latest version on this network
    fn set_code_id(&mut self, contract_id: &str, code_id: u64) {
        self.set("code_ids", contract_id, code_id);
    }

    fn get_all_addresses(&self) -> Result<HashMap<String, Addr>, BootError> {
        let mut store = HashMap::new();
        let addresses = self.get(&self.deployment_id);
        let value = addresses.as_object().unwrap();
        for (id, addr) in value {
            store.insert(id.clone(), Addr::unchecked(addr.as_str().unwrap()));
        }
        Ok(store)
    }
    fn get_all_code_ids(&self) -> Result<HashMap<String, u64>, BootError> {
        let mut store = HashMap::new();
        let code_ids = self.get("code_ids");
        let value = code_ids.as_object().unwrap();
        for (id, code_id) in value {
            store.insert(id.clone(), code_id.as_u64().unwrap());
        }
        Ok(store)
    }
}

impl Into<RegistryChainInfo> for NetworkInfo<'_> {
    fn into(self) -> RegistryChainInfo {
        RegistryChainInfo {
            chain_name: self.chain_info.chain_id.to_string(),
            chain_id: self.id.to_string().into(),
            bech32_prefix: self.chain_info.pub_address_prefix.into(),
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
            slip44: self.chain_info.coin_type,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
pub struct NetworkInfo<'a> {
    /// Identifier for the network ex. columbus-2
    pub id: &'a str,
    /// Max gas and denom info
    // #[serde(with = "cosm_denom_format")]
    pub gas_denom: &'a str,
    /// gas price
    pub gas_price: f64,
    pub grpc_urls: &'a [&'a str],
    /// Optional urls for custom functionality
    pub lcd_url: Option<&'a str>,
    pub fcd_url: Option<&'a str>,
    pub chain_info: ChainInfo<'a>,
    pub kind: NetworkKind,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct ChainInfo<'a> {
    pub chain_id: &'a str,
    /// address prefix
    pub pub_address_prefix: &'a str,
    /// coin type for key derivation
    pub coin_type: u32,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct ChainInfoOwned {
    pub chain_id: String,
    /// address prefix
    pub pub_address_prefix: String,
    /// coin type for key derivation
    pub coin_type: u32,
}

impl From<ChainInfo<'_>> for ChainInfoOwned {
    fn from(info: ChainInfo<'_>) -> Self {
        Self {
            chain_id: info.chain_id.to_owned(),
            pub_address_prefix: info.pub_address_prefix.to_owned(),
            coin_type: info.coin_type,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkKind {
    Local,
    Mainnet,
    Testnet,
}

impl NetworkKind {
    pub fn new() -> Result<Self, BootError> {
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

impl From<String> for NetworkKind {
    fn from(str: String) -> Self {
        match str.as_str() {
            "local" => NetworkKind::Local,
            "mainnet" => NetworkKind::Mainnet,
            "testnet" => NetworkKind::Testnet,
            _ => NetworkKind::Local,
        }
    }
}
