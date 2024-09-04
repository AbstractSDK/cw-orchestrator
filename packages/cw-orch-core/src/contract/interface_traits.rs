use super::{Contract, WasmPath};
use crate::{
    environment::{
        AsyncWasmQuerier, ChainInfoOwned, ChainState, CwEnv, Environment, QueryHandler, TxHandler,
        TxResponse, WasmQuerier,
    },
    error::CwEnvError,
    log::contract_target,
};
use cosmwasm_std::{Addr, Binary, Coin, Empty};
use cw_multi_test::Contract as MockContract;
use cw_storage_plus::{Item, Map, PrimaryKey};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

// Fn for custom implementation to return ContractInstance
/// Interface to the underlying `Contract` struct. Implemented automatically when using our macros.
pub trait ContractInstance<Chain: ChainState> {
    /// Return a reference to the underlying contract instance.
    fn as_instance(&self) -> &Contract<Chain>;

    /// Return a mutable reference to the underlying contract instance.
    fn as_instance_mut(&mut self) -> &mut Contract<Chain>;

    /// Returns the contract id.
    fn id(&self) -> String {
        self.as_instance().id.clone()
    }

    /// Returns the contract address for this instance.
    fn address(&self) -> Result<Addr, CwEnvError> {
        Contract::address(self.as_instance())
    }

    /// Returns the contract address as a [`String`].
    fn addr_str(&self) -> Result<String, CwEnvError> {
        Contract::address(self.as_instance()).map(|addr| addr.into_string())
    }

    /// Returns contract code_id.
    fn code_id(&self) -> Result<u64, CwEnvError> {
        Contract::code_id(self.as_instance())
    }

    /// Sets the address for the contract. Useful when the contract is already initialized
    /// and not registered in the configured state file.
    fn set_address(&self, address: &Addr) {
        Contract::set_address(self.as_instance(), address)
    }

    /// Removes the address for the contract
    fn remove_address(&self) {
        Contract::remove_address(self.as_instance())
    }

    /// Sets a default address for the contract. If the contract already has an address registered in the state, this won't be used.
    /// This is mostly used to ship address with a cw-orch package.
    fn set_default_address(&mut self, address: &Addr) {
        Contract::set_default_address(self.as_instance_mut(), address)
    }

    /// Sets the code_id for the contract. Useful when the contract is already initialized
    /// and not registered in the configured state file.
    fn set_code_id(&self, code_id: u64) {
        Contract::set_code_id(self.as_instance(), code_id)
    }

    /// Removes the code_id for the contract
    fn remove_code_id(&self) {
        Contract::remove_code_id(self.as_instance())
    }

    /// Sets a default address for the contract. If the contract already has an address registered in the state, this won't be used.
    /// This is mostly used to ship address with a cw-orch package.
    fn set_default_code_id(&mut self, code_id: u64) {
        Contract::set_default_code_id(self.as_instance_mut(), code_id)
    }
}

/// Trait that indicates that the contract can be instantiated with the associated message.
pub trait InstantiableContract {
    /// Instantiate message for the contract.
    type InstantiateMsg: Serialize + Debug;
}

/// Trait that indicates that the contract can be executed with the associated message.
pub trait ExecutableContract {
    /// Execute message for the contract.
    type ExecuteMsg: Serialize + Debug;
}

/// Trait that indicates that the contract can be queried with the associated message.
pub trait QueryableContract {
    /// Query message for the contract.
    type QueryMsg: Serialize + Debug;
}

/// Trait that indicates that the contract can be migrated with the associated message.
pub trait MigratableContract {
    /// Migrate message for the contract.
    type MigrateMsg: Serialize + Debug;
}

/// Smart contract execute entry point.
pub trait CwOrchExecute<Chain: TxHandler>: ExecutableContract + ContractInstance<Chain> {
    /// Send a ExecuteMsg to the contract.
    fn execute(
        &self,
        execute_msg: &Self::ExecuteMsg,
        coins: &[Coin],
    ) -> Result<Chain::Response, CwEnvError> {
        self.as_instance().execute(&execute_msg, coins)
    }
}

impl<T: ExecutableContract + ContractInstance<Chain>, Chain: TxHandler> CwOrchExecute<Chain> for T {}

