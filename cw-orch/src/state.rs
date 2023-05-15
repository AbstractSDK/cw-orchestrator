use crate::error::CwOrchError;
use cosmwasm_std::Addr;
use std::collections::HashMap;

/// State accessor trait
/// Indicates that the type has access to an underlying state
pub trait ChainState {
    type Out: StateInterface;
    fn state(&self) -> Self::Out;
}

/// This Interface allows interacting with the local state file of the deployment of any chain environment (mock or daemon)
pub trait StateInterface: Clone {
    /// get the address a contract using the specified contract id
    fn get_address(&self, contract_id: &str) -> Result<Addr, CwOrchError>;
    /// set the address of a contract using the specified contract id
    fn set_address(&mut self, contract_id: &str, address: &Addr);
    /// get the code id if of a contract with the specified contract id
    fn get_code_id(&self, contract_id: &str) -> Result<u64, CwOrchError>;
    /// set the code id if of a contract with the specified contract id
    fn set_code_id(&mut self, contract_id: &str, code_id: u64);
    /// get all addresses related to this deployment
    fn get_all_addresses(&self) -> Result<HashMap<String, Addr>, CwOrchError>;
    /// get all codes related to this deployment
    fn get_all_code_ids(&self) -> Result<HashMap<String, u64>, CwOrchError>;
}
