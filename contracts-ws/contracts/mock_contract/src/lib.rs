mod custom_resp;
mod msg_tests;

use std::fmt::Debug;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw_storage_plus::{Item, Map};
use serde::Serialize;

// Map for testing the querier
#[cw_serde]
pub struct TestItem {
    pub first_item: u64,
    pub second_item: String,
}
const TEST_ITEM: Item<TestItem> = Item::new("test-item");
// Item for testing the querier
const TEST_MAP_KEY: &str = "MAP_TEST_KEY";
const TEST_MAP: Map<String, TestItem> = Map::new("test-map");

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg<T = String>
where
    T: Serialize,
{
    FirstMessage {},
    // ANCHOR: into_example
    #[cw_orch(payable)]
    SecondMessage {
        /// test doc-comment
        #[cw_orch(into)]
        t: T,
    },
    // ANCHOR_END: into_example
    /// test doc-comment
    ThirdMessage {
        /// test doc-comment
        t: T,
    },
    #[cw_orch(fn_name("fourth"), payable)]
    FourthMessage,
    #[cw_orch(payable, into)]
    FifthMessage,
    SixthMessage(u64, String),
    #[cw_orch(payable)]
    SeventhMessage(Uint128, String),
}

#[cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum QueryMsg<T = String>
where
    T: Serialize + Debug,
{
    #[returns(String)]
    /// test-doc-comment
    FirstQuery {},
    #[returns(String)]
    SecondQuery {
        /// test doc-comment
        t: T,
    },
    #[returns(ThirdReturn<T>)]
    ThirdQuery {
        /// test doc-comment
        t: T,
    },
    #[returns(u64)]
    FourthQuery(u64, String),
}

#[cw_serde]
pub struct ThirdReturn<T> {
    /// test doc-comment
    pub t: T,
}

#[cw_serde]
pub struct MigrateMsg {
    pub t: String,
}

#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    cw2::set_contract_version(deps.storage, "mock-contract", "0")?;

    TEST_ITEM.save(
        deps.storage,
        &TestItem {
            first_item: 1,
            second_item: "test-item".to_string(),
        },
    )?;
    TEST_MAP.save(
        deps.storage,
        TEST_MAP_KEY.to_string(),
        &TestItem {
            first_item: 2,
            second_item: "test-map".to_string(),
        },
    )?;
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
        ExecuteMsg::SecondMessage { t: _ } => Err(StdError::generic_err("Second Message Failed")),
        ExecuteMsg::ThirdMessage { .. } => {
            Ok(Response::new().add_attribute("action", "third message passed"))
        }
        ExecuteMsg::FourthMessage => {
            Ok(Response::new().add_attribute("action", "fourth message passed"))
        }
        ExecuteMsg::FifthMessage => {
            if info.funds.is_empty() {
                return Err(StdError::generic_err("Coins missing"));
            }
            Ok(Response::new().add_attribute("action", "fourth message passed"))
        }
        ExecuteMsg::SixthMessage(_, _) => {
            Ok(Response::new().add_attribute("action", "sixth message passed"))
        }
        ExecuteMsg::SeventhMessage(amount, denom) => {
            let c = info.funds[0].clone();
            if c.amount != amount && c.denom.ne(&denom) {
                return Err(StdError::generic_err("Coins don't match message"));
            }
            Ok(Response::new().add_attribute("action", "fourth message passed"))
        }
    }
}

#[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::FirstQuery {} => to_json_binary("first query passed"),
        QueryMsg::SecondQuery { .. } => Err(StdError::generic_err("Query not available")),
        QueryMsg::ThirdQuery { .. } => to_json_binary(&ThirdReturn {
            t: "third query passed",
        }),
        QueryMsg::FourthQuery(_, _) => to_json_binary(&4u64),
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
    use cw_orch::environment::ChainInfoOwned;

    use super::*;

    impl<Chain> cw_orch::prelude::Uploadable for MockContract<Chain> {
        fn wrapper(
        ) -> Box<dyn cw_orch::prelude::MockContract<cosmwasm_std::Empty, cosmwasm_std::Empty>>
        {
            Box::new(
                cw_orch::prelude::ContractWrapper::new(execute, instantiate, query)
                    .with_migrate(migrate),
            )
        }

        fn wasm(_chain: &ChainInfoOwned) -> cw_orch::prelude::WasmPath {
            use cw_orch::prelude::*;
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
    use cosmwasm_std::{coins, from_json};
    use cw_orch::prelude::*;

    #[test]
    fn compiles() -> Result<(), CwOrchError> {
        // We need to check we can still call the execute msgs conveniently
        let mock = Mock::new("sender");
        let sender = mock.sender_addr();
        mock.set_balance(&sender, coins(156 * 3, "ujuno"))?;
        let contract = LocalMockContract::new("mock-contract", mock.clone());

        contract.upload()?;
        contract.instantiate(&InstantiateMsg {}, None, &[])?;
        contract.first_message()?;
        contract
            .second_message("s", &coins(156, "ujuno"))
            .unwrap_err();
        contract.third_message("s".to_string()).unwrap();
        contract.fourth(&coins(156, "ujuno")).unwrap();
        contract.fifth_message(&coins(156, "ujuno")).unwrap();
        contract.sixth_message(45u64, "moneys").unwrap();

        contract
            .seventh_message(156u128, "ujuno", &coins(156, "ujuno"))
            .unwrap();

        contract.first_query().unwrap();
        contract.second_query("arg".to_string()).unwrap_err();
        contract.third_query("arg".to_string()).unwrap();
        contract.fourth_query(45u64, "moneys").unwrap();

        Ok(())
    }

    #[test]
    fn raw_query() -> Result<(), CwOrchError> {
        // We need to check we can still call the execute msgs conveniently
        let sender = Addr::unchecked("sender");
        let mock = Mock::new(&sender);
        mock.set_balance(&sender, coins(156 * 2, "ujuno"))?;
        let contract = LocalMockContract::new("mock-contract", mock.clone());

        contract.upload()?;
        contract.instantiate(&InstantiateMsg {}, None, &[])?;

        let cw2_info: cw2::ContractVersion = from_json(
            mock.wasm_querier()
                .raw_query(&contract.address()?, b"contract_info".to_vec())?,
        )?;

        assert_eq!(
            cw2_info,
            cw2::ContractVersion {
                contract: "mock-contract".to_owned(),
                version: "0".to_owned()
            }
        );

        assert_eq!(
            contract.item_query(TEST_ITEM)?,
            TestItem {
                first_item: 1,
                second_item: "test-item".to_string()
            }
        );
        assert_eq!(
            contract.map_query(TEST_MAP, TEST_MAP_KEY.to_string())?,
            TestItem {
                first_item: 2,
                second_item: "test-map".to_string()
            }
        );

        Ok(())
    }
}
