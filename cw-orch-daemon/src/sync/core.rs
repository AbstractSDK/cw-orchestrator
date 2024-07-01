use std::fmt::Debug;

use super::super::senders::base_sender::Wallet;
use crate::{
    queriers::{Bank, CosmWasm, Node}, senders::query::QuerySender, CosmTxResponse, DaemonAsyncBase, DaemonBuilder, DaemonError, DaemonState
};
use cosmwasm_std::{Addr, Coin};
use cw_orch_core::{
    contract::{interface_traits::Uploadable, WasmPath},
    environment::{ChainInfoOwned, ChainState, DefaultQueriers, QueryHandler, TxHandler},
};
use cw_orch_traits::stargate::Stargate;
use serde::Serialize;
use tokio::runtime::Handle;
use tonic::transport::Channel;

use crate::senders::tx::TxSender;

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
        .build()
        .unwrap();
    ```
    ## Environment Execution

    The Daemon implements [`TxHandler`] which allows you to perform transactions on the chain.

    ## Querying

    Different Cosmos SDK modules can be queried through the daemon by calling the [`Daemon.query_client<Querier>`] method with a specific querier.
    See [Querier](crate::queriers) for examples.
*/
pub struct DaemonBase<Sender> {
    pub(crate) daemon: DaemonAsyncBase<Sender>,
    /// Runtime handle to execute async tasks
    pub rt_handle: Handle,
}

pub type Daemon = DaemonBase<Wallet>;

impl<Sender> DaemonBase<Sender> {
    /// Get the daemon builder
    pub fn builder(chain: impl Into<ChainInfoOwned>) -> DaemonBuilder {
        DaemonBuilder::new(chain)
    }

    /// Get the channel configured for this Daemon
    pub fn channel(&self) -> Channel {
        self.daemon.sender.grpc_channel()
    }

    /// Get the channel configured for this Daemon
    pub fn sender(&self) -> Sender {
        self.daemon.sender.clone()
    }

    /// Get the mutable Sender object
    pub fn sender_mut(&mut self) -> &mut Sender {
        self.daemon.sender_mut()
    }

    /// Returns a new [`DaemonBuilder`] with the current configuration.
    /// Does not consume the original [`Daemon`].
    pub fn rebuild(&self) -> DaemonBuilder {
        DaemonBuilder {
            state: Some(self.state()),
            chain: self.daemon.sender.chain_info().clone(),
            deployment_id: Some(self.daemon.state.deployment_id.clone()),
            state_path: None,
            write_on_change: None,
            handle: Some(self.rt_handle.clone()),
        }
    }

    /// Flushes all the state related to the current chain
    /// Only works on Local networks
    pub fn flush_state(&mut self) -> Result<(), DaemonError> {
        self.daemon.flush_state()
    }
}

impl<Sender> ChainState for DaemonBase<Sender> {
    type Out = DaemonState;

    fn state(&self) -> Self::Out {
        self.daemon.state.clone()
    }
}

// Execute on the real chain, returns tx response
impl<Sender: TxSender> TxHandler for DaemonBase<Sender> {
    type Response = CosmTxResponse;
    type Error = DaemonError;
    type ContractSource = WasmPath;
    type Sender = Sender;

    fn sender(&self) -> Addr {
        self.daemon.sender.address().unwrap()
    }

    fn set_sender(&mut self, sender: Self::Sender) {
        self.daemon.sender = sender
    }

    fn upload<T: Uploadable>(&self, uploadable: &T) -> Result<Self::Response, DaemonError> {
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

    fn instantiate2<I: Serialize + Debug>(
        &self,
        code_id: u64,
        init_msg: &I,
        label: Option<&str>,
        admin: Option<&Addr>,
        coins: &[cosmwasm_std::Coin],
        salt: cosmwasm_std::Binary,
    ) -> Result<Self::Response, Self::Error> {
        self.rt_handle.block_on(
            self.daemon
                .instantiate2(code_id, init_msg, label, admin, coins, salt),
        )
    }
}

impl<Sender: TxSender> Stargate for DaemonBase<Sender> {
    fn commit_any<R>(
        &self,
        msgs: Vec<prost_types::Any>,
        memo: Option<&str>,
    ) -> Result<Self::Response, Self::Error> {
        self.rt_handle
            .block_on(
                self.sender().commit_tx_any(
                    msgs.iter()
                        .map(|msg| cosmrs::Any {
                            type_url: msg.type_url.clone(),
                            value: msg.value.clone(),
                        })
                        .collect(),
                    memo,
                ),
            )
            .map_err(Into::into)
    }
}

impl<Sender: QuerySender> QueryHandler for DaemonBase<Sender> {
    type Error = DaemonError;

    fn wait_blocks(&self, amount: u64) -> Result<(), DaemonError> {
        self.rt_handle.block_on(self.daemon.wait_blocks(amount))?;

        Ok(())
    }

    fn wait_seconds(&self, secs: u64) -> Result<(), DaemonError> {
        self.rt_handle.block_on(self.daemon.wait_seconds(secs))?;

        Ok(())
    }

    fn next_block(&self) -> Result<(), DaemonError> {
        self.rt_handle.block_on(self.daemon.next_block())?;

        Ok(())
    }
}

impl<Sender: QuerySender> DefaultQueriers for DaemonBase<Sender> {
    type Bank = Bank;
    type Wasm = CosmWasm;
    type Node = Node;
}
