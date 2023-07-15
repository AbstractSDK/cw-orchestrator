use std::{
    fmt::Debug,
    rc::Rc,
    str::FromStr,
};

use cosmos_client::client::Rpc;
use cosmrs::Denom;
use cosmwasm_std::Addr;
use serde::{de::DeserializeOwned, Serialize};

use crate::state::ChainState;

use super::{
    builder::WasmDaemonBuilder,
    error::WasmDaemonError,
    queriers::{WasmDaemonQuerier},
    state::WasmDaemonState,
};

#[derive(Clone)]
/**
    Represents a blockchain node.
    It's constructed using [`WasmDaemonBuilder`].

    ## Usage
    ```rust,no_run
    # tokio_test::block_on(async {
    use cw_orch::prelude::{DaemonAsync, networks};

    let daemon: DaemonAsync = DaemonAsync::builder()
        .chain(networks::JUNO_1)
        .build()
        .await.unwrap();
    # })
    ```
    ## Environment Execution

    The DaemonAsync implements [`TxHandler`](crate::prelude::TxHandler) which allows you to perform transactions on the chain.

    ## Querying

    Different Cosmos SDK modules can be queried through the daemon by calling the [`WasmDaemon::query_client<Querier>`] method with a specific querier.
    See [Querier](crate::daemon::queriers) for examples.
*/
pub struct WasmDaemon {
    /// State of the daemon
    pub state: Rc<WasmDaemonState>,
}

impl WasmDaemon {
    /// Get the daemon builder
    pub fn builder() -> WasmDaemonBuilder {
        WasmDaemonBuilder::default()
    }

    /// Get the channel configured for this DaemonAsync.
    pub async fn client(&self) -> Rpc {
        Rpc::new(self.state().rpc_url.as_str()).await.unwrap()
    }
}

impl ChainState for WasmDaemon {
    type Out = Rc<WasmDaemonState>;

    fn state(&self) -> Self::Out {
        self.state.clone()
    }
}


// Execute on the real chain, returns tx response.
impl WasmDaemon {
    /// Get the sender address
    // pub fn sender(&self) -> Addr {
    //     self.sender.address().unwrap()
    // }

