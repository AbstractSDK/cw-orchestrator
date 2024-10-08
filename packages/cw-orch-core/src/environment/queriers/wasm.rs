use cosmwasm_std::{from_json, Addr, Checksum, CodeInfoResponse, ContractInfoResponse};
use cw_storage_plus::{Item, Map, PrimaryKey};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    contract::interface_traits::{ContractInstance, Uploadable},
    environment::ChainState,
    CwEnvError,
};

use super::Querier;

pub trait WasmQuerier: Querier {
    type Chain: ChainState;

    fn code_id_hash(&self, code_id: u64) -> Result<Checksum, Self::Error>;

    /// Query contract info
    fn contract_info(&self, address: &Addr) -> Result<ContractInfoResponse, Self::Error>;

    /// Query contract state
    fn raw_query(&self, address: &Addr, query_keys: Vec<u8>) -> Result<Vec<u8>, Self::Error>;

    fn item_query<T: Serialize + DeserializeOwned>(
        &self,
        address: &Addr,
        item: Item<T>,
    ) -> Result<T, CwEnvError> {
        let raw_value = self
            .raw_query(address, item.as_slice().to_vec())
            .map_err(Into::into)?;

        from_json(raw_value).map_err(Into::into)
    }

    fn map_query<'a, T: Serialize + DeserializeOwned, K: PrimaryKey<'a>>(
        &self,
        address: &Addr,
        map: Map<K, T>,
        key: K,
    ) -> Result<T, CwEnvError> {
        let total_key = map.key(key).to_vec();
        let current_manager_version = self.raw_query(address, total_key).map_err(Into::into)?;

        from_json(current_manager_version).map_err(Into::into)
    }

    fn smart_query<Q: Serialize, T: DeserializeOwned>(
        &self,
        address: &Addr,
        query_msg: &Q,
    ) -> Result<T, Self::Error>;

    /// Query code
    fn code(&self, code_id: u64) -> Result<CodeInfoResponse, Self::Error>;

    /// Returns the checksum of the WASM file if the env supports it. Will re-upload every time if not supported.
    fn local_hash<T: Uploadable + ContractInstance<Self::Chain>>(
        &self,
        contract: &T,
    ) -> Result<Checksum, CwEnvError>;

    fn instantiate2_addr(
        &self,
        code_id: u64,
        creator: &Addr,
        salt: cosmwasm_std::Binary,
    ) -> Result<String, Self::Error>;
}

pub trait AsyncWasmQuerier: Querier + Sync {
    fn smart_query<Q: Serialize + Sync, T: DeserializeOwned>(
        &self,
        address: &Addr,
        query_msg: &Q,
    ) -> impl std::future::Future<Output = Result<T, Self::Error>> + Send;
}
