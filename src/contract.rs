use std::{
    env,
    fs::{self, File},
    time::Duration,
};

use secp256k1::{Context, Signing};

use serde_json::{from_reader, json, Value};
use terra_rust_api::{
    client::{tx_types::TXResultSync, wasm::Wasm},
    core_types::Coin,
    messages::MsgExecuteContract,
    Message,
};

use crate::{
    error::TerraRustScriptError,
    multisig::Multisig,
    sender::{GroupConfig, Sender},
};
// https://doc.rust-lang.org/std/process/struct.Command.html
// RUSTFLAGS='-C link-arg=-s' cargo wasm

pub struct Interface<I, E, Q, M> {
    pub init_msg: Option<I>,
    pub execute_msg: Option<E>,
    pub query_msg: Option<Q>,
    pub migrate_msg: Option<M>,
}

impl<I, E, Q, M> Interface<I, E, Q, M> {}

impl<I, E, Q, M> Default for Interface<I, E, Q, M> {
    // Generates placeholder with type restrictions
    fn default() -> Self {
        Interface {
            init_msg: None,
            execute_msg: None,
            query_msg: None,
            migrate_msg: None,
        }
    }
}

pub struct ContractInstance<I, E, Q, M> {
    pub interface: Interface<I, E, Q, M>,
    pub group_config: GroupConfig,
    pub name: String,
}

impl<I: serde::Serialize, E: serde::Serialize, Q: serde::Serialize, M: serde::Serialize>
    ContractInstance<I, E, Q, M>
{
    pub async fn execute<C: Signing + Context>(
        &self,
        sender: &Sender<C>,
        exec_msg: E,
        coins: Vec<Coin>,
    ) -> Result<TXResultSync, TerraRustScriptError> {
        let execute_msg_json = json!(exec_msg);
        let contract = self.get_address()?;
        log::debug!("############{}#########", contract);
        let send: Message = if self.group_config.proposal {
            Multisig::create_proposal(
                execute_msg_json,
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
        Ok(resp)
    }

    pub async fn instantiate<C: Signing + Context>(
        &self,
        sender: &Sender<C>,
        init_msg: I,
        admin: Option<String>,
        coins: Vec<Coin>,
    ) -> Result<TXResultSync, TerraRustScriptError> {
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

        tokio::time::sleep(Duration::from_secs(6)).await;
        let result = sender.terra.tx().get(&resp.txhash).await?;

        let address =
            &result.get_attribute_from_result_logs("instantiate_contract", "contract_address")[0].1;
        log::debug!("{} address: {:?}", self.name, address);
        self.save_contract_address(address.clone())?;

        Ok(resp)
    }

    pub async fn upload<C: Signing + Context>(
        &self,
        sender: &Sender<C>,
        wasm_path: &str,
    ) -> Result<TXResultSync, TerraRustScriptError> {
        let wasm = Wasm::create(&sender.terra);
        let memo = format!("Contract: {}, Group: {}", self.name, self.group_config.name);

        let resp = wasm
            .store(&sender.secp, &sender.private_key, wasm_path, Some(memo))
            .await?;
        log::debug!("uploaded: {:?}", resp.txhash);
        // TODO: check why logs are empty
        tokio::time::sleep(Duration::from_secs(6)).await;
        let result = sender.terra.tx().get(&resp.txhash).await?;
        let code_id = result.get_attribute_from_result_logs("store_code", "code_id")[0]
            .1
            .parse::<u64>()?;
        log::debug!("code_id: {:?}", code_id);
        self.save_code_id(code_id)?;
        Ok(resp)
    }

    pub fn get_address(&self) -> Result<String, TerraRustScriptError> {
        self.group_config.get_contract_address(&self.name)
    }

    fn get_code_id(&self) -> Result<u64, TerraRustScriptError> {
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
    // pub fn execute(),
    // pub fn query(),
    // pub fn migrate(),
}

// #[async_trait]
// pub trait Interaction<
//     E: serde::Serialize + std::marker::Sync,
//     C: Signing + Context + std::marker::Sync,
// >
// {
//     async fn execute(
//         &self,
//         sender: &Sender<C>,
//         exec_msg: &E,
//         coins: Vec<Coin>,
//     ) -> Result<TXResultSync, TerraRustScriptError> {
//         let execute_msg_json = json!(exec_msg);
//         let contract = self.addresses()?;

//         let send: Message = MsgExecuteContract::create_from_value(
//             &sender.pub_addr()?,
//             &contract,
//             &execute_msg_json,
//             &coins,
//         )?;
//         // generate the transaction & calc fees
//         let messages: Vec<Message> = vec![send];
//         let (std_sign_msg, sigs) = sender
//             .terra
//             .generate_transaction_to_broadcast(&sender.secp, &sender.private_key, messages, None)
//             .await?;
//         // send it out
//         let resp = sender
//             .terra
//             .tx()
//             .broadcast_sync(&std_sign_msg, &sigs)
//             .await?;
//         match resp.code {
//             Some(code) => {
//                 log::error!("{}", serde_json::to_string(&resp)?);
//                 eprintln!("Transaction returned a {} {}", code, resp.txhash)
//             }
//             None => {
//                 println!("{}", resp.txhash)
//             }
//         };
//         Ok(resp)
//     }

// }
