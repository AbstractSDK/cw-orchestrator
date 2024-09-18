use cw20_ics20::{
    contract,
    ibc::{
        ibc_channel_close, ibc_channel_connect, ibc_channel_open, ibc_packet_ack,
        ibc_packet_receive, ibc_packet_timeout,
    },
};
use cw_orch::interface;

pub use cw20_ics20::msg::{AllowMsg, ExecuteMsg, InitMsg, MigrateMsg, QueryMsg, TransferMsg};
#[cfg(not(target_arch = "wasm32"))]
pub use interfaces::{AsyncQueryMsgInterfaceFns, ExecuteMsgInterfaceFns, QueryMsgInterfaceFns};

#[interface(InitMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Cw20Ics20;

#[cfg(not(target_arch = "wasm32"))]
use cw_orch::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: CwEnv> Uploadable for Cw20Ics20<Chain> {
    // Return the path to the wasm file
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("cw20_ics20")
            .unwrap()
    }
    // Return a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                contract::execute,
                contract::instantiate,
                contract::query,
            )
            .with_migrate(contract::migrate)
            .with_ibc(
                ibc_channel_open,
                ibc_channel_connect,
                ibc_channel_close,
                ibc_packet_receive,
                ibc_packet_ack,
                ibc_packet_timeout,
            ),
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
/// Copy messages of the contract to implement cw-orch helpers on Execute([`cw_orch::ExecuteFns`]) and Query([`cw_orch::QueryFns`]) interfaces
mod interfaces {
    use super::*;

    #[derive(cw_orch::ExecuteFns, cw_orch_from_interface_derive::FromInterface)]
    pub enum ExecuteMsgInterface {
        /// This accepts a properly-encoded ReceiveMsg from a cw20 contract
        Receive(cw20::Cw20ReceiveMsg),
        /// This allows us to transfer *exactly one* native token
        #[cw_orch(payable)]
        Transfer(cw20_ics20::msg::TransferMsg),
        /// This must be called by gov_contract, will allow a new cw20 token to be sent
        Allow(cw20_ics20::msg::AllowMsg),
        /// Change the admin (must be called by current admin)
        UpdateAdmin { admin: String },
    }

    #[cosmwasm_schema::cw_serde]
    #[derive(
        cosmwasm_schema::QueryResponses,
        cw_orch::QueryFns,
        cw_orch_from_interface_derive::FromInterface,
    )]
    pub enum QueryMsgInterface {
        /// Return the port ID bound by this contract.
        #[returns(cw20_ics20::msg::PortResponse)]
        Port {},
        /// Show all channels we have connected to.
        #[returns(cw20_ics20::msg::ListChannelsResponse)]
        ListChannels {},
        /// Returns the details of the name channel, error if not created.
        #[returns(cw20_ics20::msg::ChannelResponse)]
        Channel { id: String },
        /// Show the Config.
        #[returns(cw20_ics20::msg::ConfigResponse)]
        Config {},
        #[returns(cw_controllers::AdminResponse)]
        Admin {},
        /// Query if a given cw20 contract is allowed.
        #[returns(cw20_ics20::msg::AllowedResponse)]
        Allowed { contract: String },
        /// List all allowed cw20 contracts.
        #[returns(cw20_ics20::msg::ListAllowedResponse)]
        ListAllowed {
            start_after: Option<String>,
            limit: Option<u32>,
        },
    }
}
