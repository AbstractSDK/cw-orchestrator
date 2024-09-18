use cw_orch::interface;

use cw4_stake::contract;
pub use cw4_stake::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
#[cfg(not(target_arch = "wasm32"))]
pub use interfaces::{AsyncQueryMsgInterfaceFns, ExecuteMsgInterfaceFns, QueryMsgInterfaceFns};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw4Stake;

#[cfg(not(target_arch = "wasm32"))]
use cw_orch::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: CwEnv> Uploadable for Cw4Stake<Chain> {
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

    #[derive(cw_orch::ExecuteFns, cw_orch_from_interface_derive::FromInterface)]
    pub enum ExecuteMsgInterface {
        /// Bond will bond all staking tokens sent with the message and update membership weight
        #[cw_orch(payable)]
        Bond {},
        /// Unbond will start the unbonding process for the given number of tokens.
        /// The sender immediately loses weight from these tokens, and can claim them
        /// back to his wallet after `unbonding_period`
        Unbond { tokens: cosmwasm_std::Uint128 },
        /// Claim is used to claim your native tokens that you previously "unbonded"
        /// after the contract-defined waiting period (eg. 1 week)
        Claim {},

        /// Change the admin
        UpdateAdmin { admin: Option<String> },
        /// Add a new hook to be informed of all membership changes. Must be called by Admin
        AddHook { addr: String },
        /// Remove a hook. Must be called by Admin
        RemoveHook { addr: String },

        /// This accepts a properly-encoded ReceiveMsg from a cw20 contract
        Receive(cw20::Cw20ReceiveMsg),
    }

    #[cosmwasm_schema::cw_serde]
    #[derive(
        cosmwasm_schema::QueryResponses,
        cw_orch::QueryFns,
        cw_orch_from_interface_derive::FromInterface,
    )]
    pub enum QueryMsgInterface {
        /// Claims shows the tokens in process of unbonding for this address
        #[returns(cw_controllers::ClaimsResponse)]
        Claims { address: String },
        // Show the number of tokens currently staked by this address.
        #[returns(cw4_stake::msg::StakedResponse)]
        Staked { address: String },

        #[returns(cw_controllers::AdminResponse)]
        Admin {},
        #[returns(cw4::TotalWeightResponse)]
        TotalWeight {},
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
