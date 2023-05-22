use counter_contract::{contract::CONTRACT_NAME, msg::InstantiateMsg, CounterContract};
use counter_contract::{
    msg::{GetCountResponse, QueryMsg},
    CounterQueryMsgFns,
};
// Use prelude to get all the necessary imports
use cw_orch::prelude::*;

use cosmwasm_std::Addr;

// consts for testing
const USER: &str = "user";
const ADMIN: &str = "admin";
// ANCHOR: integration_test
// ANCHOR: setup
/// Instantiate the contract in any CosmWasm environment
fn setup<Chain: CwEnv>(chain: Chain) -> CounterContract<Chain> {
    // ANCHOR: constructor
    // Construct the counter interface
    let contract = CounterContract::new(CONTRACT_NAME, chain.clone());
    // ANCHOR_END: constructor
    let admin = Addr::unchecked(ADMIN);

    // Upload the contract
    let upload_resp = contract.upload().unwrap();

    // Get the code-id from the response.
    let code_id = upload_resp.uploaded_code_id().unwrap();
    // or get it from the interface.
    assert_eq!(code_id, contract.code_id().unwrap());

    // Instantiate the contract
    let msg = InstantiateMsg { count: 1i32 };
    let init_resp = contract.instantiate(&msg, Some(&admin), None).unwrap();

    // Get the address from the response
    let contract_addr = init_resp.instantiated_contract_address().unwrap();
    // or get it from the interface.
    assert_eq!(contract_addr, contract.address().unwrap());

    // Return the interface
    contract
}
// ANCHOR_END: setup

#[test]
fn count() {
    // Create a sender
    let sender = Addr::unchecked(ADMIN);
    // Create the mock
    let mock = Mock::new(&sender);

    // Set up the contract
    let contract = setup(mock.clone());

    // Increment the count of the contract
    contract
        // Set the caller to user
        .call_as(&Addr::unchecked(USER))
        // Call the increment function (auto-generated function provided by CounterExecuteMsgFns)
        .increment()
        .unwrap();
    
    // ANCHOR: query
    // Get the count.
    use counter_contract::CounterQueryMsgFns;
    let count1 = contract.get_count().unwrap();
    // ANCHOR_END: query

    // or query it manually
    let count2: GetCountResponse = contract.query(&QueryMsg::GetCount {}).unwrap();

    assert_eq!(count1, count2);

    // Check the count
    assert_eq!(count1.count, 2);
    // ANCHOR: reset
    // Reset
    use counter_contract::CounterExecuteMsgFns;
    contract.reset(0).unwrap();
    // ANCHOR_END: reset
}
// ANCHOR_END: integration_test
