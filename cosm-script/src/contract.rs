use std::{
    env,
    str::{from_utf8, FromStr},
    time::Duration, cell::RefCell, marker::PhantomData, rc::Rc,
};

use cosmrs::{
    cosmwasm::{MsgExecuteContract, MsgInstantiateContract, MsgMigrateContract},
    AccountId, Coin,
};

use cosmwasm_std::Empty;
use cw_multi_test::Contract as TestContract;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::from_str;

use crate::{
    cosmos_modules, data_structures::network::NetworkKind, error::CosmScriptError,
    multisig::Multisig, sender::Wallet, CosmTxResponse, Deployment, tx_handler::{TxHandler, TxResponse}, index_response::IndexResponse, state::StateInterface,
};

pub type StateReference = Rc<RefCell<dyn StateInterface>>;
/// An instance of a contract. Contains references to the execution environment (chain) and a local state (state)
/// The state is used to store contract addresses/code-ids
struct Contract<Chain: TxHandler, E: Serialize, I: Serialize, Q: Serialize, M: Serialize>
where
    TxResponse<Chain>: IndexResponse,
{
    /// Name of the contract, used to retrieve addr/code-id
    pub name: String,
    /// chain object that handles tx execution and queries.
    chain: Chain,
    /// A pointer to a state object to store contract addresses/code-ids
    /// Indicate the type of executemsg
    _execute_msg: PhantomData<E>,
    _instantiate_msg: PhantomData<I>,
    _query_msg: PhantomData<Q>,
    _migrate_msg: PhantomData<M>,
    state: StateReference,
}

pub enum ContractCodeReference{
    WasmCodePath(String),
    ContractEndpoints(Box< dyn TestContract<Empty>>)
}


/// Expose chain and state function to call them on the contract
impl<Chain: TxHandler, E: Serialize, I: Serialize, Q: Serialize, M: Serialize> Contract<Chain, E, I, Q, M>
where
TxResponse<Chain>: IndexResponse,
{
    fn new(name: &str, chain: Chain, state: StateReference) -> Self {
        Contract {
            name: name.to_string(),
            chain,
            state,
            _execute_msg: PhantomData,
            _instantiate_msg: PhantomData,
            _query_msg: PhantomData,
            _migrate_msg: PhantomData, 
        }
    }
    // fn execute(&self, msg: &E) -> TxResponse<Chain> {
    //     self.chain.execute(msg)
    // }
    // fn address(&self) -> String {
    //     self.state.get_addr(&self.name)
    // }
    // fn save_address(&self, address: &str) {
    //     self.state.save_address(address)
    // }
}

// pub struct ContractInstance<'a> {
//     pub deployment: &'a Deployment,
//     pub name: &'a str,
//     pub sender: Wallet<'a>,
//     /// Allows for setting a custom code-id in the Instance trait implemtation
//     code_id_key: Option<&'a str>,
// }
// impl<'a> ContractInstance<'a> {
//     pub fn new(
//         name: &'a str,
//         sender: Wallet<'a>,
//         deployment: &'a Deployment,
//     ) -> anyhow::Result<Self> {
//         let instance = ContractInstance {
//             deployment,
//             name,
//             sender,
//             code_id_key: None,
//         };
//         Ok(instance)
//     }

//     /// Used to overwrite the code-id getter key. Useful when you want shared code between multiple contract instances
//     /// Example: Two CW20 tokens that use the same code-id but have a different name. see
//     pub fn overwrite_code_id_key(&mut self, code_id_key_to_use: &'static str) {
//         self.code_id_key = Some(code_id_key_to_use);
//     }

//     pub async fn execute<E: Serialize>(
//         &self,
//         exec_msg: &E,
//         coins: &[Coin],
//     ) -> Result<CosmTxResponse, CosmScriptError> {
//         let contract = self.get_address()?;
//         log::info!("executing on {} at {}", self.name, contract);

//         let exec_msg: MsgExecuteContract = if self.deployment.proposal {
//             Multisig::create_proposal(
//                 &exec_msg,
//                 &self.deployment.name,
//                 &contract,
//                 &env::var(&self.deployment.network.kind.multisig_name())?,
//                 self.sender.pub_addr()?,
//                 coins,
//             )?
//         } else {
//             MsgExecuteContract {
//                 sender: self.sender.pub_addr()?,
//                 contract: AccountId::from_str(&self.get_address()?)?,
//                 msg: serde_json::to_vec(&exec_msg)?,
//                 funds: coins.to_vec(),
//             }
//         };

//         let result = self.sender.commit_tx(vec![exec_msg], None).await?;

//         Ok(result)
//     }

//     pub async fn instantiate<I: Serialize>(
//         &self,
//         init_msg: I,
//         admin: Option<String>,
//         coins: &[Coin],
//     ) -> Result<CosmTxResponse, CosmScriptError> {
//         let sender = self.sender;
//         let key = self.code_id_key.unwrap_or(self.name);
//         let code_id = self.deployment.network.get_latest_version(key)?;

//         let memo = format!("Contract: {}, Group: {}", self.name, self.deployment.name);

//         log::info!("instantiating {}", self.name);

//         let init_msg = MsgInstantiateContract {
//             code_id,
//             label: Some(self.name.into()),
//             admin: admin.map(|a| FromStr::from_str(&a).unwrap()),
//             sender: sender.pub_addr()?,
//             msg: serde_json::to_vec(&init_msg)?,
//             funds: coins.to_vec(),
//         };

//         let result = sender.commit_tx(vec![init_msg], Some(&memo)).await?;
//         let address = &result.get_attribute_from_logs("instantiate", "_contract_address")[0].1;

