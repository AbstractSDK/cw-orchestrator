use cw_orch::interface;

use cw1_whitelist::contract;
pub use cw1_whitelist::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
#[cfg(not(target_arch = "wasm32"))]
pub use interfaces::{AsyncQueryMsgInterfaceFns, ExecuteMsgInterfaceFns, QueryMsgInterfaceFns};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw1Whitelist;

#[cfg(not(target_arch = "wasm32"))]
use cw_orch::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: CwEnv> Uploadable for Cw1Whitelist<Chain> {
    // Return the path to the wasm file
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("cw1_whitelist")
            .unwrap()
    }
    // Return a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(ContractWrapper::new_with_empty(
            contract::execute,
            contract::instantiate,
            contract::query,
        ))
    }
}

#[cfg(not(target_arch = "wasm32"))]
/// Copy messages of the contract to implement cw-orch helpers on Execute([`cw_orch::ExecuteFns`]) and Query([`cw_orch::QueryFns`]) interfaces
mod interfaces {
    use super::*;

    use cosmwasm_schema::schemars::JsonSchema;

    #[derive(cw_orch::ExecuteFns, cw_orch_from_interface_derive::FromInterface)]
    enum ExecuteMsgInterface<T = Empty>
    where
        T: Clone + std::fmt::Debug + PartialEq + JsonSchema,
    {
        /// Execute requests the contract to re-dispatch all these messages with the
        /// contract's address as sender. Every implementation has it's own logic to
        /// determine in
        // This method is renamed to not conflict with [`CwOrchExecute::execute`]
        #[cw_orch(fn_name("execute_requests"))]
        Execute {
            msgs: Vec<cosmwasm_std::CosmosMsg<T>>,
        },
        /// Freeze will make a mutable contract immutable, must be called by an admin
        Freeze {},
        /// UpdateAdmins will change the admin set of the contract, must be called by an existing admin,
        /// and only works if the contract is mutable
        UpdateAdmins { admins: Vec<String> },
    }

    #[cosmwasm_schema::cw_serde]
    #[derive(
        cosmwasm_schema::QueryResponses,
        cw_orch::QueryFns,
        cw_orch_from_interface_derive::FromInterface,
    )]
    enum QueryMsgInterface<T = Empty>
    where
        T: Clone + std::fmt::Debug + PartialEq + JsonSchema,
    {
        /// Shows all admins and whether or not it is mutable
        #[returns(cw1_whitelist::msg::AdminListResponse)]
        AdminList {},
        /// Checks permissions of the caller on this proxy.
        /// If CanExecute returns true then a call to `Execute` with the same message,
        /// before any further state changes, should also succeed.
        #[returns(cw1::CanExecuteResponse)]
        CanExecute {
            sender: String,
            msg: cosmwasm_std::CosmosMsg<T>,
        },
    }
}
