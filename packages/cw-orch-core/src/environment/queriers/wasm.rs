use cosmwasm_std::{CodeInfoResponse, ContractInfoResponse};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    contract::interface_traits::{ContractInstance, Uploadable},
    environment::TxHandler,
    CwEnvError,
};
use std::fmt::Debug;

use super::QueryHandler;

pub trait WasmQuerierGetter<E> {
    type Querier: WasmQuerier<Error = E>;
    fn wasm_querier(&self) -> Self::Querier;
}
pub trait WasmQuerier {
    type Error: Into<CwEnvError> + Debug;
    fn code_id_hash(&self, code_id: u64) -> Result<String, Self::Error>;

    /// Query contract info
    fn contract_info(
        &self,
        address: impl Into<String>,
    ) -> Result<ContractInfoResponse, Self::Error>;

    /// Query contract state
    fn contract_raw_state(
        &self,
        address: impl Into<String>,
        query_data: Vec<u8>,
    ) -> Result<Vec<u8>, Self::Error>;

    fn contract_smart_state<Q: Serialize, T: DeserializeOwned>(
        &self,
        address: impl Into<String>,
        query_data: &Q,
    ) -> Result<T, Self::Error>;

    /// Query code
    fn code(&self, code_id: u64) -> Result<CodeInfoResponse, Self::Error>;

    /// Returns the checksum of the WASM file if the env supports it. Will re-upload every time if not supported.
    fn local_hash<Chain: TxHandler + QueryHandler, T: Uploadable + ContractInstance<Chain>>(
        &self,
        contract: &T,
    ) -> Result<String, CwEnvError> {
        contract.wasm().checksum()
    }
}