/// Smart contract instantiate entry point.
pub trait CwOrchInstantiate<Chain: TxHandler>:
    InstantiableContract + ContractInstance<Chain>
{
    /// Instantiates the contract.
    fn instantiate(
        &self,
        instantiate_msg: &Self::InstantiateMsg,
        admin: Option<&Addr>,
        coins: &[Coin],
    ) -> Result<Chain::Response, CwEnvError> {
        self.as_instance()
            .instantiate(instantiate_msg, admin, coins)
    }

    /// Instantiates the contract using instantiate2
    fn instantiate2(
        &self,
        instantiate_msg: &Self::InstantiateMsg,
        admin: Option<&Addr>,
        coins: &[Coin],
        salt: Binary,
    ) -> Result<Chain::Response, CwEnvError> {
        self.as_instance()
            .instantiate2(instantiate_msg, admin, coins, salt)
    }
}

impl<T: InstantiableContract + ContractInstance<Chain>, Chain: TxHandler> CwOrchInstantiate<Chain>
    for T
{
}

/// Smart contract query entry point.
pub trait CwOrchQuery<Chain: QueryHandler + ChainState>:
    QueryableContract + ContractInstance<Chain>
{
    /// Query the contract.
    fn query<G: Serialize + DeserializeOwned + Debug>(
        &self,
        query_msg: &Self::QueryMsg,
    ) -> Result<G, CwEnvError> {
        self.as_instance().query(query_msg)
    }

    /// Query the contract raw state from an raw binary key
    fn raw_query(&self, query_keys: Vec<u8>) -> Result<Vec<u8>, CwEnvError> {
        self.environment()
            .wasm_querier()
            .raw_query(&self.address()?, query_keys)
            .map_err(Into::into)
    }

    /// Query the contract raw state from an cw-storage-plus::Item
    fn item_query<T: Serialize + DeserializeOwned>(
        &self,
        query_item: Item<T>,
    ) -> Result<T, CwEnvError> {
        self.environment()
            .wasm_querier()
            .item_query(&self.address()?, query_item)
    }

    /// Query the contract raw state from a cw-storage-plus::Map
    fn map_query<'a, T: Serialize + DeserializeOwned, K: PrimaryKey<'a>>(
        &self,
        query_map: Map<K, T>,
        key: K,
    ) -> Result<T, CwEnvError> {
        self.environment()
            .wasm_querier()
            .map_query(&self.address()?, query_map, key)
    }
}
/// Smart contract query entry point.
pub trait AsyncCwOrchQuery<Chain: AsyncWasmQuerier + ChainState>:
    QueryableContract + ContractInstance<Chain>
where
    <Self as QueryableContract>::QueryMsg: Sync,
{
    /// Query the contract.
    fn async_query<'a, G: Serialize + DeserializeOwned + Debug>(
        &'a self,
        query_msg: &Self::QueryMsg,
    ) -> impl std::future::Future<Output = Result<G, CwEnvError>> + Send
    where
        Chain: 'a,
    {
        let instance = self.as_instance();
        async { instance.async_query(query_msg).await }
    }
}

impl<Chain: ChainState, T: ?Sized + ContractInstance<Chain>> Environment<Chain> for T {
    fn environment(&self) -> &Chain {
        self.as_instance().environment()
    }
}

impl<T: QueryableContract + ContractInstance<Chain>, Chain: QueryHandler + ChainState>
    CwOrchQuery<Chain> for T
{
}

impl<T: QueryableContract + ContractInstance<Chain>, Chain: AsyncWasmQuerier + ChainState>
    AsyncCwOrchQuery<Chain> for T
where
    <T as QueryableContract>::QueryMsg: std::marker::Sync,
{
}

/// Smart contract migrate entry point.
pub trait CwOrchMigrate<Chain: TxHandler>: MigratableContract + ContractInstance<Chain> {
    /// Migrate the contract.
    fn migrate(
        &self,
        migrate_msg: &Self::MigrateMsg,
        new_code_id: u64,
    ) -> Result<Chain::Response, CwEnvError> {
        self.as_instance().migrate(migrate_msg, new_code_id)
    }
}

impl<T: MigratableContract + ContractInstance<Chain>, Chain: TxHandler> CwOrchMigrate<Chain> for T {}

/// Trait to implement on the contract to enable it to be uploaded
/// Should return [`WasmPath`](crate::contract::interface_traits::WasmPath) for `Chain = Daemon`
/// and [`Box<&dyn Contract>`] for `Chain = Mock`
pub trait Uploadable {
    /// Return an object that can be used to upload the contract to a WASM-supported environment.
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        unimplemented!("no wasm file provided for this contract")
    }

    /// Return the wrapper object for the contract, only works for non-custom mock environments
    fn wrapper() -> Box<dyn MockContract<Empty, Empty>> {
        unimplemented!("no wrapper function implemented for this contract")
    }
}

