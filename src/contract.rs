use std::{
    env,
    fs::{self, File},
    rc::Rc,
    time::Duration,
};

use base64::decode;
use secp256k1::All;

use serde::Serialize;
use serde_json::{json, Value};
use terra_rust_api::{
    client::{tx_types::V1TXResult, wasm::Wasm},
    core_types::Coin,
    messages::MsgExecuteContract,
    Message,
};

use crate::{
    error::TerraRustScriptError,
    multisig::Multisig,
    sender::{GroupConfig, Sender, Wallet, self},
};
// https://doc.rust-lang.org/std/process/struct.Command.html
// RUSTFLAGS='-C link-arg=-s' cargo wasm

pub struct ContractInstance<'a> {
    pub group_config: &'a GroupConfig,
    pub name: &'a str,
    pub sender: Wallet<'a>,
}

impl<'a> ContractInstance<'a> {
    pub fn new(name: &'a str, sender: &'a Rc<sender::Sender<All>>, group_config: &'a GroupConfig) -> anyhow::Result<Self> {
        let instance = ContractInstance {
            group_config,
            name,
            sender,
        };
        instance.check_scaffold()?;
        Ok(instance)
    }

    pub async fn execute<E: Serialize>(
        &self,
        exec_msg: &E,
        coins: &Vec<Coin>,
    ) -> Result<V1TXResult, TerraRustScriptError> {
        let sender = &self.sender;
        let execute_msg_json = json!(exec_msg);
        let contract = self.get_address()?;
        log::debug!("############{}#########", contract);
        let send: Message = if self.group_config.proposal {
            Multisig::create_proposal(
                &execute_msg_json,
                &self.group_config.name,
                &contract,
                &env::var(&self.group_config.network_config.network.multisig_name())?,
                &sender.pub_addr()?,
                coins,
            )?
        } else {
            MsgExecuteContract::create_from_value(
                &sender.pub_addr()?,
                &contract,
                &execute_msg_json,
                &coins,
            )?
        };

        log::debug!("{}", serde_json::to_string(&send)?);

        // generate the transaction & calc fees
        let messages: Vec<Message> = vec![send];
        let (std_sign_msg, sigs) = sender
            .terra
            .generate_transaction_to_broadcast(&sender.secp, &sender.private_key, messages, None)
            .await?;
        // send it out
        let resp = sender
            .terra
            .tx()
            .broadcast_sync(&std_sign_msg, &sigs)
            .await?;
        match resp.code {
            Some(code) => {
                log::error!("{}", serde_json::to_string(&resp)?);
                eprintln!("Transaction returned a {} {}", code, resp.txhash)
            }
            None => {
                println!("{}", resp.txhash)
            }
        }
        let result = sender
            .terra
            .tx()
            .get_and_wait_v1(&resp.txhash, 15, Duration::from_secs(2))
            .await?;
        wait(&self.group_config).await;
        Ok(result)
    }

    pub async fn instantiate<I: Serialize>(
        &self,
        init_msg: I,
        admin: Option<String>,
        coins: Vec<Coin>,
    ) -> Result<V1TXResult, TerraRustScriptError> {
        let sender = &self.sender;
        let instantiate_msg_json = json!(init_msg);
        let code_id = self.get_code_id()?;

        let wasm = Wasm::create(&sender.terra);
        let memo = format!("Contract: {}, Group: {}", self.name, self.group_config.name);

        let resp = wasm
            .instantiate(
                &sender.secp,
                &sender.private_key,
                code_id,
                instantiate_msg_json.to_string(),
                coins,
                admin,
                Some(memo),
            )
            .await?;

        let result = sender
            .terra
            .tx()
            .get_and_wait_v1(&resp.txhash, 15, Duration::from_secs(2))
            .await?;

        let address = &result
            .tx_response
            .get_attribute_from_logs("instantiate_contract", "contract_address")[0]
            .1;
        log::debug!("{} address: {:?}", self.name, address);
        self.save_contract_address(address.clone())?;

        wait(&self.group_config).await;
        Ok(result)
    }

    pub async fn query<Q: Serialize>(&self, query_msg: Q) -> Result<Value, TerraRustScriptError> {
        let sender = &self.sender;
        let json_query = json!(query_msg);

        let wasm = Wasm::create(&sender.terra);
        let resp: Value = wasm
            .query(&self.get_address()?, &json_query.to_string())
            .await?;

        Ok(resp)
    }

    pub async fn upload(
        &self,
        name: &str,
        path: Option<&str>,
    ) -> Result<V1TXResult, TerraRustScriptError> {
        let sender = &self.sender;
        let wasm = Wasm::create(&sender.terra);
        let memo = format!("Contract: {}, Group: {}", self.name, self.group_config.name);
        let wasm_path = {
            match path {
                Some(path) => path.to_string(),
                None => format!(
                    "{}{}",
                    env::var("WASM_DIR").unwrap(),
                    format!("/{}.wasm", name)
                ),
            }
        };

        log::debug!("{}", &wasm_path);
        let resp = wasm
            .store(&sender.secp, &sender.private_key, &wasm_path, Some(memo))
            .await?;
        log::debug!("uploaded: {:?}", resp.txhash);
        // TODO: check why logs are empty

        let result = sender
            .terra
            .tx()
            .get_and_wait_v1(&resp.txhash, 15, Duration::from_secs(2))
            .await?;

        let code_id = result
            .tx_response
            .get_attribute_from_logs("store_code", "code_id")[0]
            .1
            .parse::<u64>()?;
        log::debug!("code_id: {:?}", code_id);
        self.save_code_id(code_id)?;
        wait(&self.group_config).await;
        Ok(result)
    }

