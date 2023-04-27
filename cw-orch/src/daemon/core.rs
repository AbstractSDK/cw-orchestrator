use super::{
    builder::DaemonBuilder,
    cosmos_modules,
    error::DaemonError,
    queriers::{DaemonQuerier, Node},
    sender::Wallet,
    state::{ChainKind, DaemonState},
    tx_resp::CosmTxResponse,
    wasm_path::WasmPath,
};
use crate::{state::ChainState, tx_handler::TxHandler, CallAs, ContractInstance, CwOrcExecute};
use cosmrs::{
    cosmwasm::{MsgExecuteContract, MsgInstantiateContract, MsgMigrateContract},
    tendermint::Time,
    AccountId, Denom,
};
use cosmwasm_std::{Addr, Coin};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::from_str;
use std::{
    fmt::Debug,
    rc::Rc,
    str::{from_utf8, FromStr},
    time::Duration,
};
use tokio::runtime::Handle;

#[derive(Clone)]
/**
    Represents a connection to a chain.
    Is constructed with the [DaemonBuilder].
*/
pub struct Daemon {
    pub sender: Wallet,
    pub state: Rc<DaemonState>,
    pub rt_handle: Handle,
}

impl Daemon {
    /// Get the daemon builder
    pub fn builder() -> DaemonBuilder {
        DaemonBuilder::default()
    }

    /// set the deployment after daemon
    // pub fn set_deployment(&mut self, deployment_id: impl Into<String>) -> Result<(), DaemonError> {
    //     // This ensures that you don't change the deployment of any contract that has been used before.
    //     // It reduces the probability of shooting yourself in the foot.
    //     Rc::get_mut(&mut self.state)
    //         .ok_or(DaemonError::SharedDaemonState)?
    //         .set_deployment(deployment_id);
    //     Ok(())
    // }

    /// Perform a query with a given querier
    pub fn query<Querier: DaemonQuerier>(&self) -> Querier {
        Querier::new(self.sender.channel())
    }

    async fn wait(&self) {
        match self.state.kind {
            ChainKind::Local => tokio::time::sleep(Duration::from_secs(6)).await,
            ChainKind::Mainnet => tokio::time::sleep(Duration::from_secs(60)).await,
            ChainKind::Testnet => tokio::time::sleep(Duration::from_secs(30)).await,
        }
    }
}

impl ChainState for Daemon {
    type Out = Rc<DaemonState>;

    fn state(&self) -> Self::Out {
        self.state.clone()
    }
}

// Execute on the real chain, returns tx response
impl TxHandler for Daemon {
    type Response = CosmTxResponse;
    type Error = DaemonError;
    type ContractSource = WasmPath;

    fn sender(&self) -> Addr {
        self.sender.address().unwrap()
    }

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
            .rt_handle
            .block_on(self.sender.commit_tx(vec![exec_msg], None))?;
        Ok(result)
    }

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
            .rt_handle
            .block_on(sender.commit_tx(vec![init_msg], None))?;
        // let address = &result.get_attribute_from_logs("instantiate", "_contract_address")[0].1;

        Ok(result)
    }

    fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Q,
        contract_address: &Addr,
    ) -> Result<T, DaemonError> {
        let sender = &self.sender;
        let mut client = cosmos_modules::cosmwasm::query_client::QueryClient::new(sender.channel());
        let resp = self.rt_handle.block_on(client.smart_contract_state(
            cosmos_modules::cosmwasm::QuerySmartContractStateRequest {
                address: contract_address.to_string(),
                query_data: serde_json::to_vec(&query_msg)?,
            },
        ))?;

        Ok(from_str(from_utf8(&resp.into_inner().data).unwrap())?)
    }

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
            .rt_handle
            .block_on(self.sender.commit_tx(vec![exec_msg], None))?;
        Ok(result)
    }

    fn upload(&self, contract_source: WasmPath) -> Result<Self::Response, DaemonError> {
        let sender = &self.sender;
        let wasm_path = contract_source.path();

        log::debug!("Uploading file at {:?}", wasm_path);

        let file_contents = std::fs::read(wasm_path)?;
        let store_msg = cosmrs::cosmwasm::MsgStoreCode {
            sender: sender.pub_addr()?,
            wasm_byte_code: file_contents,
            instantiate_permission: None,
        };
        let result = self
            .rt_handle
            .block_on(sender.commit_tx(vec![store_msg], None))?;

        log::info!("Uploaded: {:?}", result.txhash);

        // Extra time-out to ensure contract code propagation
        self.rt_handle.block_on(self.wait());
        Ok(result)
    }

    fn wait_blocks(&self, amount: u64) -> Result<(), DaemonError> {
        let mut last_height = self
            .rt_handle
            .block_on(self.query::<Node>().block_height())?;
        let end_height = last_height + amount;

        while last_height < end_height {
            // wait
            self.rt_handle
                .block_on(tokio::time::sleep(Duration::from_secs(4)));

            // ping latest block
            last_height = self
                .rt_handle
                .block_on(self.query::<Node>().block_height())?;
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
            .block_on(self.query::<Node>().block_height())?;
        let end_height = last_height + 1;

        while last_height < end_height {
            // wait
            self.rt_handle
                .block_on(tokio::time::sleep(Duration::from_secs(4)));

            // ping latest block
            last_height = self
                .rt_handle
                .block_on(self.query::<Node>().block_height())?;
        }
        Ok(())
    }

    fn block_info(&self) -> Result<cosmwasm_std::BlockInfo, DaemonError> {
        let block = self
            .rt_handle
            .block_on(self.query::<Node>().latest_block())?;
        let since_epoch = block.header.time.duration_since(Time::unix_epoch())?;
        let time = cosmwasm_std::Timestamp::from_nanos(since_epoch.as_nanos() as u64);
        Ok(cosmwasm_std::BlockInfo {
            height: block.header.height.value(),
            time,
            chain_id: block.header.chain_id.to_string(),
        })
    }
}

impl<T: CwOrcExecute<Daemon> + ContractInstance<Daemon> + Clone> CallAs<Daemon> for T {
    type Sender = Wallet;

    fn set_sender(&mut self, sender: &Self::Sender) {
        self.as_instance_mut().chain.sender = sender.clone();
    }

    fn call_as(&self, sender: &Self::Sender) -> Self {
        let mut contract = self.clone();
        contract.set_sender(sender);
        contract
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
