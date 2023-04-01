use crate::{contract::Contract, error::BootError, CwEnv};
use cosmwasm_std::{Addr, Coin};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

// Fn for custom implementation to return ContractInstance
pub trait ContractInstance<Chain: CwEnv> {
    fn as_instance(&self) -> &Contract<Chain>;
    fn as_instance_mut(&mut self) -> &mut Contract<Chain>;

    // Returns the contract id
    fn id(&self) -> String {
        self.as_instance().id.clone()
    }
    // State interfaces
    fn address(&self) -> Result<Addr, BootError> {
        Contract::address(self.as_instance())
    }
    fn addr_str(&self) -> Result<String, BootError> {
        Contract::address(self.as_instance()).map(|addr| addr.into_string())
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
    fn get_chain(&self) -> &Chain {
        Contract::get_chain(self.as_instance())
    }
}

/// Tells BOOT what the contract's entrypoint messages are.
pub trait CwInterface {
    type InstantiateMsg: Serialize + Debug;
    type ExecuteMsg: Serialize + Debug;
    type QueryMsg: Serialize + Debug;
    type MigrateMsg: Serialize + Debug;
}

/// Smart Contract execute endpoint
pub trait BootExecute<Chain: CwEnv> {
    type ExecuteMsg: Serialize;

    fn execute(
        &self,
        execute_msg: &Self::ExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, BootError>;
}

impl<T: CwInterface + ContractInstance<Chain>, Chain: CwEnv> BootExecute<Chain> for T {
    type ExecuteMsg = <T as CwInterface>::ExecuteMsg;

    fn execute(
        &self,
        execute_msg: &Self::ExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, BootError> {
        self.as_instance().execute(&execute_msg, coins)
    }
}

/// Smart Contract instantiate endpoint
pub trait BootInstantiate<Chain: CwEnv> {
    type InstantiateMsg: Serialize;

    fn instantiate(
        &self,
        instantiate_msg: &Self::InstantiateMsg,
        admin: Option<&Addr>,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, BootError>;
}

impl<T: CwInterface + ContractInstance<Chain>, Chain: CwEnv> BootInstantiate<Chain> for T {
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
pub trait BootQuery<Chain: CwEnv> {
    type QueryMsg: Serialize;

    fn query<G: Serialize + DeserializeOwned + Debug>(
        &self,
        query_msg: &Self::QueryMsg,
    ) -> Result<G, BootError>;
}

impl<T: CwInterface + ContractInstance<Chain>, Chain: CwEnv> BootQuery<Chain> for T {
    type QueryMsg = <T as CwInterface>::QueryMsg;

    fn query<G: Serialize + DeserializeOwned + Debug>(
        &self,
        query_msg: &Self::QueryMsg,
    ) -> Result<G, BootError> {
        self.as_instance().query(query_msg)
    }
}

/// Smart Contract migrate endpoint
pub trait BootMigrate<Chain: CwEnv> {
    type MigrateMsg: Serialize;

    fn migrate(
        &self,
        migrate_msg: &Self::MigrateMsg,
        new_code_id: u64,
    ) -> Result<Chain::Response, BootError>;
}

impl<T: CwInterface + ContractInstance<Chain>, Chain: CwEnv> BootMigrate<Chain> for T {
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

pub trait BootUpload<Chain: CwEnv> {
    fn upload(&mut self) -> Result<Chain::Response, BootError>;
}

impl<T: ContractInstance<Chain>, Chain: CwEnv> BootUpload<Chain> for T {
    fn upload(&mut self) -> Result<Chain::Response, BootError> {
        self.as_instance_mut().upload()
    }
}

/// Call a contract with a different sender
/// Clones the contract interface to prevent mutation of the original
pub trait CallAs<Chain: CwEnv>: BootExecute<Chain> + ContractInstance<Chain> + Clone {
    type Sender: Clone;

    /// Set the sender for the contract
    fn set_sender(&mut self, sender: &Self::Sender);

    /// Call a contract as a different sender.  
    /// Creates a new copy of the contract with a different sender
    fn call_as(&self, sender: &Self::Sender) -> Self;
}
