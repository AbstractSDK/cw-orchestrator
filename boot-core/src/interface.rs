use std::fmt::Debug;

use crate::{contract::Contract, error::BootError, BootEnvironment};
use cosmwasm_std::{Addr, Coin};
use serde::{de::DeserializeOwned, Serialize};

// Fn for custom implementation to return ContractInstance
pub trait ContractInstance<Chain: BootEnvironment> {
    fn instance(&self) -> &Contract<Chain>;
    fn instance_mut(&mut self) -> &mut Contract<Chain>;

    // State interfaces
    fn address(&self) -> Result<Addr, BootError> {
        Contract::address(self.instance())
    }
    fn code_id(&self) -> Result<u64, BootError> {
        Contract::code_id(self.instance())
    }
    fn set_address(&self, address: &Addr) {
        Contract::set_address(self.instance(), address)
    }
    fn set_code_id(&self, code_id: u64) {
        Contract::set_code_id(self.instance(), code_id)
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
    type E: Serialize;

    fn execute<'a>(
        &self,
        execute_msg: &'a Self::E,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, BootError>;
}

impl<T: CwInterface + ContractInstance<Chain>, Chain: BootEnvironment> BootExecute<Chain> for T {
    type E = <T as CwInterface>::ExecuteMsg;

    fn execute<'a>(
        &self,
        execute_msg: &'a Self::E,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, BootError> {
        self.instance().execute(&execute_msg, coins)
    }
}

/// Smart Contract instantiate endpoint
pub trait BootInstantiate<Chain: BootEnvironment> {
    type I: Serialize;

    fn instantiate(
        &self,
        instantiate_msg: &Self::I,
        admin: Option<&Addr>,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, BootError>;
}

impl<T: CwInterface + ContractInstance<Chain>, Chain: BootEnvironment> BootInstantiate<Chain>
    for T
{
    type I = <T as CwInterface>::InstantiateMsg;

    fn instantiate(
        &self,
        instantiate_msg: &Self::I,
        admin: Option<&Addr>,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, BootError> {
        self.instance().instantiate(instantiate_msg, admin, coins)
    }
}

/// Smart Contract query endpoint
pub trait BootQuery<Chain: BootEnvironment> {
    type Q: Serialize;

    fn query<G: Serialize + DeserializeOwned>(&self, query_msg: &Self::Q) -> Result<G, BootError>;
}

impl<T: CwInterface + ContractInstance<Chain>, Chain: BootEnvironment> BootQuery<Chain> for T {
    type Q = <T as CwInterface>::QueryMsg;

    fn query<G: Serialize + DeserializeOwned>(&self, query_msg: &Self::Q) -> Result<G, BootError> {
        self.instance().query(query_msg)
    }
}

/// Smart Contract migrate endpoint
pub trait BootMigrate<Chain: BootEnvironment> {
    type M: Serialize;

    fn migrate(
        &self,
        migrate_msg: &Self::M,
        new_code_id: u64,
    ) -> Result<Chain::Response, BootError>;
}

impl<T: CwInterface + ContractInstance<Chain>, Chain: BootEnvironment> BootMigrate<Chain> for T {
    type M = <T as CwInterface>::MigrateMsg;

    fn migrate(
        &self,
        migrate_msg: &Self::M,
        new_code_id: u64,
    ) -> Result<Chain::Response, BootError> {
        self.instance().migrate(migrate_msg, new_code_id)
    }
}

/// Smart Contract migrate endpoint

pub trait BootUpload<Chain: BootEnvironment> {
    fn upload(&mut self) -> Result<Chain::Response, BootError>;
}

impl<T: ContractInstance<Chain>, Chain: BootEnvironment> BootUpload<Chain> for T {
    fn upload(&mut self) -> Result<Chain::Response, BootError> {
        self.instance_mut().upload()
    }
}
