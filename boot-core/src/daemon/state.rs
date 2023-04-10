use super::error::DaemonError;
use crate::error::BootError;
use crate::state::StateInterface;
use cosmrs::{
    proto::cosmos::base::tendermint::v1beta1::{service_client::ServiceClient, GetNodeInfoRequest},
    Denom,
};
use cosmwasm_std::Addr;
use ibc_chain_registry::chain::{Apis, ChainData as RegistryChainInfo, FeeToken, FeeTokens, Grpc};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, json, Value};
use std::{collections::HashMap, env, fs::{File, OpenOptions}, rc::Rc, str::FromStr};
use tonic::transport::{Channel, ClientTlsConfig};
pub const DEFAULT_DEPLOYMENT: &str = "default";

/*
    the proper way of using DaemonOptions is using DaemonOptionsBuilder
    here is an example of how:
    let options = DaemonOptionsBuilder::default()
        .network(LOCAL_JUNO)
        .deployment_id("v0.1.0")
        .build()
        .unwrap();
*/
#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct DaemonOptions {
    #[builder(setter(into))]
    network: RegistryChainInfo,
    #[builder(setter(into, strip_option))]
    #[builder(default)]
    // #[builder(setter(strip_option))]
    deployment_id: Option<String>,
}

#[derive(Clone, Debug)]
pub struct DaemonState {
    // this is passed via env var STATE_FILE
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
    pub async fn new(options: DaemonOptions) -> Result<DaemonState, DaemonError> {
        let network: RegistryChainInfo = options.network;

        let mut successful_connections = vec![];

        log::debug!("Found {} gRPC endpoints", network.apis.grpc.len());

        // find working grpc channel
        for Grpc { address, .. } in network.apis.grpc.iter() {
            // get grpc endpoint
            let endpoint = Channel::builder(address.clone().try_into().unwrap());

            // try to connect to grpc endpoint
            let maybe_client = ServiceClient::connect(endpoint.clone()).await;

            // connection succeeded
            let mut client = if maybe_client.is_ok() {
                maybe_client?
            } else {
                log::warn!(
                    "Cannot connect to gRPC endpoint: {}, {:?}",
                    address,
                    maybe_client.unwrap_err()
                );

                // try HTTPS approach
                // https://github.com/hyperium/tonic/issues/363#issuecomment-638545965
                if !(address.contains("https") || address.contains("443")) {
                    continue;
                };

                log::info!("Attempting to connect with TLS");

                // re attempt to connect
                let endpoint = endpoint.clone().tls_config(ClientTlsConfig::new())?;
                let maybe_client = ServiceClient::connect(endpoint.clone()).await;

                // connection still fails
                if maybe_client.is_err() {
                    log::warn!(
                        "Cannot connect to gRPC endpoint: {}, {:?}",
                        address,
                        maybe_client.unwrap_err()
                    );
                    continue;
                };

                maybe_client?
            };

            // get client information for verification down below
            let node_info = client
                .get_node_info(GetNodeInfoRequest {})
                .await?
                .into_inner();

            // verify we are connected to the spected network
            if node_info.default_node_info.as_ref().unwrap().network != network.chain_id.as_str() {
                log::error!(
                    "Network mismatch: connection:{} != config:{}",
                    node_info.default_node_info.as_ref().unwrap().network,
                    network.chain_id.as_str()
                );
                continue;
            }

            // add endpoint to succesful connections
            successful_connections.push(endpoint.connect().await?)
        }

        // we could not get any succesful connections
        if successful_connections.is_empty() {
            return Err(DaemonError::CannotConnectGRPC);
        }

        // check if STATE_FILE en var is configured, fail if not
        let mut path = env::var("STATE_FILE").expect("STATE_FILE is not set");

        // if the network we are connecting is a local kind, add it to the fn
        if network.network_type == NetworkKind::Local.to_string() {
            let name = path.split('.').next().unwrap();
            path = format!("{name}_local.json");
        }

        // Try to get the standard fee token (probably shortest denom)
        let shortest_denom_token =
            network
                .fees
                .fee_tokens
                .iter()
                .fold(network.fees.fee_tokens[0].clone(), |acc, item| {
                    if item.denom.len() < acc.denom.len() {
                        item.clone()
                    } else {
                        acc
                    }
                });

        // build daemon state
        let state = DaemonState {
            json_file_path: path,
            kind: NetworkKind::from(network.network_type),
            deployment_id: options
                .deployment_id
                .map(Into::into)
                .unwrap_or_else(|| DEFAULT_DEPLOYMENT.into()),
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

        // write json state file
        state.write_state_json();

        // finish
        Ok(state)
    }

    // maybe we shold rename this?
    pub fn write_state_json(&self) {
        // check file exists
        let file_exists = std::path::Path::new(&self.json_file_path).exists();

        // create file if dont exists, set read/write permissions to true
        // dont truncate it
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&self.json_file_path)
            .unwrap();

        log::info!("Opening daemon state at {}", self.json_file_path);

        // read file content from fp
        // return empty json object if the file was just created
        let mut json: serde_json::Value = if file_exists {
            if file.metadata().unwrap().len().eq(&0) {
                json!({})
            } else {
                serde_json::from_reader(&file).unwrap()
            }
        } else {
            json!({})
        };

        // check and add chain_id path if it's missing
        if json.get(&self.chain.chain_id).is_none() {
            json[&self.chain.chain_id] = json!({});
        }

        // add deployment_id to chain_id path
        if json[&self.chain.chain_id].get(&self.id).is_none() {
            json[&self.chain.chain_id][&self.id] = json!({
                &self.deployment_id: {},
                "code_ids": {}
            });
        }

        // write JSON data
        // use File::create so we dont append data to the file
        // but rather write all (because we have read the data before)
        serde_json::to_writer_pretty(
            File::create(&self.json_file_path).unwrap(),
            &json
        ).unwrap();
    }

    pub fn set_deployment(&mut self, deployment_id: impl Into<String>) {
        self.deployment_id = deployment_id.into();
        self.write_state_json();
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
            .ok_or_else(|| BootError::AddrNotInStore(contract_id.to_owned()))?
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
            .ok_or_else(|| BootError::CodeIdNotInStore(contract_id.to_owned()))?
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

#[allow(clippy::from_over_into)]
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
        let network_id = env::var("NETWORK").expect("NETWORK is not set");
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
