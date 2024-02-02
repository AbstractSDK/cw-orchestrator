// use cosmwasm_schema::{cw_serde, QueryResponses};
// use cw_orch::{environment::CwEnv, interface, prelude::*};

// use cosmwasm_std::{
//     to_json_binary, Addr, Binary, Deps, DepsMut, Env, Event, MessageInfo, Response, StdError,
//     StdResult,
// };
// use cw_orch::prelude::Mock;

// #[cw_serde]
// pub struct InstantiateMsg<'a> {
//     addr: &'a str,
// }

// #[cw_serde]
// #[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))]
// pub enum ExecuteMsg<'a> {
//     FirstMessage { addr: &'a str },
// }

// #[cw_serde]
// #[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))]
// #[derive(QueryResponses)]
// pub enum QueryMsg<'a> {
//     #[returns(String)]
//     /// test-doc-comment
//     FirstQuery { addr: &'a str },
// }

// #[cw_serde]
// pub struct MigrateMsg<'a> {
//     pub addr: &'a str,
// }

// pub fn instantiate(
//     _deps: DepsMut,
//     _env: Env,
//     _info: MessageInfo,
//     _msg: InstantiateMsg,
// ) -> StdResult<Response> {
//     Ok(Response::new().add_attribute("action", "instantiate"))
// }

// pub fn execute(
//     _deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     msg: ExecuteMsg,
// ) -> StdResult<Response> {
//     match msg {
//         ExecuteMsg::FirstMessage { addr } => {
//             Ok(Response::new().add_attribute("action", "first message passed"))
//         }
//     }
// }

// #[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
// pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
//     match msg {
//         QueryMsg::FirstQuery { addr } => to_json_binary("first query passed"),
//     }
// }

// #[cfg_attr(feature = "export", cosmwasm_std::entry_point)]
// pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
//     Ok(Response::new())
// }

// #[interface(InstantiateMsg<'a>, ExecuteMsg<'a>, QueryMsg<'a>, MigrateMsg<'a>)]
// pub struct MacroContract;

// impl<'a, Chain: CwEnv> Uploadable for MacroContract<'a, Chain> {
//     fn wrapper(&self) -> <Mock as TxHandler>::ContractSource {
//         Box::new(ContractWrapper::new_with_empty(execute, instantiate, query).with_migrate(migrate))
//     }
// }

// #[test]
// fn test_instantiate() {
//     let contract = MacroContract::new(
//         "test:mock_contract",
//         Mock::new(&Addr::unchecked("Ghazshag")),
//     );
//     contract.upload().unwrap();

//     contract
//         .instantiate(
//             &InstantiateMsg {
//                 addr: "abstract_user",
//             },
//             None,
//             None,
//         )
//         .unwrap();
// }

// #[test]
// fn test_execute() {
//     let contract = MacroContract::new(
//         "test:mock_contract",
//         Mock::new(&Addr::unchecked("Ghazshag")),
//     );
//     contract.upload().unwrap();

//     contract
//         .instantiate(
//             &InstantiateMsg {
//                 addr: "abstract_account",
//             },
//             None,
//             None,
//         )
//         .unwrap();

//     let response = contract
//         .execute(
//             &ExecuteMsg::FirstMessage {
//                 addr: "abstract_account",
//             },
//             None,
//         )
//         .unwrap();

//     response.has_event(
//         &Event::new("wasm")
//             .add_attribute("_contract_addr", "contract0")
//             .add_attribute("action", "first message passed"),
//     );
// }

// #[test]
// fn test_query() {
//     let contract = MacroContract::new(
//         "test:mock_contract",
//         Mock::new(&Addr::unchecked("Ghazshag")),
//     );
//     contract.upload().unwrap();

//     contract
//         .instantiate(
//             &InstantiateMsg {
//                 addr: "abstract_account",
//             },
//             None,
//             None,
//         )
//         .unwrap();

//     let response: String = contract
//         .query(&QueryMsg::FirstQuery {
//             addr: "abstract_account",
//         })
//         .unwrap();
//     assert_eq!(response, "first query passed");
// }

// #[test]
// fn test_migrate() {
//     let admin = Addr::unchecked("Ghazshag");
//     let contract = MacroContract::new("test:mock_contract", Mock::new(&admin));
//     contract.upload().unwrap();

//     contract
//         .instantiate(
//             &InstantiateMsg {
//                 addr: "abstract_account",
//             },
//             Some(&admin),
//             None,
//         )
//         .unwrap();
//     let response = contract
//         .migrate(
//             &MigrateMsg {
//                 addr: "abstract_account",
//             },
//             contract.code_id().unwrap(),
//         )
//         .unwrap();
//     assert_eq!(response.events.len(), 1);
// }
