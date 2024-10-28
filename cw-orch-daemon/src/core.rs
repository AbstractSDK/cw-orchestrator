use super::{
    cosmos_modules, error::DaemonError, queriers::Node, senders::Wallet, tx_resp::CosmTxResponse,
};
use crate::{
    queriers::CosmWasm,
    senders::{builder::SenderBuilder, query::QuerySender, tx::TxSender},
    DaemonAsyncBuilder, DaemonState,
};
use cosmrs::{
    cosmwasm::{MsgExecuteContract, MsgInstantiateContract, MsgMigrateContract},
    proto::cosmwasm::wasm::v1::MsgInstantiateContract2,
    tendermint::Time,
    AccountId, Any, Denom,
};
use cosmwasm_std::{Addr, Binary, Coin};
use cw_orch_core::{
    contract::{interface_traits::Uploadable, WasmPath},
    environment::{
        AccessConfig, AsyncWasmQuerier, ChainInfoOwned, ChainState, IndexResponse, Querier,
    },
    log::transaction_target,
};
use flate2::{write, Compression};
use prost::Message;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::from_str;
use std::{
    fmt::Debug,
    io::Write,
    ops::Deref,
    str::{from_utf8, FromStr},
    time::Duration,
};
use tonic::transport::Channel;

pub const INSTANTIATE_2_TYPE_URL: &str = "/cosmwasm.wasm.v1.MsgInstantiateContract2";

#[derive(Clone)]
/**
    Represents a blockchain node.
    It's constructed using [`DaemonAsyncBuilder`].

    ## Usage
    ```rust,no_run
    # tokio_test::block_on(async {
    use cw_orch_daemon::{DaemonAsync, networks};

    let daemon: DaemonAsync = DaemonAsync::builder(networks::JUNO_1)
        .build()
        .await.unwrap();
    # })
    ```
    ## Environment Execution

    The DaemonAsync implements async methods of [`TxHandler`](cw_orch_core::environment::TxHandler) which allows you to perform transactions on the chain.

    ## Querying

    Different Cosmos SDK modules can be queried through the daemon by calling the [`DaemonAsync::query_client<Querier>`] method with a specific querier.
    See [Querier](crate::queriers) for examples.

    ## Warning

    This daemon is thread safe and can be used between threads.
    However, please make sure that you are not trying to broadcast multiple transactions at once when using this Daemon on different threads.
    If you do so, you will get account sequence errors and your transactions won't get broadcasted.
*/
pub struct DaemonAsyncBase<Sender = Wallet> {
    /// Sender to send transactions to the chain
    sender: Sender,
    /// State of the daemon
    pub(crate) state: DaemonState,
}

pub type DaemonAsync = DaemonAsyncBase<Wallet>;

impl<Sender> DaemonAsyncBase<Sender> {
    pub(crate) fn new(sender: Sender, state: DaemonState) -> Self {
        Self { sender, state }
    }

    pub fn chain_info(&self) -> &ChainInfoOwned {
        self.state.chain_data.as_ref()
    }

    /// Get the daemon builder
    pub fn builder(chain: impl Into<ChainInfoOwned>) -> DaemonAsyncBuilder {
        DaemonAsyncBuilder::new(chain)
    }

    /// Set the Sender for use with this Daemon
    /// The sender will be configured with the chain's data.
    pub async fn new_sender<T: SenderBuilder>(
        self,
        sender_options: T,
    ) -> DaemonAsyncBase<T::Sender> {
        let sender = sender_options
            .build(&self.state.chain_data)
            .await
            .expect("Failed to build sender");
        DaemonAsyncBase {
            sender,
            state: self.state,
        }
    }

    /// Get a mutable Sender
    pub fn sender_mut(&mut self) -> &mut Sender {
        &mut self.sender
    }

    // Get a read-only Sender
    pub fn sender(&self) -> &Sender {
        &self.sender
    }

