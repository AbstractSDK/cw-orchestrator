#[cfg(test)]
mod tests {
    // Use prelude to get all the necessary imports
    use crate::{msg::InstantiateMsg, ContractCounter};
    use cw_orch::prelude::*;

    use cosmwasm_std::{Addr, Coin, Empty, Uint128};

    // consts for testing
    const USER: &str = "user";
    const ADMIN: &str = "admin";
    const NATIVE_DENOM: &str = "denom";

    /// Instantiate the contract in any CosmWasm environment
    fn proper_instantiate<Chain: CwEnv>(chain: Chain) -> ContractCounter<Chain> {
        // Construct the counter interface
        let contract = ContractCounter::new(CONTRACT_NAME, chain.clone());

        // Upload the contract
        contract.upload().unwrap();

        // Instantiate the contract
        let msg = InstantiateMsg { count: 1i32 };
        let contract_addr = contract.instantiate(&msg, ADMIN, None).unwrap();

        // Return the contract
        contract
    }

    mod count {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn count() {
            // Create a sender
            let sender = Addr::unchecked(ADMIN);
            // Create the mock
            let mock = Mock::new(&sender);

            // Set up the contract
            let contract = proper_instantiate(mock.clone());

            // increment the count
            contract.call_as(Addr::unchecked(USER)).increment().unwrap();
        }
    }
}
