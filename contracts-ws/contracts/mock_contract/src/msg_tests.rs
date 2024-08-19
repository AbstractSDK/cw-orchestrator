// Here we implement some tests for the interface macros to make sure everything works and keeps working
// This is a contract not exposed, only used for compilation test (necessary)

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};

// ANCHOR: unordered_msg_def
#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
#[cw_orch(disable_fields_sorting)]
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
#[cfg(not(target_arch = "wasm32"))]
mod interface {
    use super::*;
    use cw_orch::prelude::*;

    impl<Chain> Uploadable for TestContract<Chain> {
        fn wrapper() -> <Mock as TxHandler>::ContractSource {
            Box::new(ContractWrapper::new_with_empty(execute, instantiate, query))
        }
    }

    impl<Chain> Uploadable for OrderedTestContract<Chain> {
        fn wrapper() -> <Mock as TxHandler>::ContractSource {
            Box::new(ContractWrapper::new_with_empty(
                execute_ordered,
                instantiate,
                query,
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cw_orch::prelude::*;
    #[test]
    pub fn test() -> Result<(), CwOrchError> {
        let chain = Mock::new("sender");

        let contract = TestContract::new("test", chain.clone());
        let contract_ordered = OrderedTestContract::new("test-ordered", chain.clone());

        contract.upload()?;
        contract_ordered.upload()?;
        contract.instantiate(&Empty {}, None, &[])?;
        contract_ordered.instantiate(&Empty {}, None, &[])?;

        contract.test(5u64, "test")?;
        contract_ordered.test("test".to_string(), 5u64)?;

        Ok(())
    }
}
