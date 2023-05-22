//! Main functional component for interacting with a contract. Used as the base for generating contract interfaces.
use crate::environment::ChainUpload;
use crate::prelude::{CwEnv, Uploadable};
use crate::{
    environment::TxResponse, error::CwOrchError, index_response::IndexResponse,
    state::StateInterface,
};
use cosmwasm_std::{Addr, Coin};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

/// An instance of a contract. Contains references to the execution environment (chain) and a local state (state)
/// The state is used to store contract addresses/code-ids
#[derive(Clone)]
pub struct Contract<Chain: CwEnv> {
    /// ID of the contract, used to retrieve addr/code-id
    pub id: String,
    /// Chain object that handles tx execution and queries.
    pub(crate) chain: Chain,
}

/// Expose chain and state function to call them on the contract
impl<Chain: CwEnv + Clone> Contract<Chain> {
    /// Creates a new contract instance
    pub fn new(id: impl ToString, chain: Chain) -> Self {
        Contract {
            id: id.to_string(),
            chain,
        }
    }

    /// `get_chain` instead of `chain` to disambiguate from the std prelude .chain() method.
    pub fn get_chain(&self) -> &Chain {
        &self.chain
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
    ) -> Result<TxResponse<Chain>, CwOrchError> {
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
    ) -> Result<TxResponse<Chain>, CwOrchError> {
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

    /// Query the contract
    pub fn query<Q: Serialize + Debug, T: Serialize + DeserializeOwned + Debug>(
        &self,
        query_msg: &Q,
    ) -> Result<T, CwOrchError> {
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
    ) -> Result<TxResponse<Chain>, CwOrchError> {
        log::info!("Migrating {:?} to code_id {}", self.id, new_code_id);
        self.chain
            .migrate(migrate_msg, new_code_id, &self.address()?)
            .map_err(Into::into)
    }

    // State interfaces
    /// Returns state address for contract
    pub fn address(&self) -> Result<Addr, CwOrchError> {
        self.chain.state().get_address(&self.id)
    }

    /// Sets state address for contract
    pub fn set_address(&self, address: &Addr) {
        self.chain.state().set_address(&self.id, address)
    }

    /// Returns state code_id for contract
    pub fn code_id(&self) -> Result<u64, CwOrchError> {
        self.chain.state().get_code_id(&self.id)
    }

    /// Sets state code_id for contract
    pub fn set_code_id(&self, code_id: u64) {
        self.chain.state().set_code_id(&self.id, code_id)
    }
}

impl<Chain: CwEnv + Clone + ChainUpload> Contract<Chain> {
    /// Upload a contract given its source
    pub fn upload(&self, source: &impl Uploadable) -> Result<TxResponse<Chain>, CwOrchError> {
        log::info!("Uploading {}", self.id);
        let resp = self.chain.upload(source).map_err(Into::into)?;
        let code_id = resp.uploaded_code_id()?;
        self.set_code_id(code_id);
        log::info!("uploaded {} with code id {}", self.id, code_id);
        log::debug!("Upload response: {:?}", resp);
        Ok(resp)
    }
}
