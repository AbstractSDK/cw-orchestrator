use cosmos_sdk_proto::{cosmos::{auth::v1beta1::BaseAccount, bank::v1beta1::MsgSend}, tendermint::Protobuf};
use cosmrs::{crypto::secp256k1::SigningKey, AccountId, tx::Gas, rpc::Client, Any};
use prost::Message;
use secp256k1::{All, Context, Secp256k1, Signing};
use serde_json::{from_reader, json, Map, Value};
use std::{env, fs::File, rc::Rc};

use crate::{error::TerraRustScriptError, chain::Chain, keys::private::PrivateKey};

pub type Wallet<'a> = &'a Rc<Sender<All>>;

pub struct Sender<C: Signing + Context> {
    pub chain: Chain,
    pub private_key: SigningKey,
    pub secp: Secp256k1<C>,
}

impl<C: Signing + Context> Sender<C> {
    pub fn pub_addr(&self) -> Result<AccountId, TerraRustScriptError> {
        Ok(self.private_key.public_key().account_id(&self.chain.pub_addr_prefix)?)
    }

    pub fn new(
        config: &GroupConfig,
        chain: Chain,
        secp: Secp256k1<C>,
    ) -> Result<Sender<C>, TerraRustScriptError> {
        // NETWORK_MNEMONIC_GROUP
        let mut composite_name = config.network_config.network.mnemonic_name().to_string();
        composite_name.push('_');
        composite_name.push_str(&config.name.to_ascii_uppercase());

        // use group mnemonic if specified, else use default network mnemonic
        let p_key: PrivateKey = if let Some(mnemonic) = env::var_os(&composite_name) {
            PrivateKey::from_words(&secp, mnemonic.to_str().unwrap(), 0, 0,chain.coin_type)?
        } else {
            log::debug!("{}", config.network_config.network.mnemonic_name());
            let mnemonic = env::var(config.network_config.network.mnemonic_name())?;
            PrivateKey::from_words(&secp, &mnemonic, 0, 0, chain.coin_type)?
        };

        let cosmos_private_key = SigningKey::from_bytes(&p_key.raw_key()).unwrap();

        Ok(Sender {
            chain,
            private_key: cosmos_private_key,
            secp,
        })
    }

    pub async fn sequence_number(&self) -> Result<u64, TerraRustScriptError> {
        // SimulateRequest for gas

        // Auth query client
        let addr = self.pub_addr().unwrap().to_string();

        let mut client = cosmos_sdk_proto::cosmos::auth::v1beta1::query_client::QueryClient::new(self.chain.rpc_channel.clone());

        let resp = client.account(cosmos_sdk_proto::cosmos::auth::v1beta1::QueryAccountRequest{
            address: addr
        }).await?.into_inner();
        
        let acc: BaseAccount = BaseAccount::decode(resp.account.unwrap().value.as_ref()).unwrap();
        Ok(acc.sequence)
    } 
}

#[derive(Clone, Debug)]
pub struct GroupConfig {
    pub network_config: NetworkConfig,
    pub name: String,
    pub file_path: String,
    pub proposal: bool,
}

impl GroupConfig {
    pub async fn new(
        network: Network,
        name: String,
        client: reqwest::Client,
        denom: &str,
        file_path: String,
        proposal: bool,
    ) -> anyhow::Result<GroupConfig> {
        check_group_existance(&name, &file_path)?;

        Ok(GroupConfig {
            network_config: network.config(client, denom).await?,
            name,
            file_path,
            proposal,
        })
    }

    pub fn get_contract_address(
        &self,
        contract_name: &str,
    ) -> Result<String, TerraRustScriptError> {
        let file = File::open(&self.file_path)
            .expect(&format!("file should be present at {}", self.file_path));
        let json: serde_json::Value = from_reader(file)?;
        let maybe_address = json[self.name.clone()][contract_name].get("addr");
        match maybe_address {
            Some(addr) => Ok(addr.as_str().unwrap().into()),
            None => Err(TerraRustScriptError::AddrNotInFile(
                contract_name.to_owned(),
            )),
        }
    }

    pub fn get_contract_code_id(&self, contract_name: &str) -> Result<u64, TerraRustScriptError> {
        let file = File::open(&self.file_path)
            .expect(&format!("file should be present at {}", self.file_path));
        let json: serde_json::Value = from_reader(file).unwrap();
        let maybe_code_id = json[self.name.clone()][contract_name].get("code_id");
        match maybe_code_id {
            Some(code_id) => Ok(code_id.as_u64().unwrap()),
            None => Err(TerraRustScriptError::AddrNotInFile(
                contract_name.to_owned(),
            )),
        }
    }

    pub fn get_saved_state(&self) -> Map<String, Value> {
        let file = File::open(&self.file_path)
            .expect(&format!("file should be present at {}", self.file_path));
        let json: serde_json::Value = from_reader(file).unwrap();
        json.get(&self.name).unwrap().as_object().unwrap().clone()
    }
}

fn check_group_existance(name: &String, file_path: &String) -> anyhow::Result<()> {
    let file = File::open(file_path).expect(&format!("file should be present at {}", file_path));
    let mut cfg: serde_json::Value = from_reader(file).unwrap();
    let maybe_group = cfg.get(name);
    match maybe_group {
        Some(_) => Ok(()),
        None => {
            cfg[name] = json!({});
            serde_json::to_writer_pretty(File::create(file_path)?, &cfg)?;
            Ok(())
        }
    }
}
