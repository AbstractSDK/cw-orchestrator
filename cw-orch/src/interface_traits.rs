use crate::{contract::Contract, error::CwOrcError, CwEnv, TxHandler};
use cosmwasm_std::{Addr, Coin};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

// Fn for custom implementation to return ContractInstance
pub trait ContractInstance<Chain: CwEnv> {
    /// Return a reference to the underlying Contract instance
    fn as_instance(&self) -> &Contract<Chain>;
    /// Return a mutable reference to the underlying Contract instance
    fn as_instance_mut(&mut self) -> &mut Contract<Chain>;

    // Returns the contract id
    fn id(&self) -> String {
        self.as_instance().id.clone()
    }
    // State interfaces
    fn address(&self) -> Result<Addr, CwOrcError> {
        Contract::address(self.as_instance())
    }
    fn addr_str(&self) -> Result<String, CwOrcError> {
        Contract::address(self.as_instance()).map(|addr| addr.into_string())
    }
    fn code_id(&self) -> Result<u64, CwOrcError> {
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

/// Tells cw-orchestrator what the contract's entrypoint messages are.
pub trait ExecuteableContract {
    type ExecuteMsg: Serialize + Debug;
}
pub trait InstantiateableContract {
    type InstantiateMsg: Serialize + Debug;
}
pub trait QueryableContract {
    type QueryMsg: Serialize + Debug;
}
pub trait MigrateableContract {
    type MigrateMsg: Serialize + Debug;
}

/// Smart Contract execute endpoint
pub trait CwOrcExecute<Chain: CwEnv>: ExecuteableContract + ContractInstance<Chain> {
    fn execute(
        &self,
        execute_msg: &Self::ExecuteMsg,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, CwOrcError> {
        self.as_instance().execute(&execute_msg, coins)
    }
}

impl<T: ExecuteableContract + ContractInstance<Chain>, Chain: CwEnv> CwOrcExecute<Chain> for T {}

/// Smart Contract instantiate endpoint
pub trait CwOrcInstantiate<Chain: CwEnv>:
    InstantiateableContract + ContractInstance<Chain>
{
    fn instantiate(
        &self,
        instantiate_msg: &Self::InstantiateMsg,
        admin: Option<&Addr>,
        coins: Option<&[Coin]>,
    ) -> Result<Chain::Response, CwOrcError> {
        self.as_instance()
            .instantiate(instantiate_msg, admin, coins)
    }
}

impl<T: InstantiateableContract + ContractInstance<Chain>, Chain: CwEnv> CwOrcInstantiate<Chain>
    for T
{
}

/// Smart Contract query endpoint
pub trait CwOrcQuery<Chain: CwEnv>: QueryableContract + ContractInstance<Chain> {
    fn query<G: Serialize + DeserializeOwned + Debug>(
        &self,
        query_msg: &Self::QueryMsg,
    ) -> Result<G, CwOrcError> {
        self.as_instance().query(query_msg)
    }
}

impl<T: QueryableContract + ContractInstance<Chain>, Chain: CwEnv> CwOrcQuery<Chain> for T {}

/// Smart Contract migrate endpoint
pub trait CwOrcMigrate<Chain: CwEnv>: MigrateableContract + ContractInstance<Chain> {
    fn migrate(
        &self,
        migrate_msg: &Self::MigrateMsg,
        new_code_id: u64,
    ) -> Result<Chain::Response, CwOrcError> {
        self.as_instance().migrate(migrate_msg, new_code_id)
    }
}

impl<T: MigrateableContract + ContractInstance<Chain>, Chain: CwEnv> CwOrcMigrate<Chain> for T {}

/// Trait to implement on the contract to enable it to be uploaded
/// Should return [`WasmPath`](crate::WasmPath) for `Chain = Daemon`
/// and [`Box<&dyn Contract>`] for `Chain = Mock`
pub trait Uploadable<Chain: TxHandler> {
    /// Return an object that can be used to upload the contract to the environment.
    fn source(&self) -> Chain::ContractSource;
}

/// Smart Contract upload endpoint
pub trait CwOrcUpload<Chain: CwEnv>: ContractInstance<Chain> + Uploadable<Chain> {
    fn upload(&self) -> Result<Chain::Response, CwOrcError> {
        self.as_instance().upload(self.source())
    }
}

/// enable `.upload()` for contracts that implement `Uploadable` for that environment.
impl<T: ContractInstance<Chain> + Uploadable<Chain>, Chain: CwEnv> CwOrcUpload<Chain> for T {}

/// Call a contract with a different sender
/// Clones the contract interface to prevent mutation of the original
pub trait CallAs<Chain: CwEnv>: CwOrcExecute<Chain> + ContractInstance<Chain> + Clone {
    type Sender: Clone;

    /// Set the sender for the contract
    fn set_sender(&mut self, sender: &Self::Sender);

    /// Call a contract as a different sender.  
    /// Creates a new copy of the contract with a different sender
    fn call_as(&self, sender: &Self::Sender) -> Self;
}
