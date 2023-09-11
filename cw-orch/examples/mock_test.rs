use std::str::FromStr;

use cosmwasm_std::{coins, Addr, BlockInfo, Decimal, Timestamp};

use counter_contract::{
    contract::CounterContract,
    msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg},
};
use cw_orch::prelude::{CallAs, CwOrchExecute, CwOrchInstantiate, CwOrchQuery, CwOrchUpload, Mock};

/// This example shows how to create and use the cw-multi-test mock environment
pub fn main() {
    // ANCHOR: mock_creation
    let sender = Addr::unchecked("juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y");

    let mock = Mock::new(&sender);
    // ANCHOR_END: mock_creation

    // ANCHOR: mock_usage
    let contract_counter = CounterContract::new("mock:contract_counter", mock);

    let upload_res = contract_counter.upload();
    upload_res.unwrap();
    // ANCHOR_END: mock_usage

    let init_res = contract_counter.instantiate(&InstantiateMsg { count: 0 }, Some(&sender), None);
    init_res.unwrap();

    let exec_res = contract_counter.execute(&ExecuteMsg::Increment {}, None);
    exec_res.unwrap();

    let query_res = contract_counter.query::<GetCountResponse>(&QueryMsg::GetCount {});
    assert_eq!(query_res.unwrap().count, 1);
}

// This is used for documentation only
// This is actually only used to avoid having the `mut` keyword inside the mock_usage anchor (only necessary for set_sender)
pub fn customize() {
    let sender = Addr::unchecked("juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y");

    let mock = Mock::new(&sender);

    let mut contract_counter = CounterContract::new("mock:contract_counter", mock.clone());

    // ANCHOR: mock_customization
    let new_sender = Addr::unchecked("entirely-new-sender");
    mock.set_balance(&new_sender, coins(100_000, "ujunox"))
        .unwrap();

    // Reuploads as the new sender
    contract_counter.call_as(&new_sender).upload().unwrap();

    // Here the contract_counter sender is again `sender`

    // Sets the new_sender as the definite sender
    contract_counter.set_sender(&new_sender);

    // From now on the contract_counter sender is `new_sender`
    // ANCHOR_END: mock_customization

    // ANCHOR: deep_mock_customization
    mock.app
        .borrow_mut()
        .init_modules(|router, api, storage| {
            router.staking.add_validator(
                api,
                storage,
                &BlockInfo {
                    height: 16736,
                    time: Timestamp::from_seconds(13345762376),
                    chain_id: "juno-1".to_string(),
                },
                cosmwasm_std::Validator {
                    address: "new-validator-address".to_string(),
                    commission: Decimal::from_str("0.5").unwrap(), // Greedy validator
                    max_commission: Decimal::from_str("1").unwrap(), // Dangerous validator
                    max_change_rate: Decimal::from_str("1").unwrap(), // Very dangerous validator
                },
            )
        })
        .unwrap();
    // ANCHOR_END: deep_mock_customization
}
