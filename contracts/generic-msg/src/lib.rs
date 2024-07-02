// Here we implement some tests for the interface macros to make sure everything works and keeps working
// This is a contract not exposed, only used for compilation test (necessary)

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use serde::Serialize;

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum GenericExecuteMsg<T: Serialize> {
    Test { a: String },
    Generic(T)
}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum Foo {
    Bar(String),
    Baz(u64),
}

impl From<Foo> for GenericExecuteMsg<Foo> {
    fn from(msg: Foo) -> Self {
        Self::Generic(msg)
    }
}

type ExecuteMsg = GenericExecuteMsg<Foo>;

#[cw_orch::interface(Empty, ExecuteMsg, Empty, Empty)]
pub struct TestContract;

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
}

#[cfg(test)]
mod test {
    use super::*;
    use cw_orch::prelude::*;
    #[test]
    pub fn test() -> Result<(), CwOrchError> {
        let chain = Mock::new("sender");

        let contract = TestContract::new("test", chain.clone());

        contract.upload()?;
        contract.instantiate(&Empty {}, None, None)?;

        contract.test("abc")?;
        contract.generic(Foo::Bar(String::from("abc")))?;

        contract.bar("abc")?;

        Ok(())
    }
}