    /// Flushes all the state related to the current chain
    /// Only works on Local networks
    pub fn flush_state(&mut self) -> Result<(), DaemonError> {
        self.state.flush()
    }

    /// Returns a new [`DaemonAsyncBuilder`] with the current configuration.
    /// Does not consume the original [`DaemonAsync`].
    pub fn rebuild(&self) -> DaemonAsyncBuilder {
        DaemonAsyncBuilder {
            state: Some(self.state()),
            chain: self.state.chain_data.deref().clone(),
            deployment_id: Some(self.state.deployment_id.clone()),
            state_path: None,
            write_on_change: None,
            mnemonic: None,
            // If it was test it will just use same tempfile as state
            is_test: false,
            // Uses same ChainInfo
            load_network: false,
        }
    }
}

impl<Sender: QuerySender> DaemonAsyncBase<Sender> {
    /// Get the channel configured for this DaemonAsync.
    pub fn channel(&self) -> Channel {
        self.sender().channel()
    }

    /// Query a contract.
    pub async fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, DaemonError> {
        let mut client = cosmos_modules::cosmwasm::query_client::QueryClient::new(self.channel());
        let resp = client
            .smart_contract_state(cosmos_modules::cosmwasm::QuerySmartContractStateRequest {
                address: contract_address.to_string(),
                query_data: serde_json::to_vec(&query_msg)?,
            })
            .await?;

        Ok(from_str(from_utf8(&resp.into_inner().data).unwrap())?)
    }

    /// Wait for a given amount of blocks.
    pub async fn wait_blocks(&self, amount: u64) -> Result<(), DaemonError> {
        let mut last_height = Node::new_async(self.channel())._block_height().await?;
        let end_height = last_height + amount;

        let average_block_speed = Node::new_async(self.channel())
            ._average_block_speed(Some(0.9))
            .await?;

        let wait_time = average_block_speed.mul_f64(amount as f64);

        // now wait for that amount of time
        async_std::task::sleep(wait_time).await;
        // now check every block until we hit the target
        while last_height < end_height {
            // wait

            async_std::task::sleep(average_block_speed).await;

            // ping latest block
            last_height = Node::new_async(self.channel())._block_height().await?;
        }
        Ok(())
    }

    /// Wait for a given amount of seconds.
    pub async fn wait_seconds(&self, secs: u64) -> Result<(), DaemonError> {
        async_std::task::sleep(Duration::from_secs(secs)).await;

        Ok(())
    }

    /// Wait for the next block.
    pub async fn next_block(&self) -> Result<(), DaemonError> {
        self.wait_blocks(1).await
    }

    /// Get the current block info.
    pub async fn block_info(&self) -> Result<cosmwasm_std::BlockInfo, DaemonError> {
        let block = Node::new_async(self.channel())._latest_block().await?;
        let since_epoch = block.header.time.duration_since(Time::unix_epoch())?;
        let time = cosmwasm_std::Timestamp::from_nanos(since_epoch.as_nanos() as u64);
        Ok(cosmwasm_std::BlockInfo {
            height: block.header.height.value(),
            time,
            chain_id: block.header.chain_id.to_string(),
        })
    }
}

impl<Sender> ChainState for DaemonAsyncBase<Sender> {
    type Out = DaemonState;

    fn state(&self) -> Self::Out {
        self.state.clone()
    }
}

// Execute on the real chain, returns tx response.
impl<Sender: TxSender> DaemonAsyncBase<Sender> {
    /// Get the sender address
    pub fn sender_addr(&self) -> Addr {
        self.sender().address()
    }

