use super::error::DaemonError;
use crate::{daemon::channel::DaemonChannel, error::BootError, state::StateInterface};
use cosmrs::Denom;
use cosmwasm_std::Addr;
use ibc_chain_registry::chain::{Apis, ChainData as RegistryChainInfo, FeeToken, FeeTokens, Grpc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, env, fs::File, path::Path, rc::Rc, str::FromStr};
use tonic::transport::Channel;

pub const DEFAULT_DEPLOYMENT: &str = "default";

/**
 Create [`DaemonOptions`] through [`DaemonOptionsBuilder`]
## Example
```
use boot_core::{DaemonOptionsBuilder, networks};

let options = DaemonOptionsBuilder::default()
    .network(networks::LOCAL_JUNO)
    .deployment_id("v0.1.0")
    .build()
    .unwrap();
```
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

impl DaemonOptions {
    pub fn get_network(&self) -> RegistryChainInfo {
        self.network.clone()
    }
}

#[derive(Clone, Debug)]
pub struct DaemonState {
    /// this is passed via env var STATE_FILE
    pub json_file_path: String,
    /// What kind of network
    pub kind: ChainKind,
    /// Identifier for the network ex. columbus-2
    pub chain_id: String,
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

        if network.apis.grpc.is_empty() {
            return Err(DaemonError::GRPCListIsEmpty);
        }

        log::info!("Found {} gRPC endpoints", network.apis.grpc.len());

        // find working grpc channel
        let grpc_channel = DaemonChannel::connect(&network.apis.grpc, &network.chain_id)
            .await?
            .unwrap();

        // check if STATE_FILE en var is configured, fail if not
        let mut json_file_path = env::var("STATE_FILE").expect("STATE_FILE is not set");

        // if the network we are connecting is a local kind, add it to the fn
        if network.network_type == ChainKind::Local.to_string() {
            let name = Path::new(&json_file_path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();
            let folder = Path::new(&json_file_path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap();

            json_file_path = format!("{folder}/{name}_local.json");
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
            json_file_path,
            kind: ChainKind::from(network.network_type),
            deployment_id: options
                .deployment_id
                .map(Into::into)
                .unwrap_or_else(|| DEFAULT_DEPLOYMENT.into()),
            grpc_channel,
            chain: ChainInfoOwned {
                network_id: network.chain_name.to_string(),
                pub_address_prefix: network.bech32_prefix,
                coin_type: network.slip44,
            },
            chain_id: network.chain_id.to_string(),
            gas_denom: Denom::from_str(&shortest_denom_token.denom).unwrap(),
            gas_price: shortest_denom_token.fixed_min_gas_price,
            lcd_url: None,
            fcd_url: None,
        };

        log::info!(
            "Writing daemon state JSON file: {:#?}",
            state.json_file_path
        );

        // write json state file
        crate::daemon::json_file::write(
            &state.json_file_path,
            &state.chain_id,
            &state.chain.network_id,
            &state.deployment_id,
        );

        // finish
        Ok(state)
    }

    pub fn set_deployment(&mut self, deployment_id: impl Into<String>) {
        self.deployment_id = deployment_id.into();
        crate::daemon::json_file::write(
            &self.json_file_path,
            &self.chain_id,
            &self.chain.network_id,
            &self.deployment_id,
        );
    }

    /// Get the state filepath and read it as json
    fn read_state(&self) -> serde_json::Value {
        crate::daemon::json_file::read(&self.json_file_path)
    }

    /// Retrieve a stateful value using the chainId and networkId
    fn get(&self, key: &str) -> Value {
        let json = self.read_state();
        json[&self.chain.network_id][&self.chain_id.to_string()][key].clone()
    }

    /// Set a stateful value using the chainId and networkId
    fn set<T: Serialize>(&self, key: &str, contract_id: &str, value: T) {
        let mut json = self.read_state();

        json[&self.chain.network_id][&self.chain_id.to_string()][key][contract_id] = json!(value);

        serde_json::to_writer_pretty(File::create(&self.json_file_path).unwrap(), &json).unwrap();
    }
}

impl StateInterface for Rc<DaemonState> {
    /// Read address for contract in deployment id from state file
    fn get_address(&self, contract_id: &str) -> Result<Addr, BootError> {
        let value = self
            .get(&self.deployment_id)
            .get(contract_id)
            .ok_or_else(|| BootError::AddrNotInStore(contract_id.to_owned()))?
            .clone();
        Ok(Addr::unchecked(value.as_str().unwrap()))
    }

    /// Set address for contract in deployment id in state file
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

    /// Get all addresses for deployment id from state file
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
impl Into<RegistryChainInfo> for ChainInfo<'_> {
    fn into(self) -> RegistryChainInfo {
        RegistryChainInfo {
            chain_name: self.chain_info.network_id.to_string(),
            chain_id: self.chain_id.to_string().into(),
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
    pub chain_info: NetworkInfo<'a>,
    pub kind: ChainKind,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct NetworkInfo<'a> {
    pub network_id: &'a str,
    /// address prefix
    pub pub_address_prefix: &'a str,
    /// coin type for key derivation
    pub coin_type: u32,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct ChainInfoOwned {
    pub network_id: String,
    /// address prefix
    pub pub_address_prefix: String,
    /// coin type for key derivation
    pub coin_type: u32,
}

impl From<NetworkInfo<'_>> for ChainInfoOwned {
    fn from(info: NetworkInfo<'_>) -> Self {
        Self {
            network_id: info.network_id.to_owned(),
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
    pub fn new() -> Result<Self, BootError> {
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
