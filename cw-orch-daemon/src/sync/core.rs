use std::{fmt::Debug, sync::Arc, time::Duration};

use super::super::{sender::Wallet, DaemonAsync};
use crate::{
    queriers::{cosmrs_to_cosmwasm_coins, Bank, DaemonQuerier, Node},
    CosmTxResponse, DaemonBuilder, DaemonError, DaemonState,
};

use cosmrs::tendermint::Time;
use cosmwasm_std::{Addr, Coin};
use cw_orch_core::{
    contract::{interface_traits::Uploadable, WasmPath},
    environment::{BankQuerier, ChainState, EnvironmentInfo, EnvironmentQuerier, TxHandler},
};
use cw_orch_traits::stargate::Stargate;
use serde::{de::DeserializeOwned, Serialize};
use tokio::runtime::Handle;
use tonic::transport::Channel;

#[derive(Clone)]
/**
    Represents a blockchain node.
    Is constructed with the [DaemonBuilder].

    ## Usage

    ```rust,no_run
    use cw_orch_daemon::{Daemon, networks};
    use tokio::runtime::Runtime;

    let rt = Runtime::new().unwrap();
    let daemon: Daemon = Daemon::builder()
        .chain(networks::JUNO_1)
        .handle(rt.handle())
        .build()
        .unwrap();
    ```
    ## Environment Execution

    The Daemon implements [`TxHandler`] which allows you to perform transactions on the chain.

    ## Querying

    Different Cosmos SDK modules can be queried through the daemon by calling the [`Daemon.query_client<Querier>`] method with a specific querier.
    See [Querier](crate::queriers) for examples.
*/
pub struct Daemon {
    pub daemon: DaemonAsync,
    /// Runtime handle to execute async tasks
    pub rt_handle: Handle,
}

impl Daemon {
    /// Get the daemon builder
    pub fn builder() -> DaemonBuilder {
        DaemonBuilder::default()
    }

    /// Perform a query with a given querier
    /// See [Querier](crate::queriers) for examples.
    pub fn query_client<Querier: DaemonQuerier>(&self) -> Querier {
        self.daemon.query_client()
    }

    /// Get the channel configured for this Daemon
    pub fn channel(&self) -> Channel {
        self.daemon.state.grpc_channel.clone()
    }

    /// Get the channel configured for this Daemon
    pub fn wallet(&self) -> Wallet {
        self.daemon.sender.clone()
    }

    /// Returns a new [`DaemonBuilder`] with the current configuration. 
    /// Does not consume the original [`Daemon`].
    pub fn rebuild(&self) -> DaemonBuilder {
        let mut builder = Self::builder();
        builder
            .chain(self.state().chain_data.clone())
            .sender((*self.daemon.sender).clone())
            .handle(&self.rt_handle)
            .deployment_id(&self.state().deployment_id);
        builder
    }
}

impl ChainState for Daemon {
    type Out = Arc<DaemonState>;

    fn state(&self) -> Self::Out {
        self.daemon.state.clone()
    }
}

// Execute on the real chain, returns tx response
impl TxHandler for Daemon {
    type Response = CosmTxResponse;
    type Error = DaemonError;
    type ContractSource = WasmPath;
    type Sender = Wallet;

    fn sender(&self) -> Addr {
        self.daemon.sender.address().unwrap()
    }

    fn set_sender(&mut self, sender: Self::Sender) {
        self.daemon.sender = sender
    }

    fn upload(&self, uploadable: &impl Uploadable) -> Result<Self::Response, DaemonError> {
        self.rt_handle.block_on(self.daemon.upload(uploadable))
    }

    fn execute<E: Serialize>(
        &self,
        exec_msg: &E,
        coins: &[cosmwasm_std::Coin],
        contract_address: &Addr,
    ) -> Result<Self::Response, DaemonError> {
        self.rt_handle
            .block_on(self.daemon.execute(exec_msg, coins, contract_address))
    }

