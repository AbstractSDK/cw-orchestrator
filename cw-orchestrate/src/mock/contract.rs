// use std::fmt;

// use cosmwasm_std::{CustomQuery, CustomMsg};
// use cw_multi_test::Contract as MockContract;
// use schemars::JsonSchema;
// use serde::de::DeserializeOwned;

// use crate::{Contract, Mock};

// impl<T, Q> MockContract<T, Q> for Contract<Mock, T, Q>
// where
//     T: CustomMsg + DeserializeOwned,
//     Q: CustomQuery,{
//     fn execute(
//         &self,
//         deps: cosmwasm_std::DepsMut<Q>,
//         env: cosmwasm_std::Env,
//         info: cosmwasm_std::MessageInfo,
//         msg: Vec<u8>,
//     ) -> anyhow::Result<cosmwasm_std::Response<T>> {
//         self.
//     }

//     fn instantiate(
//         &self,
//         deps: cosmwasm_std::DepsMut<Q>,
//         env: cosmwasm_std::Env,
//         info: cosmwasm_std::MessageInfo,
//         msg: Vec<u8>,
//     ) -> anyhow::Result<cosmwasm_std::Response<T>> {
//         todo!()
//     }

//     fn query(&self, deps: cosmwasm_std::Deps<Q>, env: cosmwasm_std::Env, msg: Vec<u8>) -> anyhow::Result<cosmwasm_std::Binary> {
//         todo!()
//     }

//     fn sudo(&self, deps: cosmwasm_std::DepsMut<Q>, env: cosmwasm_std::Env, msg: Vec<u8>) -> anyhow::Result<cosmwasm_std::Response<T>> {
//         todo!()
//     }

//     fn reply(&self, deps: cosmwasm_std::DepsMut<Q>, env: cosmwasm_std::Env, msg: cosmwasm_std::Reply) -> anyhow::Result<cosmwasm_std::Response<T>> {
//         todo!()
//     }

//     fn migrate(&self, deps: cosmwasm_std::DepsMut<Q>, env: cosmwasm_std::Env, msg: Vec<u8>) -> anyhow::Result<cosmwasm_std::Response<T>> {
//         todo!()
//     }
// }
