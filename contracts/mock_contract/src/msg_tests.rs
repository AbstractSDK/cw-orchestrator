// Here we implement some tests for the interface macros to make sure everything works and keeps working
// This is a contract not exposed, only used for compilation test (necessary)

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use cw_orch::prelude::*;

// ANCHOR: unordered_msg_def
#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[disable_fields_sorting]
pub enum ExecuteMsg {
    Test { b: u64, a: String },
}
// ANCHOR_END: unordered_msg_def

// ANCHOR: ordered_msg_def
#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsgOrdered {
    Test { b: u64, a: String },
}
// ANCHOR_END: ordered_msg_def

#[cw_orch::interface(Empty, ExecuteMsg, Empty, Empty)]
pub struct TestContract;

#[cw_orch::interface(Empty, ExecuteMsgOrdered, Empty, Empty)]
pub struct OrderedTestContract;

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn execute_ordered(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsgOrdered,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
    Ok(vec![].into())
}

impl<Chain: CwEnv> Uploadable for TestContract<Chain> {
    fn wrapper(&self) -> <Mock as TxHandler>::ContractSource {
        Box::new(ContractWrapper::new_with_empty(execute, instantiate, query))
    }
}

impl<Chain: CwEnv> Uploadable for OrderedTestContract<Chain> {
    fn wrapper(&self) -> <Mock as TxHandler>::ContractSource {
        Box::new(ContractWrapper::new_with_empty(
            execute_ordered,
            instantiate,
            query,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test() -> Result<(), CwOrchError> {
        let chain = Mock::new(&Addr::unchecked("sender"));

        let contract = TestContract::new("test", chain.clone());
        let contract_ordered = OrderedTestContract::new("test-ordered", chain.clone());

        contract.upload()?;
        contract_ordered.upload()?;
        contract.instantiate(&Empty {}, None, None)?;
        contract_ordered.instantiate(&Empty {}, None, None)?;

        contract.test(5, "test".to_string())?;
        contract_ordered.test("test".to_string(), 5)?;

        Ok(())
    }
}