/// Trait that indicates that the contract can be uploaded.
pub trait CwOrchUpload<Chain: TxHandler>: ContractInstance<Chain> + Uploadable + Sized {
    /// upload the contract to the configured environment.
    fn upload(&self) -> Result<Chain::Response, CwEnvError> {
        self.as_instance().upload(self)
    }
}

/// enable `.upload()` for contracts that implement `Uploadable` for that environment.
impl<T: ContractInstance<Chain> + Uploadable, Chain: TxHandler> CwOrchUpload<Chain> for T {}

/// Enables calling a contract with a different sender.
///
/// Clones the contract interface to prevent mutation of the original.
pub trait CallAs<Chain: TxHandler>: CwOrchExecute<Chain> + ContractInstance<Chain> + Clone {
    fn set_sender(&mut self, sender: &<Chain as TxHandler>::Sender) {
        self.as_instance_mut().chain.set_sender(sender.clone())
    }
    /// Call a contract as a different sender.
    /// Clones the contract interface with a different sender.
    fn call_as(&self, sender: &<Chain as TxHandler>::Sender) -> Self {
        let mut contract = self.clone();
        contract.set_sender(sender);
        contract
    }
}

impl<T: CwOrchExecute<Chain> + ContractInstance<Chain> + Clone, Chain: TxHandler> CallAs<Chain>
    for T
{
}

/// Helper methods for conditional uploading of a contract.
pub trait ConditionalUpload<Chain: CwEnv>: CwOrchUpload<Chain> {
    /// Only upload the contract if it is not uploaded yet (checksum does not match)
    fn upload_if_needed(&self) -> Result<Option<TxResponse<Chain>>, CwEnvError> {
        if let Ok(true) = self.latest_is_uploaded() {
            Ok(None)
        } else {
            Some(self.upload()).transpose().map_err(Into::into)
        }
    }

    /// Returns whether the checksum of the WASM file matches the checksum of the latest uploaded code for this contract.
    fn latest_is_uploaded(&self) -> Result<bool, CwEnvError> {
        let Some(latest_uploaded_code_id) = self.code_id().ok() else {
            return Ok(false);
        };

        let chain = self.environment();
        let on_chain_hash = chain
            .wasm_querier()
            .code_id_hash(latest_uploaded_code_id)
            .map_err(Into::into)?;
        let local_hash = self.environment().wasm_querier().local_hash(self)?;

        Ok(local_hash == on_chain_hash)
    }

    /// Returns whether the contract is running the latest uploaded code for it
    fn is_running_latest(&self) -> Result<bool, CwEnvError> {
        let Some(latest_uploaded_code_id) = self.code_id().ok() else {
            return Ok(false);
        };
        let chain = self.environment();
        let info = chain
            .wasm_querier()
            .contract_info(&self.address()?)
            .map_err(Into::into)?;
        Ok(latest_uploaded_code_id == info.code_id)
    }
}

impl<T, Chain: CwEnv> ConditionalUpload<Chain> for T where T: CwOrchUpload<Chain> {}

/// Helper methods for conditional migration of a contract.
pub trait ConditionalMigrate<Chain: CwEnv>:
    CwOrchMigrate<Chain> + ConditionalUpload<Chain>
{
    /// Only migrate the contract if it is not on the latest code-id yet
    fn migrate_if_needed(
        &self,
        migrate_msg: &Self::MigrateMsg,
    ) -> Result<Option<TxResponse<Chain>>, CwEnvError> {
        if self.is_running_latest()? {
            log::info!(target: &contract_target(), "Skipped migration. {} is already running the latest code", self.id());
            Ok(None)
        } else {
            Some(self.migrate(migrate_msg, self.code_id()?))
                .transpose()
                .map_err(Into::into)
        }
    }
    /// Uploads the contract if the local contract hash is different from the latest on-chain code hash.
    /// Proceeds to migrates the contract if the contract is not running the latest code.
    fn upload_and_migrate_if_needed(
        &self,
        migrate_msg: &Self::MigrateMsg,
    ) -> Result<Option<Vec<TxResponse<Chain>>>, CwEnvError> {
        let mut txs = Vec::with_capacity(2);

        if let Some(tx) = self.upload_if_needed()? {
            txs.push(tx);
        };

        if let Some(tx) = self.migrate_if_needed(migrate_msg)? {
            txs.push(tx);
        };

        if txs.is_empty() {
            Ok(None)
        } else {
            Ok(Some(txs))
        }
    }
}
impl<T, Chain: CwEnv> ConditionalMigrate<Chain> for T where
    T: CwOrchMigrate<Chain> + ConditionalUpload<Chain>
{
}