    // /// Execute a message on a contract.
    // pub async fn execute<E: Serialize>(
    //     &self,
    //     exec_msg: &E,
    //     coins: &[cosmwasm_std::Coin],
    //     contract_address: &Addr,
    // ) -> Result<CosmTxResponse, WasmDaemonError> {
    //     let exec_msg: MsgExecuteContract = MsgExecuteContract {
    //         sender: self.sender.pub_addr()?,
    //         contract: AccountId::from_str(contract_address.as_str())?,
    //         msg: serde_json::to_vec(&exec_msg)?,
    //         funds: parse_cw_coins(coins)?,
    //     };
    //     let result = self.sender.commit_tx(vec![exec_msg], None).await?;
    //     self.client().
    //     Ok(result)
    // }
    //
    // /// Instantiate a contract.
    // pub async fn instantiate<I: Serialize + Debug>(
    //     &self,
    //     code_id: u64,
    //     init_msg: &I,
    //     label: Option<&str>,
    //     admin: Option<&Addr>,
    //     coins: &[Coin],
    // ) -> Result<CosmTxResponse, WasmDaemonError> {
    //     let sender = &self.sender;
    //
    //     let init_msg = MsgInstantiateContract {
    //         code_id,
    //         label: Some(label.unwrap_or("instantiate_contract").to_string()),
    //         admin: admin.map(|a| FromStr::from_str(a.as_str()).unwrap()),
    //         sender: sender.pub_addr()?,
    //         msg: serde_json::to_vec(&init_msg)?,
    //         funds: parse_cw_coins(coins)?,
    //     };
    //
    //     let result = sender.commit_tx(vec![init_msg], None).await?;
    //
    //     Ok(result)
    // }
    //
    // /// Query a contract.
    pub async fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, WasmDaemonError> {

        let resp = self.client().await.wasm.smart_contract_state(contract_address.as_str(), query_msg).await?;

        Ok(resp)
    }
    //
    // /// Migration a contract.
    // pub async fn migrate<M: Serialize + Debug>(
    //     &self,
    //     migrate_msg: &M,
    //     new_code_id: u64,
    //     contract_address: &Addr,
    // ) -> Result<CosmTxResponse, WasmDaemonError> {
    //     let exec_msg: MsgMigrateContract = MsgMigrateContract {
    //         sender: self.sender.pub_addr()?,
    //         contract: AccountId::from_str(contract_address.as_str())?,
    //         msg: serde_json::to_vec(&migrate_msg)?,
    //         code_id: new_code_id,
    //     };
    //     let result = self.sender.commit_tx(vec![exec_msg], None).await?;
    //     Ok(result)
    // }
    //
    // /// Wait for a given amount of blocks.
    // pub async fn wait_blocks(&self, amount: u64) -> Result<(), WasmDaemonError> {
    //     let mut last_height = self.query_client::<Node>().block_height().await?;
    //     let end_height = last_height + amount;
    //
    //     let average_block_speed = self
    //         .query_client::<Node>()
    //         .average_block_speed(Some(0.9))
    //         .await?;
    //
    //     let wait_time = average_block_speed * amount;
    //
    //     // now wait for that amount of time
    //     tokio::time::sleep(Duration::from_secs(wait_time)).await;
    //     // now check every block until we hit the target
    //     while last_height < end_height {
    //         // wait
    //
    //         tokio::time::sleep(Duration::from_secs(average_block_speed)).await;
    //
    //         // ping latest block
    //         last_height = self.query_client::<Node>().block_height().await?;
    //     }
    //     Ok(())
    // }
    //
    // /// Wait for a given amount of seconds.
    // pub async fn wait_seconds(&self, secs: u64) -> Result<(), WasmDaemonError> {
    //     tokio::time::sleep(Duration::from_secs(secs)).await;
    //
    //     Ok(())
    // }
    //
    // /// Wait for the next block.
    // pub async fn next_block(&self) -> Result<(), WasmDaemonError> {
    //     self.wait_blocks(1).await
    // }
    //
    // /// Get the current block info.
    // pub async fn block_info(&self) -> Result<cosmwasm_std::BlockInfo, WasmDaemonError> {
    //     let block = self.query_client::<Node>().latest_block().await?;
    //     let since_epoch = block.header.time.duration_since(Time::unix_epoch())?;
    //     let time = cosmwasm_std::Timestamp::from_nanos(since_epoch.as_nanos() as u64);
    //     Ok(cosmwasm_std::BlockInfo {
    //         height: block.header.height.value(),
    //         time,
    //         chain_id: block.header.chain_id.to_string(),
    //     })
    // }
    //
    // /// Upload a contract to the chain.
    // pub async fn upload(
    //     &self,
    //     uploadable: &impl Uploadable,
    // ) -> Result<CosmTxResponse, WasmDaemonError> {
    //     let sender = &self.sender;
    //     let wasm_path = uploadable.wasm();
    //
    //     log::debug!("Uploading file at {:?}", wasm_path);
    //
    //     let file_contents = std::fs::read(wasm_path.path())?;
    //     let store_msg = cosmrs::cosmwasm::MsgStoreCode {
    //         sender: sender.pub_addr()?,
    //         wasm_byte_code: file_contents,
    //         instantiate_permission: None,
    //     };
    //
    //     let result = sender.commit_tx(vec![store_msg], None).await?;
    //
    //     log::info!("Uploaded: {:?}", result.txhash);
    //
    //     let code_id = result.uploaded_code_id().unwrap();
    //
    //     // wait for the node to return the contract information for this upload
    //     let wasm = CosmWasm::new(self.client());
    //     while wasm.code(code_id).await.is_err() {
    //         self.next_block().await?;
    //     }
    //     Ok(result)
    // }
}

pub(crate) fn parse_cw_coins(
    coins: &[cosmwasm_std::Coin],
) -> Result<Vec<cosmrs::Coin>, WasmDaemonError> {
    coins
        .iter()
        .map(|cosmwasm_std::Coin { amount, denom }| {
            Ok(cosmrs::Coin {
                amount: amount.u128(),
                denom: Denom::from_str(denom)?,
            })
        })
        .collect::<Result<Vec<_>, WasmDaemonError>>()
}
