use counter_contract::{
    msg::InstantiateMsg, CounterContract, CounterExecuteMsgFns, CounterQueryMsgFns,
};
use cw_orch::prelude::*;

/// This example shows how to create and use the cw-multi-test mock environment
pub fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mock = MockBech32::new("osmosis-1");

    let contract_counter = CounterContract::new(mock.clone());

    contract_counter.upload()?;
    contract_counter.instantiate(&InstantiateMsg { count: 0 }, None, &[])?;
    contract_counter.increment()?;

    let query_res = contract_counter.get_count()?;
    assert_eq!(query_res.count, 1);

    let new_sender = mock.addr_make("new-sender");
    contract_counter.call_as(&new_sender).increment()?;

    let query_res = contract_counter.get_count()?;
    assert_eq!(query_res.count, 2);

    Ok(())
}
