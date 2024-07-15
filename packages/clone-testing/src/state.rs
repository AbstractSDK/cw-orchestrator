use cosmwasm_std::Addr;
use cw_orch_core::{
    environment::{ChainInfoOwned, StateInterface},
    CwEnvError,
};
use cw_orch_daemon::DaemonState;
use itertools::Itertools;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone, Debug)]
/// Mock state for testing, stores addresses and code-ids.
pub struct MockState {
    /// Deployed contract code ids
    pub code_ids: HashMap<String, u64>,
    /// Deployed contract addresses
    pub addresses: HashMap<String, Addr>,
    /// State read from file. Used to actually integrate with actual deployments
    pub daemon_state: DaemonState,
}

impl MockState {
    /// Creates a new empty mock state
    pub fn new(chain: ChainInfoOwned, deployment_id: &str) -> Self {
        Self {
            addresses: HashMap::new(),
            code_ids: HashMap::new(),
            daemon_state: DaemonState::new(
                DaemonState::state_file_path().unwrap(),
                &Arc::new(chain),
                deployment_id.to_string(),
                true,
                false,
            )
            .unwrap(),
        }
    }
}

impl StateInterface for MockState {
    fn get_address(&self, contract_id: &str) -> Result<Addr, CwEnvError> {
        // First we look for the address inside the mock state
        self.addresses
            .get(contract_id)
            .ok_or_else(|| CwEnvError::AddrNotInStore(contract_id.to_owned()))
            .map(|val| val.to_owned())
            // If not present, we look for it in the daemon state
            .or_else(|_| self.daemon_state.get_address(contract_id))
    }

    fn set_address(&mut self, contract_id: &str, address: &Addr) {
        self.addresses
            .insert(contract_id.to_string(), address.to_owned());
    }

    fn remove_address(&mut self, contract_id: &str) {
        self.addresses.remove(contract_id);
    }

    /// Get the locally-saved version of the contract's version on this network
    fn get_code_id(&self, contract_id: &str) -> Result<u64, CwEnvError> {
        self.code_ids
            .get(contract_id)
            .ok_or_else(|| CwEnvError::CodeIdNotInStore(contract_id.to_owned()))
            .map(|val| val.to_owned())
            // If not present, we look for it in the daemon state
            .or_else(|_| self.daemon_state.get_code_id(contract_id))
    }

    /// Set the locally-saved version of the contract's latest version on this network
    fn set_code_id(&mut self, contract_id: &str, code_id: u64) {
        self.code_ids.insert(contract_id.to_string(), code_id);
    }

    fn remove_code_id(&mut self, contract_id: &str) {
        self.code_ids.remove(contract_id);
    }

    fn get_all_addresses(&self) -> Result<HashMap<String, Addr>, CwEnvError> {
        let mock_addresses = self.addresses.clone();
        let daemon_addresses = self.daemon_state.get_all_addresses().unwrap_or_default();

        Ok(mock_addresses
            .into_iter()
            .chain(daemon_addresses)
            .unique()
            .collect())
    }

    fn get_all_code_ids(&self) -> Result<HashMap<String, u64>, CwEnvError> {
        let mock_code_ids = self.code_ids.clone();
        let daemon_code_ids = self.daemon_state.get_all_code_ids().unwrap_or_default();

        Ok(mock_code_ids
            .into_iter()
            .chain(daemon_code_ids)
            .unique()
            .collect())
    }
}

#[cfg(test)]
mod test {
    use cosmwasm_std::Addr;
    use cw_orch_core::{environment::StateInterface, CwEnvError};
    use cw_orch_daemon::networks::JUNO_1;
    use speculoos::prelude::*;

    use super::MockState;

    const CONTRACT_ID: &str = "123";
    const CONTRACT_ADDR: &str = "cosmos123";
    #[test]
    fn mock_state() {
        let mut mock = MockState::new(JUNO_1.into(), "default-id");
        env_logger::init();

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
            .is_greater_than_or_equal_to(1)
    }
}
