use crate::daemon::DaemonState;

use crate::{
    error::CwOrchError,
    state::{DeployDetails, StateInterface},
};

use cosmwasm_std::Addr;

use std::{collections::HashMap, rc::Rc};

impl StateInterface for Rc<DaemonState> {
    /// Read address for contract in deployment id from state file
    fn get_address(&self, contract_id: &str) -> Result<Addr, CwOrchError> {
        let value = self
            .get(&self.deployment_id)
            .get(contract_id)
            .ok_or_else(|| CwOrchError::AddrNotInStore(contract_id.to_owned()))?
            .clone();
        Ok(Addr::unchecked(value.as_str().unwrap()))
    }

    /// Set address for contract in deployment id in state file
    fn set_address(&mut self, contract_id: &str, address: &Addr) {
        self.set(&self.deployment_id, contract_id, address.as_str());
    }

    /// Get the locally-saved version of the contract's version on this network
    fn get_code_id(&self, contract_id: &str) -> Result<u64, CwOrchError> {
        let value = self
            .get("code_ids")
            .get(contract_id)
            .ok_or_else(|| CwOrchError::CodeIdNotInStore(contract_id.to_owned()))?
            .clone();
        Ok(value.as_u64().unwrap())
    }

    /// Set the locally-saved version of the contract's latest version on this network
    fn set_code_id(&mut self, contract_id: &str, code_id: u64) {
        self.set("code_ids", contract_id, code_id);
    }

    /// Get all addresses for deployment id from state file
    fn get_all_addresses(&self) -> Result<HashMap<String, Addr>, CwOrchError> {
        let mut store = HashMap::new();
        let addresses = self.get(&self.deployment_id);
        let value = addresses.as_object().unwrap();
        for (id, addr) in value {
            store.insert(id.clone(), Addr::unchecked(addr.as_str().unwrap()));
        }
        Ok(store)
    }

    fn get_all_code_ids(&self) -> Result<HashMap<String, u64>, CwOrchError> {
        let mut store = HashMap::new();
        let code_ids = self.get("code_ids");
        let value = code_ids.as_object().unwrap();
        for (id, code_id) in value {
            store.insert(id.clone(), code_id.as_u64().unwrap());
        }
        Ok(store)
    }

    fn deploy_details(&self) -> DeployDetails {
        DeployDetails {
            chain_id: self.chain_data.chain_id.to_string(),
            chain_name: self.chain_data.chain_name.clone(),
            deployment_id: self.deployment_id.clone(),
        }
    }
}
