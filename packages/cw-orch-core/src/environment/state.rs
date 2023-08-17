//! State interfaces for execution environments.

use crate::error::CwEnvError;
use cosmwasm_std::Addr;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

/// State accessor trait.
/// Indicates that the type has access to an underlying state.
pub trait ChainState {
    /// The type of the underlying state.
    type Out: StateInterface;
    /// Get the underlying state.
    fn state(&self) -> Self::Out;
}

/// This Interface allows for managing the local state of a deployment on any CosmWasm-supported environment.
pub trait StateInterface: Clone {
    /// Get the address of a contract using the specified contract id.
    fn get_address(&self, contract_id: &str) -> Result<Addr, CwEnvError>;

    /// Set the address of a contract using the specified contract id.
    fn set_address(&mut self, contract_id: &str, address: &Addr);

    /// Get the code id for a contract with the specified contract id.
    fn get_code_id(&self, contract_id: &str) -> Result<u64, CwEnvError>;

    /// Set the code id for a contract with the specified contract id.
    fn set_code_id(&mut self, contract_id: &str, code_id: u64);

    /// Get all addresses related to this deployment.
    fn get_all_addresses(&self) -> Result<HashMap<String, Addr>, CwEnvError>;

    /// Get all codes related to this deployment.
    fn get_all_code_ids(&self) -> Result<HashMap<String, u64>, CwEnvError>;

    /// Get some details used for deployment on the current chain
    /// This is used for
    fn deploy_details(&self) -> DeployDetails;
}

/// Details about the chain and env you are deploying on
pub struct DeployDetails {
    /// E.g. juno-2
    pub chain_id: String,
    /// E.g. juno
    pub chain_name: String,
    /// E.g. default
    pub deployment_id: String,
}

impl<S: StateInterface> StateInterface for Rc<RefCell<S>> {
    fn get_address(&self, contract_id: &str) -> Result<Addr, CwEnvError> {
        (**self).borrow().get_address(contract_id)
    }

    fn set_address(&mut self, contract_id: &str, address: &Addr) {
        (**self).borrow_mut().set_address(contract_id, address)
    }

    fn get_code_id(&self, contract_id: &str) -> Result<u64, CwEnvError> {
        (**self).borrow().get_code_id(contract_id)
    }

    fn set_code_id(&mut self, contract_id: &str, code_id: u64) {
        (**self).borrow_mut().set_code_id(contract_id, code_id)
    }

    fn get_all_addresses(&self) -> Result<HashMap<String, Addr>, CwEnvError> {
        (**self).borrow().get_all_addresses()
    }

    fn get_all_code_ids(&self) -> Result<HashMap<String, u64>, CwEnvError> {
        (**self).borrow().get_all_code_ids()
    }

    fn deploy_details(&self) -> DeployDetails {
        (**self).borrow().deploy_details()
    }
}
