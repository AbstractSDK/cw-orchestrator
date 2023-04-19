use crate::CwEnv;
use crate::{
    error::BootError, index_response::IndexResponse, state::StateInterface, tx_handler::TxResponse,
};
use cosmwasm_std::{Addr, Coin, CustomQuery, Empty};
use cw_multi_test::Contract as TestContract;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

/**
An instance of a contract.

Contains references to the execution environment (chain) and a local state (state).

The state is used to store contract addresses/code-ids

## Example
```
use std::sync::Arc;
use tokio::runtime::Runtime;

use boot_core::{
    instantiate_daemon_env, networks::LOCAL_JUNO,
    Contract, ContractWrapper, Daemon,
    DaemonOptionsBuilder,
};

let runtime = Arc::new(Runtime::new().unwrap());

let options = DaemonOptionsBuilder::default()
    .network(LOCAL_JUNO)
    .deployment_id("v0.1.0")
    .build()
    .unwrap();

let (sender, chain) = instantiate_daemon_env(&runtime, options).unwrap();

let contract = Contract::new("cw-plus:cw20_base", chain)
    .with_mock(Box::new(
        ContractWrapper::new_with_empty(
            cw20_base::contract::execute,
            cw20_base::contract::instantiate,
            cw20_base::contract::query,
        )
        .with_migrate(cw20_base::contract::migrate),
    ))
    .with_wasm_path("cw20_base.wasm");
```
*/
#[derive(Clone)]
pub struct Contract<Chain: CwEnv> {
    /// ID of the contract, used to retrieve addr/code-id
    pub id: String,
    /// Contract end points
    pub(crate) source: ContractCodeReference,
    /// Chain object that handles tx execution and queries.
    pub(crate) chain: Chain,
}

#[derive(Default)]
pub struct ContractCodeReference<ExecT = Empty, QueryT = Empty>
where
    ExecT: Clone + Debug + PartialEq + JsonSchema + DeserializeOwned + 'static,
    QueryT: CustomQuery + DeserializeOwned + 'static,
{
    /// This wasm file will be uploaded
    pub wasm_code_path: Option<String>,
    /// Such as instantiate, execute and query
    pub contract_endpoints: Option<Box<dyn TestContract<ExecT, QueryT>>>,
}

impl Clone for ContractCodeReference {
    fn clone(&self) -> Self {
        Self {
            wasm_code_path: self.wasm_code_path.clone(),
            contract_endpoints: None,
        }
    }
}

/// Expose chain and state function to call them on the contract
impl<Chain: CwEnv + Clone> Contract<Chain> {
    pub fn new(id: impl ToString, chain: Chain) -> Self {
        Contract {
            id: id.to_string(),
            chain,
            source: ContractCodeReference::default(),
        }
    }

    /// `get_chain` instead of `chain` to disambiguate from the std prelude .chain() method.
    pub fn get_chain(&self) -> &Chain {
        &self.chain
    }

    /// Full path to the wasm file to be uploaded
    pub fn with_wasm_path(mut self, path: impl ToString) -> Self {
        self.source.wasm_code_path = Some(path.to_string());
        self
    }

    /// Create with mock contract
    pub fn with_mock(mut self, mock_contract: Box<dyn TestContract<Empty, Empty>>) -> Self {
        self.source.contract_endpoints = Some(mock_contract);
        self
    }

    /// Change mock contract
    pub fn set_mock(&mut self, mock_contract: Box<dyn TestContract<Empty, Empty>>) {
        self.source.contract_endpoints = Some(mock_contract);
    }

    /// Sets the address of the contract in the local state
    pub fn with_address(self, address: Option<&Addr>) -> Self {
        if let Some(address) = address {
            self.set_address(address)
        }
        self
    }

    // Chain interfaces
    /// Executes an operation on the contract
    pub fn execute<E: Serialize + Debug>(
        &self,
        msg: &E,
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, BootError> {
        log::info!("Executing {:#?} on {}", msg, self.id);
        let resp = self
            .chain
            .execute(msg, coins.unwrap_or(&[]), &self.address()?);
        log::debug!("execute response: {:?}", resp);
        resp.map_err(Into::into)
    }

    /// Initializes the contract
    pub fn instantiate<I: Serialize + Debug>(
        &self,
        msg: &I,
        admin: Option<&Addr>,
        coins: Option<&[Coin]>,
    ) -> Result<TxResponse<Chain>, BootError> {
        log::info!("Instantiating {} with msg {:#?}", self.id, msg);

        let resp = self
            .chain
            .instantiate(
                self.code_id()?,
                msg,
                Some(&self.id),
                admin,
                coins.unwrap_or(&[]),
            )
            .map_err(Into::into)?;
        let contract_address = resp.instantiated_contract_address()?;

        self.set_address(&contract_address);

        log::info!("Instantiated {} with address {}", self.id, contract_address);

        log::debug!("Instantiate response: {:?}", resp);

        Ok(resp)
    }

    /// Uploads the contract
    pub fn upload(&mut self) -> Result<TxResponse<Chain>, BootError> {
        log::info!("Uploading {}", self.id);
        let resp = self.chain.upload(&mut self.source).map_err(Into::into)?;
        let code_id = resp.uploaded_code_id()?;
        self.set_code_id(code_id);
        log::info!("uploaded {} with code id {}", self.id, code_id);
        log::debug!("Upload response: {:?}", resp);
        Ok(resp)
    }

    /// Queries the contract
    pub fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned + Debug>(
        &self,
        query_msg: &Q,
    ) -> Result<T, BootError> {
        log::info!("Querying {:#?} on {}", query_msg, self.id);
        let resp = self
            .chain
            .query(query_msg, &self.address()?)
            .map_err(Into::into)?;
        log::debug!("Query response: {:?}", resp);
        Ok(resp)
    }

    /// Migrates the contract
    pub fn migrate<M: Serialize + Debug>(
        &self,
        migrate_msg: &M,
        new_code_id: u64,
    ) -> Result<TxResponse<Chain>, BootError> {
        log::info!("Migrating {:?} to code_id {}", self.id, new_code_id);
        self.chain
            .migrate(migrate_msg, new_code_id, &self.address()?)
            .map_err(Into::into)
    }

    // State interfaces
    /// Returns state address for contract
    pub fn address(&self) -> Result<Addr, BootError> {
        self.chain.state().get_address(&self.id)
    }

    /// Sets state address for contract
    pub fn set_address(&self, address: &Addr) {
        self.chain.state().set_address(&self.id, address)
    }

    /// Returns state code_id for contract
    pub fn code_id(&self) -> Result<u64, BootError> {
        self.chain.state().get_code_id(&self.id)
    }

    /// Sets state code_id for contract
    pub fn set_code_id(&self, code_id: u64) {
        self.chain.state().set_code_id(&self.id, code_id)
    }
}
