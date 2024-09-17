use cw_orch::interface;

use cw4_group::contract;
pub use cw4_group::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
#[cfg(not(target_arch = "wasm32"))]
pub use interfaces::{AsyncQueryMsgInterfaceFns, ExecuteMsgInterfaceFns, QueryMsgInterfaceFns};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw4Group;

#[cfg(not(target_arch = "wasm32"))]
use cw_orch::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: CwEnv> Uploadable for Cw4Group<Chain> {
    // Return the path to the wasm file
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("cw4_group")
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

    #[derive(cw_orch::ExecuteFns, cw_orch_from_interface_derive::FromInterface)]
    pub enum ExecuteMsgInterface {
        /// Change the admin
        UpdateAdmin { admin: Option<String> },
        /// apply a diff to the existing members.
        /// remove is applied after add, so if an address is in both, it is removed
        UpdateMembers {
            remove: Vec<String>,
            add: Vec<cw4::Member>,
        },
        /// Add a new hook to be informed of all membership changes. Must be called by Admin
        AddHook { addr: String },
        /// Remove a hook. Must be called by Admin
        RemoveHook { addr: String },
    }

    #[cosmwasm_schema::cw_serde]
    #[derive(
        cosmwasm_schema::QueryResponses,
        cw_orch::QueryFns,
        cw_orch_from_interface_derive::FromInterface,
    )]
    pub enum QueryMsgInterface {
        #[returns(cw_controllers::AdminResponse)]
        Admin {},
        #[returns(cw4::TotalWeightResponse)]
        TotalWeight { at_height: Option<u64> },
        #[returns(cw4::MemberListResponse)]
        ListMembers {
            start_after: Option<String>,
            limit: Option<u32>,
        },
        #[returns(cw4::MemberResponse)]
        Member {
            addr: String,
            at_height: Option<u64>,
        },
        /// Shows all registered hooks.
        #[returns(cw_controllers::HooksResponse)]
        Hooks {},
    }
}