    /// Execute a message on a contract.
    pub async fn execute<E: Serialize>(
        &self,
        exec_msg: &E,
        coins: &[cosmwasm_std::Coin],
        contract_address: &Addr,
    ) -> Result<CosmTxResponse, DaemonError> {
        let exec_msg: MsgExecuteContract = MsgExecuteContract {
            sender: self.sender().msg_sender().map_err(Into::into)?,
            contract: AccountId::from_str(contract_address.as_str())?,
            msg: serde_json::to_vec(&exec_msg)?,
            funds: parse_cw_coins(coins)?,
        };
        let result = self
            .sender()
            .commit_tx(vec![exec_msg], None)
            .await
            .map_err(Into::into)?;
        log::info!(target: &transaction_target(), "Execution done: {:?}", result.txhash);

        Ok(result)
    }

    /// Instantiate a contract.
    pub async fn instantiate<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[Coin],
    ) -> Result<CosmTxResponse, DaemonError> {
        let init_msg = MsgInstantiateContract {
            code_id,
            label: Some(label.unwrap_or("instantiate_contract").to_string()),
            admin: admin.map(|a| FromStr::from_str(a.as_str()).unwrap()),
            sender: self.sender().msg_sender().map_err(Into::into)?,
            msg: serde_json::to_vec(&init_msg)?,
            funds: parse_cw_coins(coins)?,
        };

        let result = self
            .sender()
            .commit_tx(vec![init_msg], None)
            .await
            .map_err(Into::into)?;

        log::info!(target: &transaction_target(), "Instantiation done: {:?}", result.txhash);

        Ok(result)
    }

    /// Instantiate a contract.
    pub async fn instantiate2<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[Coin],
        salt: Binary,
    ) -> Result<CosmTxResponse, DaemonError> {
        let init_msg = MsgInstantiateContract2 {
            code_id,
            label: label.unwrap_or("instantiate_contract").to_string(),
            admin: admin.map(Into::into).unwrap_or_default(),
            sender: self.sender_addr().to_string(),
            msg: serde_json::to_vec(&init_msg)?,
            funds: proto_parse_cw_coins(coins)?,
            salt: salt.to_vec(),
            fix_msg: false,
        };

        let result = self
            .sender()
            .commit_tx_any(
                vec![Any {
                    type_url: INSTANTIATE_2_TYPE_URL.to_string(),
                    value: init_msg.encode_to_vec(),
                }],
                None,
            )
            .await
            .map_err(Into::into)?;

        log::info!(target: &transaction_target(), "Instantiation done: {:?}", result.txhash);

        Ok(result)
    }

    /// Migration a contract.
    pub async fn migrate<M: Serialize + Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
        contract_address: &Addr,
    ) -> Result<CosmTxResponse, DaemonError> {
        let exec_msg: MsgMigrateContract = MsgMigrateContract {
            sender: self.sender().msg_sender().map_err(Into::into)?,
            contract: AccountId::from_str(contract_address.as_str())?,
            msg: serde_json::to_vec(&migrate_msg)?,
            code_id: new_code_id,
        };
        let result = self
            .sender()
            .commit_tx(vec![exec_msg], None)
            .await
            .map_err(Into::into)?;
        Ok(result)
    }

    /// Upload a contract to the chain.
    pub async fn upload<T: Uploadable>(
        &self,
        uploadable: &T,
    ) -> Result<CosmTxResponse, DaemonError> {
        self.upload_with_access_config(uploadable, None).await
    }

    /// Upload a contract to the chain and specify the permissions for instantiating
    pub async fn upload_with_access_config<T: Uploadable>(
        &self,
        _uploadable: &T,
        access: Option<AccessConfig>,
    ) -> Result<CosmTxResponse, DaemonError> {
        let wasm_path = <T as Uploadable>::wasm(self.chain_info());

        log::debug!(target: &transaction_target(), "Uploading file at {:?}", wasm_path);

        let result = upload_wasm(self.sender(), wasm_path, access).await?;

        log::info!(target: &transaction_target(), "Uploading done: {:?}", result.txhash);

        let code_id = result.uploaded_code_id().unwrap();

        // wait for the node to return the contract information for this upload
        let wasm = CosmWasm::new_async(self.channel());
        while wasm._code(code_id).await.is_err() {
            self.next_block().await?;
        }
        Ok(result)
    }
}

