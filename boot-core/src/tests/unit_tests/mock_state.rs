/*
    MockState tests
    I know the MockState is mostly a HashMap
    but it returns two errors that we need to test ;)
*/
#[cfg(test)]
mod general {
    use speculoos::prelude::*;
    use cosmwasm_std::Addr;
    use crate::{MockState, StateInterface, BootError};

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
        asserting(&"address is correct for contract_id")
            .that(unchecked_address)
            .is_equal_to(&addr);

        // assert we get the right code_id
        let fetched_id = mock.get_code_id(CONTRACT_ID).unwrap();
        asserting(&"code_id is correct for contract_id")
            .that(&fetched_id)
            .is_equal_to(&code_id);

        // assert we get AddrNotInStore error
        let missing_id = &"456";
        let error = mock.get_address(missing_id).unwrap_err();
        let error_msg = BootError::AddrNotInStore(String::from(*missing_id)).to_string();
        asserting(&format!("Asserting we get BootError: {}", error_msg))
            .that(&error.to_string())
            .is_equal_to(BootError::AddrNotInStore(String::from(*missing_id)).to_string());

        // assert we get CodeIdNotInStore error
        let error_msg = BootError::CodeIdNotInStore(String::from(*missing_id)).to_string();
        let error = mock.get_code_id(missing_id).unwrap_err();
        asserting(&format!("Asserting we get BootError: {}", error_msg))
            .that(&error.to_string())
            .is_equal_to(BootError::CodeIdNotInStore(String::from(*missing_id)).to_string());

        let total = mock.get_all_addresses().unwrap().len();
        asserting(&"total addresses is one")
            .that(&total)
            .is_equal_to(&1);

        let total = mock.get_all_code_ids().unwrap().len();
        asserting(&"total code_ids is one")
            .that(&total)
            .is_equal_to(&1)
    }
}