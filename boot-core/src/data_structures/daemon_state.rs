use crate::error::BootError;
use crate::state::StateInterface;
use cosmrs::Denom;
use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, json};
use std::{collections::HashMap, env, fs::File, rc::Rc, str::FromStr};
use tonic::transport::Channel;

#[derive(Clone, Debug)]
pub struct DaemonState {
    pub json_file_path: String,
    /// What kind of network
    pub kind: NetworkKind,
    /// Identifier for the network ex. columbus-2
    pub id: String,
    /// gRPC channel
    pub grpc_channel: Channel,
    /// Underlying chain details
    pub chain: ChainInfo<'static>,
    /// Max gas and denom info
    pub gas_denom: Denom,
    /// gas price
    pub gas_price: f64,
    /// Optional urls for custom functionality
    pub lcd_url: Option<String>,
    pub fcd_url: Option<String>,
}

impl DaemonState {
    pub async fn new(network: NetworkInfo<'static>) -> Result<DaemonState, BootError> {
        let grpc_channel = Channel::from_static(network.grpc_url).connect().await?;
        let mut path = env::var("DAEMON_STATE_PATH").unwrap();
        if network.kind == NetworkKind::Local {
            let name = path.split('.').next().unwrap();
            path = format!("{}_local.json", name);
        }

        let state = DaemonState {
            json_file_path: path,
            kind: network.kind,
            grpc_channel,
            chain: network.chain_info,
            id: network.id.to_string(),
            gas_denom: Denom::from_str(network.gas_denom).unwrap(),
            gas_price: network.gas_price,
            lcd_url: network.lcd_url.map(|url| url.to_string()),
            fcd_url: network.fcd_url.map(|url| url.to_string()),
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
        if json.get(self.chain.chain_id).is_none() {
            json[self.chain.chain_id] = json!({});
        }
        if json[self.chain.chain_id].get(&self.id).is_none() {
            json[self.chain.chain_id][&self.id] = json!({
                "addresses": {},
                "code_ids": {}
            });
        }
        serde_json::to_writer_pretty(File::create(&self.json_file_path).unwrap(), &json).unwrap();
    }
}

impl StateInterface for Rc<DaemonState> {
    fn get_address(&self, contract_id: &str) -> Result<Addr, BootError> {
        let file = File::open(&self.json_file_path)
            .unwrap_or_else(|_| panic!("file should be present at {}", self.json_file_path));
        let json: serde_json::Value = from_reader(file)?;
        let value = json[&self.chain.chain_id][&self.id.to_string()]["addresses"]
            .get(contract_id)
            .ok_or_else(|| BootError::AddrNotInFile(contract_id.to_owned()))?
            .clone();
        Ok(Addr::unchecked(value.as_str().unwrap()))
    }

    fn set_address(&mut self, contract_id: &str, address: &Addr) {
        let file = File::open(&self.json_file_path)
            .unwrap_or_else(|_| panic!("file should be present at {}", self.json_file_path));
        let mut json: serde_json::Value = from_reader(file).unwrap();

        json[&self.chain.chain_id][&self.id.to_string()]["addresses"][contract_id] =
            json!(address.as_str());
        serde_json::to_writer_pretty(File::create(&self.json_file_path).unwrap(), &json).unwrap();
    }

    /// Get the locally-saved version of the contract's version on this network
    fn get_code_id(&self, contract_id: &str) -> Result<u64, BootError> {
        let file = File::open(&self.json_file_path)
            .unwrap_or_else(|_| panic!("file should be present at {}", self.json_file_path));
        let json: serde_json::Value = from_reader(file)?;
        let value = json[&self.chain.chain_id][&self.id.to_string()]["code_ids"]
            .get(contract_id)
            .ok_or_else(|| BootError::CodeIdNotInFile(contract_id.to_owned()))?
            .clone();
        Ok(value.as_u64().unwrap())
    }

    /// Set the locally-saved version of the contract's latest version on this network
    fn set_code_id(&mut self, contract_id: &str, code_id: u64) {
        let file = File::open(&self.json_file_path)
            .unwrap_or_else(|_| panic!("file should be present at {}", self.json_file_path));
        let mut json: serde_json::Value = from_reader(file).unwrap();

        json[&self.chain.chain_id][&self.id.to_string()]["code_ids"][contract_id] = json!(code_id);
        serde_json::to_writer_pretty(File::create(&self.json_file_path).unwrap(), &json).unwrap();
    }
    fn get_all_addresses(&self) -> Result<HashMap<String, Addr>, BootError> {
        let mut store = HashMap::new();
        let file = File::open(&self.json_file_path)
            .unwrap_or_else(|_| panic!("file should be present at {}", self.json_file_path));
        let json: serde_json::Value = from_reader(file).unwrap();
        let value = json[&self.chain.chain_id][&self.id.to_string()]["addresses"]
            .as_object()
            .unwrap();
        for (id, addr) in value {
            store.insert(id.clone(), Addr::unchecked(addr.as_str().unwrap()));
        }
        Ok(store)
    }
    fn get_all_code_ids(&self) -> Result<HashMap<String, u64>, BootError> {
        let mut store = HashMap::new();
        let file = File::open(&self.json_file_path)
            .unwrap_or_else(|_| panic!("file should be present at {}", self.json_file_path));
        let json: serde_json::Value = from_reader(file).unwrap();
        let value = json[&self.chain.chain_id][&self.id.to_string()]["code_ids"]
            .as_object()
            .unwrap();
        for (id, code_id) in value {
            store.insert(id.clone(), code_id.as_u64().unwrap());
        }
        Ok(store)
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
    pub grpc_url: &'a str,
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
