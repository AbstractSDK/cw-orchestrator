use super::{
    cosmos_modules,
    error::DaemonError,
    queriers::{DaemonQuerier, Node},
    sender::{Sender, Wallet},
    state::{ChainKind, DaemonOptions, DaemonState},
    tx_resp::CosmTxResponse,
};
use crate::{
    contract::ContractCodeReference, state::ChainState, tx_handler::TxHandler, BootExecute, CallAs,
    ContractInstance,
};
use cosmrs::{
    cosmwasm::{MsgExecuteContract, MsgInstantiateContract, MsgMigrateContract},
    tendermint::Time,
    AccountId, Denom,
};
use cosmwasm_std::{Addr, Coin, Empty};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::from_str;
use std::{
    fmt::Debug,
    rc::Rc,
    str::{from_utf8, FromStr},
    sync::Arc,
    time::Duration,
};
use tokio::runtime::Runtime;

/**
# instantiate_daemon_env

Creates a new [Daemon] instance and returns the sender address using within the instance and the daemon instance.

## Arguments

* `runtime` - An [Arc] reference to a [Runtime] instance.
* `options` - A [DaemonOptions] struct that contains the options for creating the [Daemon] instance.

## Returns

* ([Addr], [Daemon]) - sender address being used and the daemon instance

## Errors

Returns an [anyhow::Error] if there was an error while creating the [DaemonState].

## Example

```ignore
use std::sync::Arc;
use tokio::runtime::Runtime;

use boot_core::{DaemonOptionsBuilder, networks, instantiate_daemon_env};

let runtime = Arc::new(Runtime::new().unwrap());

let options = DaemonOptionsBuilder::default()
    .network(networks::LOCAL_JUNO)
    .deployment_id("v0.1.0")
    .build()
    .unwrap();

let (sender, chain) = instantiate_daemon_env(&runtime, options).unwrap();
```
*/
pub fn instantiate_daemon_env(
    runtime: &Arc<Runtime>,
    options: DaemonOptions,
) -> anyhow::Result<(Addr, Daemon)> {
    let state = Rc::new(runtime.block_on(DaemonState::new(options))?);
    let sender = Rc::new(Sender::new(&state)?);
    let chain = Daemon::new(&sender, &state, runtime)?;
    Ok((sender.address()?, chain))
}

#[derive(Clone)]
pub struct Daemon {
    pub sender: Wallet,
    pub state: Rc<DaemonState>,
    pub runtime: Arc<Runtime>,
}

impl Daemon {
    pub fn new(
        sender: &Wallet,
        state: &Rc<DaemonState>,
        runtime: &Arc<Runtime>,
    ) -> anyhow::Result<Self> {
        let instance = Self {
            sender: sender.clone(),
            state: state.clone(),
            runtime: runtime.clone(),
        };
        Ok(instance)
    }

    async fn wait(&self) {
        match self.state.kind {
            ChainKind::Local => tokio::time::sleep(Duration::from_secs(6)).await,
            ChainKind::Mainnet => tokio::time::sleep(Duration::from_secs(60)).await,
            ChainKind::Testnet => tokio::time::sleep(Duration::from_secs(30)).await,
        }
    }

    pub fn set_deployment(&mut self, deployment_id: impl Into<String>) -> Result<(), DaemonError> {
        // This ensures that you don't change the deployment of any contract that has been used before.
        // It reduces the probability of shooting yourself in the foot.
        Rc::get_mut(&mut self.state)
            .ok_or(DaemonError::SharedDaemonState)?
            .set_deployment(deployment_id);
        Ok(())
    }

    /// Perform a query with a given querier
    pub fn query<Querier: DaemonQuerier>(&self) -> Querier {
        Querier::new(self.sender.channel())
    }
}

impl ChainState for Daemon {
    type Out = Rc<DaemonState>;

    fn state(&self) -> Self::Out {
        self.state.clone()
    }
}

/// [Daemon] implements the [TxHandler] trait which defines how transactions
/// are handled by a CosmWasm-based daemon.
///
/// ## Example
/// ```ignore
/// use cosmwasm_std::{Coin, Addr};
///
/// let daemon = Daemon::new("tcp://127.0.0.1:26657".to_string());
/// let coins = vec![Coin::new(1000, "token")];
/// let contract_addr = Addr::unchecked("contract_address");
///
/// // Execute a message on the smart contract at the specified address
/// let exec_msg = ExecuteMsg::Deposit {};
/// let response = daemon.execute(&exec_msg, &coins, &contract_addr)?;
///
/// // Upload smart contract code
/// let mut code = ContractCodeReference::<Empty>::new();
/// code.set_wasm("contract.wasm".to_string());
/// let response = daemon.upload(&mut code)?;
///
/// // Wait for a number of blocks to be confirmed
/// daemon.wait_blocks(5)?;
///
/// // Get block information
/// let block_info = daemon.block_info()?;
/// ```
impl TxHandler for Daemon {
    /// The `Response` type associated with the [TxHandler] trait for this struct is [CosmTxResponse].
    type Response = CosmTxResponse;

    /// The `Error` type associated with the [TxHandler] trait for this struct is [DaemonError].
    type Error = DaemonError;

    /// Returns the address of the sender.
    fn sender(&self) -> Addr {
        self.sender.address().unwrap()
    }

    /// Executes a contract on the blockchain network with the given input parameters.
    fn execute<E: Serialize>(
        &self,
        exec_msg: &E,
        coins: &[cosmwasm_std::Coin],
        contract_address: &Addr,
    ) -> Result<Self::Response, DaemonError> {
        let exec_msg: MsgExecuteContract = MsgExecuteContract {
            sender: self.sender.pub_addr()?,
            contract: AccountId::from_str(contract_address.as_str())?,
            msg: serde_json::to_vec(&exec_msg)?,
            funds: parse_cw_coins(coins)?,
        };
        let result = self
            .runtime
            .block_on(self.sender.commit_tx(vec![exec_msg], None))?;
        Ok(result)
    }