    fn instantiate<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[Coin],
    ) -> Result<Self::Response, DaemonError> {
        self.rt_handle.block_on(
            self.daemon
                .instantiate(code_id, init_msg, label, admin, coins),
        )
    }

    fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, DaemonError> {
        self.rt_handle
            .block_on(self.daemon.query(query_msg, contract_address))
    }

    fn migrate<M: Serialize + Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
        contract_address: &Addr,
    ) -> Result<Self::Response, DaemonError> {
        self.rt_handle.block_on(
            self.daemon
                .migrate(migrate_msg, new_code_id, contract_address),
        )
    }

    fn wait_blocks(&self, amount: u64) -> Result<(), DaemonError> {
        let mut last_height = self
            .rt_handle
            .block_on(self.query_client::<Node>().block_height())?;
        let end_height = last_height + amount;

        while last_height < end_height {
            // wait
            self.rt_handle
                .block_on(tokio::time::sleep(Duration::from_secs(4)));

            // ping latest block
            last_height = self
                .rt_handle
                .block_on(self.query_client::<Node>().block_height())?;
        }
        Ok(())
    }

    fn wait_seconds(&self, secs: u64) -> Result<(), DaemonError> {
        self.rt_handle
            .block_on(tokio::time::sleep(Duration::from_secs(secs)));

        Ok(())
    }

    fn next_block(&self) -> Result<(), DaemonError> {
        let mut last_height = self
            .rt_handle
            .block_on(self.query_client::<Node>().block_height())?;
        let end_height = last_height + 1;

        while last_height < end_height {
            // wait
            self.rt_handle
                .block_on(tokio::time::sleep(Duration::from_secs(4)));

            // ping latest block
            last_height = self
                .rt_handle
                .block_on(self.query_client::<Node>().block_height())?;
        }
        Ok(())
    }

    fn block_info(&self) -> Result<cosmwasm_std::BlockInfo, DaemonError> {
        let block = self
            .rt_handle
            .block_on(self.query_client::<Node>().latest_block())?;
        let since_epoch = block.header.time.duration_since(Time::unix_epoch())?;
        let time = cosmwasm_std::Timestamp::from_nanos(since_epoch.as_nanos() as u64);
        Ok(cosmwasm_std::BlockInfo {
            height: block.header.height.value(),
            time,
            chain_id: block.header.chain_id.to_string(),
        })
    }
}

impl BankQuerier for Daemon {
    fn balance(
        &self,
        address: impl Into<String>,
        denom: Option<String>,
    ) -> Result<Vec<cosmwasm_std::Coin>, <Self as TxHandler>::Error> {
        let bank = Bank::new(self.channel());

        let cosmrs_coins = self.rt_handle.block_on(bank.balance(address, denom))?;

        cosmrs_coins
            .iter()
            .map(|c| cosmrs_to_cosmwasm_coins(c.clone()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    fn supply_of(
        &self,
        denom: impl Into<String>,
    ) -> Result<cosmwasm_std::Coin, <Self as TxHandler>::Error> {
        let bank = Bank::new(self.channel());

        let cosmrs_coin = self.rt_handle.block_on(bank.supply_of(denom))?;
        cosmrs_to_cosmwasm_coins(cosmrs_coin.clone()).map_err(Into::into)
    }
}

impl Stargate for Daemon {
    fn commit_any<R>(
        &self,
        msgs: Vec<prost_types::Any>,
        memo: Option<&str>,
    ) -> Result<Self::Response, Self::Error> {
        self.rt_handle.block_on(
            self.wallet().commit_tx_any(
                msgs.iter()
                    .map(|msg| cosmrs::Any {
                        type_url: msg.type_url.clone(),
                        value: msg.value.clone(),
                    })
                    .collect(),
                memo,
            ),
        )
    }
}

impl EnvironmentQuerier for Daemon {
    fn env_info(&self) -> EnvironmentInfo {
        let state = &self.daemon.sender.daemon_state;
        EnvironmentInfo {
            chain_id: state.chain_data.chain_id.to_string(),
            chain_name: state.chain_data.chain_name.clone(),
            deployment_id: state.deployment_id.clone(),
        }
    }
}
