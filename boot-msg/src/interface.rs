use std::fmt::Debug;
use cosmwasm_std::{Addr, Coin, CosmosMsg, StdResult, wasm_execute};
use serde::{Serialize};

/// Smart Contract execute endpoint
pub trait CwContract {
    fn address(&self) -> Addr;
}

pub trait CwContractExecute: CwContract {
    type ExecuteMsg: Serialize;

    fn execute_msg(
        &self,
        execute_msg: &Self::ExecuteMsg,
        funds: Vec<Coin>,
    ) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::from(wasm_execute(self.address(), execute_msg, funds)?))
    }
}

impl<T: CwInterface + CwContract> CwContractExecute for T {
    type ExecuteMsg = <T as CwInterface>::ExecuteMsg;
}

/// Trait that holds the entry-point types for a contract.
pub trait CwInterface {
    type InstantiateMsg: Serialize + Debug;
    type ExecuteMsg: Serialize + Debug;
    type QueryMsg: Serialize + Debug;
    type MigrateMsg: Serialize + Debug;
}
