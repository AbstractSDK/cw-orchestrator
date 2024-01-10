use anyhow::Result as AnyResult;
use cosmwasm_std::{CustomMsg, CustomQuery};
use cw_multi_test::Contract;
use cw_orch_core::contract::MockContract;
use serde::de::DeserializeOwned;

pub struct MockContractWrapper<T, Q>(pub Box<dyn MockContract<T, Q>>)
where
    T: CustomMsg,
    Q: CustomQuery + DeserializeOwned + 'static;

impl<C, Q> Contract<C, Q> for MockContractWrapper<C, Q>
where
    C: CustomMsg,
    Q: CustomQuery + DeserializeOwned,
{
    fn execute(
        &self,
        deps: cosmwasm_std::DepsMut<Q>,
        env: cosmwasm_std::Env,
        info: cosmwasm_std::MessageInfo,
        msg: Vec<u8>,
    ) -> AnyResult<cosmwasm_std::Response<C>> {
        self.0.execute(deps, env, info, msg)
    }

    fn instantiate(
        &self,
        deps: cosmwasm_std::DepsMut<Q>,
        env: cosmwasm_std::Env,
        info: cosmwasm_std::MessageInfo,
        msg: Vec<u8>,
    ) -> AnyResult<cosmwasm_std::Response<C>> {
        self.0.instantiate(deps, env, info, msg)
    }

    fn query(
        &self,
        deps: cosmwasm_std::Deps<Q>,
        env: cosmwasm_std::Env,
        msg: Vec<u8>,
    ) -> AnyResult<cosmwasm_std::Binary> {
        self.0.query(deps, env, msg)
    }

    fn sudo(
        &self,
        deps: cosmwasm_std::DepsMut<Q>,
        env: cosmwasm_std::Env,
        msg: Vec<u8>,
    ) -> AnyResult<cosmwasm_std::Response<C>> {
        self.0.sudo(deps, env, msg)
    }

    fn reply(
        &self,
        deps: cosmwasm_std::DepsMut<Q>,
        env: cosmwasm_std::Env,
        msg: cosmwasm_std::Reply,
    ) -> AnyResult<cosmwasm_std::Response<C>> {
        self.0.reply(deps, env, msg)
    }

    fn migrate(
        &self,
        deps: cosmwasm_std::DepsMut<Q>,
        env: cosmwasm_std::Env,
        msg: Vec<u8>,
    ) -> AnyResult<cosmwasm_std::Response<C>> {
        self.0.migrate(deps, env, msg)
    }
}
