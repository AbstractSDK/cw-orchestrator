mod msg_tests;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(old_cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    FirstMessage {},
    #[payable]
    SecondMessage {
        /// test doc-comment
        t: String,
    },
}

#[cw_serde]
#[derive(old_cw_orch::QueryFns, QueryResponses)]
pub enum QueryMsg {
    #[returns(String)]
    /// test-doc-comment
    FirstQuery {},
    #[returns(u64)]
    SecondQuery {
        /// test doc-comment
        t: String,
    },
}

#[cw_serde]
pub struct MigrateMsg {
    pub t: String,
}

#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::FirstMessage {} => {
            Ok(Response::new().add_attribute("action", "first message passed"))
        }
        ExecuteMsg::SecondMessage { t: _ } => {
            Ok(Response::new().add_attribute("action", "first message passed"))
        }
    }
}

#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::FirstQuery {} => to_json_binary("first query passed"),
        QueryMsg::SecondQuery { .. } => to_json_binary(&89u64),
    }
}

#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> StdResult<Response> {
    if msg.t.eq("success") {
        Ok(Response::new())
    } else {
        Err(StdError::generic_err(
            "migrate endpoint reached but no test implementation",
        ))
    }
}

#[cw_orch::interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct MockContract;

#[cfg(not(target_arch = "wasm32"))]
pub mod interface {
    use old_cw_orch::environment::ChainInfoOwned;

    use crate::{execute, instantiate, migrate, query, MockContract};

    impl<Chain> old_cw_orch::prelude::Uploadable for MockContract<Chain> {
        fn wrapper(
        ) -> Box<dyn old_cw_orch::prelude::MockContract<cosmwasm_std::Empty, cosmwasm_std::Empty>>
        {
            Box::new(
                old_cw_orch::prelude::ContractWrapper::new(execute, instantiate, query)
                    .with_migrate(migrate),
            )
        }

        fn wasm(_chain: &ChainInfoOwned) -> old_cw_orch::prelude::WasmPath {
            use old_cw_orch::prelude::*;
            artifacts_dir_from_workspace!()
                .find_wasm_path("mock_contract")
                .unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use super::MockContract as LocalMockContract;
    use super::*;
    use cosmwasm_std::coins;
    use cw_orch::prelude::*;

    #[test]
    fn compiles() -> Result<(), CwOrchError> {
        // We need to check we can still call the execute msgs conveniently
        let sender = Addr::unchecked("sender");
        let mock = Mock::new(&sender);
        mock.set_balance(&sender, coins(156 * 2, "ujuno"))?;
        let contract = LocalMockContract::new("mock-contract", mock.clone());

        contract.upload()?;
        contract.instantiate(&InstantiateMsg {}, None, None)?;
        contract.first_message()?;
        contract.second_message("s".to_string(), &coins(156, "ujuno"))?;

        contract.first_query()?;
        contract.second_query("arg".to_string())?;

        Ok(())
    }
}