    /// Instantiates a new contract on the blockchain network with the given input parameters.
    fn instantiate<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[Coin],
    ) -> Result<Self::Response, DaemonError> {
        let sender = &self.sender;

        let init_msg = MsgInstantiateContract {
            code_id,
            label: Some(label.unwrap_or("instantiate_contract").to_string()),
            admin: admin.map(|a| FromStr::from_str(a.as_str()).unwrap()),
            sender: sender.pub_addr()?,
            msg: serde_json::to_vec(&init_msg)?,
            funds: parse_cw_coins(coins)?,
        };

        let result = self
            .runtime
            .block_on(sender.commit_tx(vec![init_msg], None))?;

        Ok(result)
    }

    /// Queries a contract on the blockchain network with the given `QueryMsg`
    fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, DaemonError> {
        let sender = &self.sender;

        let mut client = cosmos_modules::cosmwasm::query_client::QueryClient::new(sender.channel());

        let resp = self.runtime.block_on(client.smart_contract_state(
            cosmos_modules::cosmwasm::QuerySmartContractStateRequest {
                address: contract_address.to_string(),
                query_data: serde_json::to_vec(&query_msg)?,
            },
        ))?;

        Ok(from_str(from_utf8(&resp.into_inner().data).unwrap())?)
    }

    /// Executes a migration of a contract to a new version with the given `new_code_id`. The contract is
    fn migrate<M: Serialize + Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
        contract_address: &Addr,
    ) -> Result<Self::Response, DaemonError> {
        let exec_msg: MsgMigrateContract = MsgMigrateContract {
            sender: self.sender.pub_addr()?,
            contract: AccountId::from_str(contract_address.as_str())?,
            msg: serde_json::to_vec(&migrate_msg)?,
            code_id: new_code_id,
        };
        let result = self
            .runtime
            .block_on(self.sender.commit_tx(vec![exec_msg], None))?;
        Ok(result)
    }

    /// Uploads the given contract source code to the blockchain.
    fn upload(
        &self,
        contract_source: &mut ContractCodeReference<Empty>,
    ) -> Result<Self::Response, DaemonError> {
        let sender = &self.sender;
        let wasm_path = &contract_source.get_wasm_code_path()?;

        log::debug!("{}", wasm_path);

        let file_contents = std::fs::read(wasm_path)?;
        let store_msg = cosmrs::cosmwasm::MsgStoreCode {
            sender: sender.pub_addr()?,
            wasm_byte_code: file_contents,
            instantiate_permission: None,
        };

        let result = self
            .runtime
            .block_on(sender.commit_tx(vec![store_msg], None))?;

        log::info!("Uploaded: {:?}", result.txhash);

        // Extra time-out to ensure contract code propagation
        self.runtime.block_on(self.wait());

        Ok(result)
    }

    /// Wait for a specified number of blocks to be produced.
    fn wait_blocks(&self, amount: u64) -> Result<(), DaemonError> {
        let mut last_height = self.runtime.block_on(self.query::<Node>().block_height())?;
        let end_height = last_height + amount;

        while last_height < end_height {
            // wait
            self.runtime
                .block_on(tokio::time::sleep(Duration::from_secs(4)));

            // ping latest block
            last_height = self.runtime.block_on(self.query::<Node>().block_height())?;
        }

        Ok(())
    }

    /// Wait for a specified number of seconds.
    fn wait_seconds(&self, secs: u64) -> Result<(), DaemonError> {
        self.runtime
            .block_on(tokio::time::sleep(Duration::from_secs(secs)));

        Ok(())
    }

    /// Wait until the next block is produced and confirmed by the network.
    fn next_block(&self) -> Result<(), DaemonError> {
        let mut last_height = self.runtime.block_on(self.query::<Node>().block_height())?;
        let end_height = last_height + 1;

        while last_height < end_height {
            // wait
            self.runtime
                .block_on(tokio::time::sleep(Duration::from_secs(4)));

            // ping latest block
            last_height = self.runtime.block_on(self.query::<Node>().block_height())?;
        }

        Ok(())
    }

    /// Get the current block information from the chain.
    fn block_info(&self) -> Result<cosmwasm_std::BlockInfo, DaemonError> {
        let block = self.runtime.block_on(self.query::<Node>().latest_block())?;
        let since_epoch = block.header.time.duration_since(Time::unix_epoch())?;
        let time = cosmwasm_std::Timestamp::from_nanos(since_epoch.as_nanos() as u64);

        Ok(cosmwasm_std::BlockInfo {
            height: block.header.height.value(),
            time,
            chain_id: block.header.chain_id.to_string(),
        })
    }
}

/// Provides a trait implementation to call a contract instance as a given sender
impl<T: BootExecute<Daemon> + ContractInstance<Daemon> + Clone> CallAs<Daemon> for T {
    type Sender = Wallet;

    /// Sets the sender of the contract instance
    fn set_sender(&mut self, sender: &Self::Sender) {
        self.as_instance_mut().chain.sender = sender.clone();
    }

    /// Returns a clone of the contract instance with the given sender
    fn call_as(&self, sender: &Self::Sender) -> Self {
        let mut contract = self.clone();
        contract.set_sender(sender);
        contract
    }
}

/// Parses a vector of [cosmwasm_std::Coin] into a vector of [cosmrs::Coin]
///
/// ## Arguments
///
/// * `coins` - A slice of [cosmwasm_std::Coin] to be parsed
///
/// ## Errors
///
/// Returns a [DaemonError] if the conversion fails.
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