//         log::debug!("{} address: {:?}", self.name, address);
//         self.save_contract_address(address)?;

//         Ok(result)
//     }

//     pub async fn query<Q: Serialize, T: Serialize + DeserializeOwned>(
//         &self,
//         query_msg: Q,
//     ) -> Result<T, CosmScriptError> {
//         let sender = self.sender;

//         let mut client = cosmos_modules::cosmwasm::query_client::QueryClient::new(sender.channel());
//         let resp = client
//             .smart_contract_state(cosmos_modules::cosmwasm::QuerySmartContractStateRequest {
//                 address: self.get_address()?,
//                 query_data: serde_json::to_vec(&query_msg)?,
//             })
//             .await?;

//         Ok(from_str(from_utf8(&resp.into_inner().data).unwrap())?)
//     }

//     /// Uploads given .wasm file and stores resulting code-id in contract store.
//     /// *path* can be either a full/relative path. (indicated by the .wasm) or just a regular name. In the second case the WASM_DIR env var
//     /// will be read and the path will be costructed to be WASM_DIR/*path*.wasm
//     pub async fn upload(&self, path: &str) -> Result<CosmTxResponse, CosmScriptError> {
//         let sender = &self.sender;
//         let memo = format!("Contract: {}, Group: {}", self.name, self.deployment.name);
//         let wasm_path = if path.contains(".wasm") {
//             path.to_string()
//         } else {
//             format!("{}/{}.wasm", env::var("WASM_DIR").unwrap(), path)
//         };

//         log::debug!("{}", wasm_path);

//         let file_contents = std::fs::read(wasm_path)?;
//         let store_msg = cosmrs::cosmwasm::MsgStoreCode {
//             sender: sender.pub_addr()?,
//             wasm_byte_code: file_contents,
//             instantiate_permission: None,
//         };
//         let result = sender.commit_tx(vec![store_msg], Some(&memo)).await?;

//         log::info!("uploaded: {:?}", result.txhash);
//         // TODO: check why logs are empty

//         let code_id = result.get_attribute_from_logs("store_code", "code_id")[0]
//             .1
//             .parse::<u64>()?;
//         log::info!("code_id: {:?}", code_id);
//         self.save_code_id(code_id)?;
//         wait(self.deployment).await;
//         Ok(result)
//     }

//     pub async fn migrate<M: Serialize>(
//         &self,
//         migrate_msg: M,
//         new_code_id: u64,
//     ) -> Result<CosmTxResponse, CosmScriptError> {

//         let contract = self.get_address()?;
//         log::info!("executing on {} at {}", self.name, contract);

//         let exec_msg: MsgMigrateContract =
//         MsgMigrateContract {
//                 sender: self.sender.pub_addr()?,
//                 contract: AccountId::from_str(&self.get_address()?)?,
//                 msg: serde_json::to_vec(&migrate_msg)?,
//                 code_id: new_code_id
            
//         };

//         let result = self.sender.commit_tx(vec![exec_msg], None).await?;

//         Ok(result)
//     }

//     // Getters //

//     pub fn get_address(&self) -> Result<String, CosmScriptError> {
//         self.deployment.get_contract_address(self.name)
//     }

//     /// get the on-chain contract code-id
//     pub async fn get_code_id(&self) -> Result<u64, CosmScriptError> {
//         let addr = self.get_address()?;
//         let channel = self.deployment.network.grpc_channel.clone();
//         let mut client = cosmos_modules::cosmwasm::query_client::QueryClient::new(channel);

//         let resp = client
//             .contract_info(cosmos_modules::cosmwasm::QueryContractInfoRequest { address: addr })
//             .await?
//             .into_inner();

//         let code_id = resp.contract_info.unwrap().code_id;
//         Ok(code_id)
//     }

//     // Setters //

//     pub fn save_code_id(&self, code_id: u64) -> Result<(), CosmScriptError> {
//         self.deployment
//             .network
//             .set_contract_version(self.name, code_id)
//     }

//     pub fn save_contract_address(&self, contract_address: &str) -> Result<(), CosmScriptError> {
//         self.deployment
//             .save_contract_address(self.name, contract_address)
//     }

//     pub async fn is_local_version(&self) -> anyhow::Result<bool> {
//         todo!()

//         // let on_chain_encoded_hash = self
//         //     .sender
//         //     .terra
//         //     .wasm()
//         //     .codes(self.get_code_id()?)
//         //     .await?
//         //     .result
//         //     .code_hash;
//         // let path = format!("{}/checksums.txt", env::var("WASM_DIR")?);

//         // let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

//         // let parsed: Vec<&str> = contents.rsplit(".wasm").collect();

//         // let name = self.name.split(':').last().unwrap();

//         // let containing_line = parsed
//         //     .iter()
//         //     .filter(|line| line.contains(name))
//         //     .next()
//         //     .unwrap();
//         // log::debug!("{:?}", containing_line);

//         // let local_hash = containing_line
//         //     .trim_start_matches('\n')
//         //     .split_whitespace()
//         //     .next()
//         //     .unwrap();

//         // let on_chain_hash = base16::encode_lower(&decode(on_chain_encoded_hash)?);
//         // Ok(on_chain_hash == local_hash)
//     }
// }

// async fn wait(deployment: &Deployment) {
//     match deployment.network.kind {
//         NetworkKind::Local => tokio::time::sleep(Duration::from_secs(6)).await,
//         NetworkKind::Mainnet => tokio::time::sleep(Duration::from_secs(60)).await,
//         NetworkKind::Testnet => tokio::time::sleep(Duration::from_secs(30)).await,
//     }
// }