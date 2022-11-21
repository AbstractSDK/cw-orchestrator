use std::fmt::Debug;

use crate::{contract::Contract, error::BootError, BootEnvironment};
use cosmwasm_std::{Addr, Coin};
use serde::{de::DeserializeOwned, Serialize};

// Fn for custom implementation to return ContractInstance
pub trait ContractInstance<Chain: BootEnvironment> {
    fn as_instance(&self) -> &Contract<Chain>;
    fn as_instance_mut(&mut self) -> &mut Contract<Chain>;

    // State interfaces
    fn address(&self) -> Result<Addr, BootError> {
        Contract::address(self.as_instance())
    }
    fn code_id(&self) -> Result<u64, BootError> {
        Contract::code_id(self.as_instance())
    }
    fn set_address(&self, address: &Addr) {
        Contract::set_address(self.as_instance(), address)
    }
    fn set_code_id(&self, code_id: u64) {
        Contract::set_code_id(self.as_instance(), code_id)
    }
    fn get_chain(&self) -> Chain {
        Contract::get_chain(self.as_instance())
    }
}

/// Implementing CwInterface ensures type safety
pub trait CwInterface {
    type InstantiateMsg: Serialize + Debug;
    type ExecuteMsg: Serialize + Debug;
    type QueryMsg: Serialize + Debug;
    type MigrateMsg: Serialize + Debug;
}

/// Smart Contract execute endpoint
pub trait BootExecute<Chain: BootEnvironment> {
    type ExecuteMsg: Serialize;

    fn execute<'a>(
        &self,
        execute_msg: &'a Self::ExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, BootError>;
}

impl<T: CwInterface + ContractInstance<Chain>, Chain: BootEnvironment> BootExecute<Chain> for T {
    type ExecuteMsg = <T as CwInterface>::ExecuteMsg;

    fn execute<'a>(
        &self,
        execute_msg: &'a Self::ExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, BootError> {
        self.as_instance().execute(&execute_msg, coins)
    }
}

/// Smart Contract instantiate endpoint
pub trait BootInstantiate<Chain: BootEnvironment> {
    type InstantiateMsg: Serialize;

    fn instantiate(
        &self,
        instantiate_msg: &Self::InstantiateMsg,
        admin: Option<&Addr>,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, BootError>;
}

impl<T: CwInterface + ContractInstance<Chain>, Chain: BootEnvironment> BootInstantiate<Chain>
    for T
{
    type InstantiateMsg = <T as CwInterface>::InstantiateMsg;

    fn instantiate(
        &self,
        instantiate_msg: &Self::InstantiateMsg,
        admin: Option<&Addr>,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, BootError> {
        self.as_instance()
            .instantiate(instantiate_msg, admin, coins)
    }
}

/// Smart Contract query endpoint
pub trait BootQuery<Chain: BootEnvironment> {
    type QueryMsg: Serialize;

    fn query<G: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Self::QueryMsg,
    ) -> Result<G, BootError>;
}

impl<T: CwInterface + ContractInstance<Chain>, Chain: BootEnvironment> BootQuery<Chain> for T {
    type QueryMsg = <T as CwInterface>::QueryMsg;

    fn query<G: Serialize + DeserializeOwned>(
        &self,
        query_msg: &Self::QueryMsg,
    ) -> Result<G, BootError> {
        self.as_instance().query(query_msg)
    }
}

/// Smart Contract migrate endpoint
pub trait BootMigrate<Chain: BootEnvironment> {
    type MigrateMsg: Serialize;

    fn migrate(
        &self,
        migrate_msg: &Self::MigrateMsg,
        new_code_id: u64,
    ) -> Result<Chain::Response, BootError>;
}

impl<T: CwInterface + ContractInstance<Chain>, Chain: BootEnvironment> BootMigrate<Chain> for T {
    type MigrateMsg = <T as CwInterface>::MigrateMsg;

    fn migrate(
        &self,
        migrate_msg: &Self::MigrateMsg,
        new_code_id: u64,
    ) -> Result<Chain::Response, BootError> {
        self.as_instance().migrate(migrate_msg, new_code_id)
    }
}

/// Smart Contract migrate endpoint

pub trait BootUpload<Chain: BootEnvironment> {
    fn upload(&mut self) -> Result<Chain::Response, BootError>;
}

impl<T: ContractInstance<Chain>, Chain: BootEnvironment> BootUpload<Chain> for T {
    fn upload(&mut self) -> Result<Chain::Response, BootError> {
        self.as_instance_mut().upload()
    }
}
