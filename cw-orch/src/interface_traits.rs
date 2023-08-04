use crate::{
    contract::Contract,
    error::CwOrchError,
    prelude::{CwEnv, TxHandler, WasmPath},
};
use cosmwasm_std::{Addr, Coin, Empty};
use cw_multi_test::Contract as MockContract;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

// Fn for custom implementation to return ContractInstance
/// Interface to the underlying `Contract` struct. Implemented automatically when using our macros.
pub trait ContractInstance<Chain: CwEnv> {
    /// Return a reference to the underlying contract instance.
    fn as_instance(&self) -> &Contract<Chain>;

    /// Return a mutable reference to the underlying contract instance.
    fn as_instance_mut(&mut self) -> &mut Contract<Chain>;

    /// Returns the contract id.
    fn id(&self) -> String {
        self.as_instance().id.clone()
    }

    /// Returns the contract address for this instance.
    fn address(&self) -> Result<Addr, CwOrchError> {
        Contract::address(self.as_instance())
    }

    /// Returns the contract address as a [`String`].
    fn addr_str(&self) -> Result<String, CwOrchError> {
        Contract::address(self.as_instance()).map(|addr| addr.into_string())
    }

    /// Returns contract code_id.
    fn code_id(&self) -> Result<u64, CwOrchError> {
        Contract::code_id(self.as_instance())
    }

    /// Sets the address for the contract. Useful when the contract is already initialized
    /// and not registered in the configured state file.
    fn set_address(&self, address: &Addr) {
        Contract::set_address(self.as_instance(), address)
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

    /// Sets a default address for the contract. If the contract already has an address registered in the state, this won't be used.
    /// This is mostly used to ship address with a cw-orch package.
    fn set_default_code_id(&mut self, code_id: u64) {
        Contract::set_default_code_id(self.as_instance_mut(), code_id)
    }

    /// Returns the chain that this contract is deployed on.
    fn get_chain(&self) -> &Chain {
        Contract::get_chain(self.as_instance())
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
pub trait CwOrchExecute<Chain: CwEnv>: ExecutableContract + ContractInstance<Chain> {
    /// Send a ExecuteMsg to the contract.
    fn execute(
        &self,
        execute_msg: &Self::ExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, CwOrchError> {
        self.as_instance().execute(&execute_msg, coins)
    }
}

impl<T: ExecutableContract + ContractInstance<Chain>, Chain: CwEnv> CwOrchExecute<Chain> for T {}

/// Smart contract instantiate entry point.
pub trait CwOrchInstantiate<Chain: CwEnv>: InstantiableContract + ContractInstance<Chain> {
    /// Instantiates the contract.
    fn instantiate(
        &self,
        instantiate_msg: &Self::InstantiateMsg,
        admin: Option<&Addr>,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, CwOrchError> {
        self.as_instance()
            .instantiate(instantiate_msg, admin, coins)
    }
}

impl<T: InstantiableContract + ContractInstance<Chain>, Chain: CwEnv> CwOrchInstantiate<Chain>
    for T
{
}

/// Smart contract query entry point.
pub trait CwOrchQuery<Chain: CwEnv>: QueryableContract + ContractInstance<Chain> {
    /// Query the contract.
    fn query<G: Serialize + DeserializeOwned + Debug>(
        &self,
        query_msg: &Self::QueryMsg,
    ) -> Result<G, CwOrchError> {
        self.as_instance().query(query_msg)
    }
}

impl<T: QueryableContract + ContractInstance<Chain>, Chain: CwEnv> CwOrchQuery<Chain> for T {}

/// Smart contract migrate entry point.
pub trait CwOrchMigrate<Chain: CwEnv>: MigratableContract + ContractInstance<Chain> {
    /// Migrate the contract.
    fn migrate(
        &self,
        migrate_msg: &Self::MigrateMsg,
        new_code_id: u64,
    ) -> Result<Chain::Response, CwOrchError> {
        self.as_instance().migrate(migrate_msg, new_code_id)
    }
}

impl<T: MigratableContract + ContractInstance<Chain>, Chain: CwEnv> CwOrchMigrate<Chain> for T {}

/// Trait to implement on the contract to enable it to be uploaded
/// Should return [`WasmPath`](crate::prelude::WasmPath) for `Chain = Daemon`
/// and [`Box<&dyn Contract>`] for `Chain = Mock`
pub trait Uploadable<ExecT = Empty, QueryT = Empty> {
    /// Return an object that can be used to upload the contract to a WASM-supported environment.
    fn wasm(&self) -> WasmPath {
        unimplemented!("no wasm file provided for this contract")
    }

    /// Return the wrapper object for the contract, only works for non-custom mock environments
    fn wrapper(&self) -> Box<dyn MockContract<ExecT, QueryT>> {
        unimplemented!("no wrapper function implemented for this contract")
    }
}

/// Trait that indicates that the contract can be uploaded.
pub trait CwOrchUpload<Chain: CwEnv>:
    ContractInstance<Chain>
    + Uploadable<<Chain as TxHandler>::ExecC, <Chain as TxHandler>::QueryC>
    + Sized
{
    /// upload the contract to the configured environment.
    fn upload(&self) -> Result<Chain::Response, CwOrchError> {
        self.as_instance().upload(self)
    }
}

/// enable `.upload()` for contracts that implement `Uploadable` for that environment.
impl<
        T: ContractInstance<Chain>
            + Uploadable<<Chain as TxHandler>::ExecC, <Chain as TxHandler>::QueryC>,
        Chain: CwEnv,
    > CwOrchUpload<Chain> for T
{
}

/// Enables calling a contract with a different sender.
///
/// Clones the contract interface to prevent mutation of the original.
pub trait CallAs<Chain: CwEnv>: CwOrchExecute<Chain> + ContractInstance<Chain> + Clone {
    /// The sender type for environment
    type Sender: Clone;

    /// Set the sender for interactions with the contract.
    fn set_sender(&mut self, sender: &Self::Sender);

    /// Call a contract as a different sender.
    /// Clones the contract interface with a different sender.
    fn call_as(&self, sender: &Self::Sender) -> Self;
}
