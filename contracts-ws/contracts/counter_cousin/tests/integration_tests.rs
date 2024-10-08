// ANCHOR: all
use counter_cousin_contract::{
    msg::{GetCountResponse, InstantiateMsg, QueryMsg},
    ContractError, CounterContract, CounterExecuteMsgFns, CounterQueryMsgFns,
};
// Use prelude to get all the necessary imports
use cw_orch::prelude::*;

use cosmwasm_std::Addr;

// consts for testing
const USER: &str = "user";
const ADMIN: &str = "admin";
// ANCHOR: integration_test

// ANCHOR: count_test
#[test]
fn count() -> anyhow::Result<()> {
    // Create the mock. This will be our chain object throughout
    let mock = Mock::new(ADMIN);
    let user = Addr::unchecked(USER);

    // Set up the contract (Definition below) ↓↓
    let (contract, cousin) = setup(mock.clone())?;

    // Increment the count of the contract
    contract.call_as(&user).increment()?;
    contract.call_as(&user).increment()?;

    cousin.call_as(&user).increment()?;
    cousin.call_as(&user).increment()?;
    cousin.call_as(&user).increment()?;

    let count = contract.get_count()?;
    assert_eq!(count.count, 3);

    let count = contract.get_cousin_count()?;
    assert_eq!(count.count, 4);

    let count = cousin.get_count()?;
    assert_eq!(count.count, 4);

    let count = cousin.get_cousin_count()?;
    assert_eq!(count.count, 3);

    Ok(())
}
// ANCHOR_END: count_test

// ANCHOR: setup
/// Instantiate the contract in any CosmWasm environment
fn setup<Chain: CwEnv>(
    chain: Chain,
) -> anyhow::Result<(CounterContract<Chain>, CounterContract<Chain>)> {
    // ANCHOR: constructor
    // Construct the counter interface
    let contract = CounterContract::new("cousin", chain.clone());
    let cousin = CounterContract::new("counter", chain.clone());
    // ANCHOR_END: constructor

    // Upload the contract
    contract.upload()?;
    cousin.upload()?;

    // Instantiate the contract
    let msg = InstantiateMsg { count: 1i32 };
    contract.instantiate(&msg, None, &[])?;
    cousin.instantiate(&msg, None, &[])?;

    contract.set_cousin(cousin.address()?)?;
    cousin.set_cousin(contract.address()?)?;

    // Return the interface
    Ok((contract, cousin))
}
// ANCHOR_END: setup

// ANCHOR_END: integration_test
// ANCHOR_END: all
