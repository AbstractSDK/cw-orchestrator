use crate::error::BootError;
use crate::state::StateInterface;

use cosmwasm_std::Addr;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MockState {
    pub code_ids: HashMap<String, u64>,
    pub addresses: HashMap<String, Addr>,
}

impl MockState {
    pub fn new() -> Self {
        Self {
            addresses: HashMap::new(),
            code_ids: HashMap::new(),
        }
    }
}

impl Default for MockState {
    fn default() -> Self {
        Self::new()
    }
}

impl StateInterface for MockState {
    fn get_address(&self, contract_id: &str) -> Result<Addr, BootError> {
        self.addresses
            .get(contract_id)
            .ok_or_else(|| BootError::AddrNotInFile(contract_id.to_owned()))
            .map(|val| val.to_owned())
    }

    fn set_address(&mut self, contract_id: &str, address: &Addr) {
        self.addresses
            .insert(contract_id.to_string(), address.to_owned());
    }

    /// Get the locally-saved version of the contract's version on this network
    fn get_code_id(&self, contract_id: &str) -> Result<u64, BootError> {
        self.code_ids
            .get(contract_id)
            .ok_or_else(|| BootError::CodeIdNotInFile(contract_id.to_owned()))
            .map(|val| val.to_owned())
    }
    /// Set the locally-saved version of the contract's latest version on this network
    fn set_code_id(&mut self, contract_id: &str, code_id: u64) {
        self.code_ids.insert(contract_id.to_string(), code_id);
    }
    fn get_all_addresses(&self) -> Result<HashMap<String, Addr>, BootError> {
        Ok(self.addresses.clone())
    }
    fn get_all_code_ids(&self) -> Result<HashMap<String, u64>, BootError> {
        Ok(self.code_ids.clone())
    }
}