    pub async fn migrate<M: Serialize>(
        &self,
        migrate_msg: M,
        new_code_id: u64,
    ) -> Result<V1TXResult, TerraRustScriptError> {
        let sender = &self.sender;
        let migrate_msg_json = json!(migrate_msg);

        let wasm = Wasm::create(&sender.terra);

        let old_code_id = wasm.info(&self.get_address()?).await?.result.code_id;
        let memo = format!("Contract: {}, OldCodeId: {}", self.name, old_code_id);

        let resp = wasm
            .migrate(
                &sender.secp,
                &sender.private_key,
                &self.get_address()?,
                new_code_id,
                Some(migrate_msg_json.to_string()),
                Some(memo),
            )
            .await?;

        let result = sender
            .terra
            .tx()
            .get_and_wait_v1(&resp.txhash, 15, Duration::from_secs(2))
            .await?;

        wait(&self.group_config).await;
        Ok(result)
    }

    pub fn get_address(&self) -> Result<String, TerraRustScriptError> {
        self.group_config.get_contract_address(&self.name)
    }

    pub fn get_code_id(&self) -> Result<u64, TerraRustScriptError> {
        self.group_config.get_contract_code_id(&self.name)
    }

    fn save_code_id(&self, code_id: u64) -> Result<(), TerraRustScriptError> {
        let s = fs::read_to_string(&self.group_config.file_path).unwrap();
        let mut cfg: Value = serde_json::from_str(&s)?;
        cfg[&self.group_config.name][&self.name]["code_id"] = Value::Number(code_id.into());

        serde_json::to_writer_pretty(File::create(&self.group_config.file_path)?, &cfg)?;
        Ok(())
    }

    pub fn save_contract_address(
        &self,
        contract_address: String,
    ) -> Result<(), TerraRustScriptError> {
        let s = fs::read_to_string(&self.group_config.file_path).unwrap();
        let mut cfg: Value = serde_json::from_str(&s)?;
        cfg[&self.group_config.name][&self.name]["addr"] = Value::String(contract_address);

        serde_json::to_writer_pretty(File::create(&self.group_config.file_path)?, &cfg)?;
        Ok(())
    }

    pub fn save_other_contract_address(
        &self,
        contract_name: String,
        contract_address: String,
    ) -> Result<(), TerraRustScriptError> {
        let s = fs::read_to_string(&self.group_config.file_path).unwrap();
        let mut cfg: Value = serde_json::from_str(&s)?;
        cfg[&self.group_config.name][&contract_name]["addr"] = Value::String(contract_address);

        serde_json::to_writer_pretty(File::create(&self.group_config.file_path)?, &cfg)?;
        Ok(())
    }

    pub fn check_scaffold(&self) -> anyhow::Result<()> {
        let s = fs::read_to_string(&self.group_config.file_path)?;
        let mut cfg: Value = serde_json::from_str(&s)?;

        if cfg[&self.group_config.name].get(&self.name).is_none() {
            let scaffold = json!({});
            cfg[&self.group_config.name][&self.name] = scaffold;
            serde_json::to_writer_pretty(File::create(&self.group_config.file_path)?, &cfg)?;
        }

        Ok(())
    }

    pub async fn is_local_version(&self) -> anyhow::Result<bool> {
        let on_chain_encoded_hash = self
            .sender
            .terra
            .wasm()
            .codes(self.get_code_id()?)
            .await?
            .result
            .code_hash;
        let path = format!("{}/checksums.txt", env::var("WASM_DIR")?);

        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

        let parsed: Vec<&str> = contents.rsplit(".wasm").collect();

        let name = self.name.split(':').last().unwrap();

        let containing_line = parsed
            .iter()
            .filter(|line| line.contains(name))
            .next()
            .unwrap();
        log::debug!("{:?}", containing_line);

        let local_hash = containing_line
            .trim_start_matches('\n')
            .split_whitespace()
            .next()
            .unwrap();

        let on_chain_hash = base16::encode_lower(&decode(on_chain_encoded_hash)?);
        Ok(on_chain_hash == local_hash)
    }
}

async fn wait(groupconfig: &GroupConfig) {
    match groupconfig.network_config.network {
        crate::sender::Network::LocalTerra => tokio::time::sleep(Duration::from_secs(6)).await,
        crate::sender::Network::Mainnet => tokio::time::sleep(Duration::from_secs(60)).await,
        crate::sender::Network::Testnet => tokio::time::sleep(Duration::from_secs(30)).await,
    }
}
