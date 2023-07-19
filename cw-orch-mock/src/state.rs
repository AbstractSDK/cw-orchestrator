use cosmwasm_std::{testing::mock_env, Addr};
use cw_orch_environment::{
    environment::{DeployDetails, StateInterface},
    CwEnvError,
};

use std::collections::HashMap;

#[derive(Clone, Debug)]
/// Mock state for testing, stores addresses and code-ids.
pub struct MockState {
    /// Deployed contract code ids
    pub code_ids: HashMap<String, u64>,
    /// Deployed contract addresses
    pub addresses: HashMap<String, Addr>,
}

impl MockState {
    /// Creates a new empty mock state
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
    fn get_address(&self, contract_id: &str) -> Result<Addr, CwEnvError> {
        self.addresses
            .get(contract_id)
            .ok_or_else(|| CwEnvError::AddrNotInStore(contract_id.to_owned()))
            .map(|val| val.to_owned())
    }

    fn set_address(&mut self, contract_id: &str, address: &Addr) {
        self.addresses
            .insert(contract_id.to_string(), address.to_owned());
    }

    /// Get the locally-saved version of the contract's version on this network
    fn get_code_id(&self, contract_id: &str) -> Result<u64, CwEnvError> {
        self.code_ids
            .get(contract_id)
            .ok_or_else(|| CwEnvError::CodeIdNotInStore(contract_id.to_owned()))
            .map(|val| val.to_owned())
    }

    /// Set the locally-saved version of the contract's latest version on this network
    fn set_code_id(&mut self, contract_id: &str, code_id: u64) {
        self.code_ids.insert(contract_id.to_string(), code_id);
    }

    fn get_all_addresses(&self) -> Result<HashMap<String, Addr>, CwEnvError> {
        Ok(self.addresses.clone())
    }

    fn get_all_code_ids(&self) -> Result<HashMap<String, u64>, CwEnvError> {
        Ok(self.code_ids.clone())
    }

    fn deploy_details(&self) -> DeployDetails {
        let chain_id: String = mock_env().block.chain_id;
        let chain_name: String = chain_id.rsplitn(2, '-').collect::<Vec<_>>()[1].to_string();
        DeployDetails {
            chain_id,
            chain_name,
            deployment_id: "default".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::Addr;
    use cw_orch_environment::{environment::StateInterface, CwEnvError};
    use speculoos::prelude::*;

    use super::MockState;

    const CONTRACT_ID: &str = "123";
    const CONTRACT_ADDR: &str = "cosmos123";

    #[test]
    fn mock_state() {
        let mut mock = MockState::default();

        let unchecked_address = &Addr::unchecked(CONTRACT_ADDR);
        let code_id = 123u64;

        mock.set_address(CONTRACT_ID, unchecked_address);
        mock.set_code_id(CONTRACT_ID, code_id);

        // assert we get the right address
        let addr = mock.get_address(CONTRACT_ID).unwrap();
        asserting!(&"address is correct for contract_id")
            .that(unchecked_address)
            .is_equal_to(&addr);

        // assert we get the right code_id
        let fetched_id = mock.get_code_id(CONTRACT_ID).unwrap();
        asserting!(&"code_id is correct for contract_id")
            .that(&fetched_id)
            .is_equal_to(code_id);

        // assert we get AddrNotInStore error
        let missing_id = &"456";
        let error = mock.get_address(missing_id).unwrap_err();
        let error_msg = CwEnvError::AddrNotInStore(String::from(*missing_id)).to_string();
        asserting!(&(format!("Asserting we get CwEnvError: {}", error_msg)))
            .that(&error.to_string())
            .is_equal_to(CwEnvError::AddrNotInStore(String::from(*missing_id)).to_string());

        // assert we get CodeIdNotInStore error
        let error_msg = CwEnvError::CodeIdNotInStore(String::from(*missing_id)).to_string();
        let error = mock.get_code_id(missing_id).unwrap_err();
        asserting!(&(format!("Asserting we get CwEnvError: {}", error_msg)))
            .that(&error.to_string())
            .is_equal_to(CwEnvError::CodeIdNotInStore(String::from(*missing_id)).to_string());

        // validate we can get all addresses
        let total = mock.get_all_addresses().unwrap().len();
        asserting!(&"total addresses is one")
            .that(&total)
            .is_equal_to(1);

        // validate we can get all code_ids
        let total = mock.get_all_code_ids().unwrap().len();
        asserting!(&"total code_ids is one")
            .that(&total)
            .is_equal_to(1)
    }
}