pub async fn upload_wasm<T: TxSender>(
    sender: &T,
    wasm_path: WasmPath,
    access: Option<AccessConfig>,
) -> Result<CosmTxResponse, DaemonError> {
    let file_contents = std::fs::read(wasm_path.path())?;
    let mut e = write::GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(&file_contents)?;
    let wasm_byte_code = e.finish()?;
    let store_msg = cosmrs::cosmwasm::MsgStoreCode {
        sender: sender.msg_sender().map_err(Into::into)?,
        wasm_byte_code,
        instantiate_permission: access.map(access_config_to_cosmrs).transpose()?,
    };

    sender
        .commit_tx(vec![store_msg], None)
        .await
        .map_err(Into::into)
}

pub(crate) fn access_config_to_cosmrs(
    access_config: AccessConfig,
) -> Result<cosmrs::cosmwasm::AccessConfig, DaemonError> {
    let response = match access_config {
        AccessConfig::Nobody => cosmrs::cosmwasm::AccessConfig {
            permission: cosmrs::cosmwasm::AccessType::Nobody,
            addresses: vec![],
        },
        AccessConfig::Everybody => cosmrs::cosmwasm::AccessConfig {
            permission: cosmrs::cosmwasm::AccessType::Everybody,
            addresses: vec![],
        },
        AccessConfig::AnyOfAddresses(addresses) => cosmrs::cosmwasm::AccessConfig {
            permission: cosmrs::cosmwasm::AccessType::AnyOfAddresses,
            addresses: addresses
                .into_iter()
                .map(|a| a.parse())
                .collect::<Result<_, _>>()?,
        },
        AccessConfig::Unspecified => cosmrs::cosmwasm::AccessConfig {
            permission: cosmrs::cosmwasm::AccessType::Unspecified,
            addresses: vec![],
        },
    };
    Ok(response)
}

impl Querier for DaemonAsync {
    type Error = DaemonError;
}

impl AsyncWasmQuerier for DaemonAsync {
    /// Query a contract.
    fn smart_query<Q: Serialize + Sync, T: DeserializeOwned>(
        &self,
        address: &Addr,
        query_msg: &Q,
    ) -> impl std::future::Future<Output = Result<T, DaemonError>> + Send {
        let query_data = serde_json::to_vec(query_msg).unwrap();
        async {
            let mut client =
                cosmos_modules::cosmwasm::query_client::QueryClient::new(self.channel());
            let resp = client
                .smart_contract_state(cosmos_modules::cosmwasm::QuerySmartContractStateRequest {
                    address: address.into(),
                    query_data,
                })
                .await?;
            Ok(from_str(from_utf8(&resp.into_inner().data).unwrap())?)
        }
    }
}

pub(crate) fn parse_cw_coins(
    coins: &[cosmwasm_std::Coin],
) -> Result<Vec<cosmrs::Coin>, DaemonError> {
    coins
        .iter()
        .map(|cosmwasm_std::Coin { amount, denom }| {
            Ok(cosmrs::Coin {
                amount: amount.u128(),
                denom: Denom::from_str(denom)?,
            })
        })
        .collect::<Result<Vec<_>, DaemonError>>()
}

pub(crate) fn proto_parse_cw_coins(
    coins: &[cosmwasm_std::Coin],
) -> Result<Vec<cosmrs::proto::cosmos::base::v1beta1::Coin>, DaemonError> {
    coins
        .iter()
        .map(|cosmwasm_std::Coin { amount, denom }| {
            Ok(cosmrs::proto::cosmos::base::v1beta1::Coin {
                amount: amount.to_string(),
                denom: denom.clone(),
            })
        })
        .collect::<Result<Vec<_>, DaemonError>>()
}
