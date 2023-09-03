// Here we implement some tests for the interface macros to make sure everything works and keeps working
// This is a contract not exposed, only used for compilation test (necessary)

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Empty};
use cw_orch::prelude::*;

#[cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))]
#[cfg_attr(feature = "interface", disable_fields_sorting)]
pub enum ExecuteMsg {
    Test { b: u64, a: String },
}

#[cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))]
pub enum ExecuteMsgOrdered {
    Test { b: u64, a: String },
}

#[cw_orch::interface(Empty, ExecuteMsg, Empty, Empty)]
pub struct MockContract;

#[cw_orch::interface(Empty, ExecuteMsgOrdered, Empty, Empty)]
pub struct OrderedMockContract;

#[allow(unused)]
pub fn test() -> Result<(), CwOrchError> {
    let chain = Mock::new(&Addr::unchecked("sender"));

    let contract = MockContract::new("test", chain.clone());
    let contract_ordered = OrderedMockContract::new("test-ordered", chain.clone());

    contract.test(5, "test".to_string())?;
    contract_ordered.test("test".to_string(), 5)?;

    Ok(())
}
