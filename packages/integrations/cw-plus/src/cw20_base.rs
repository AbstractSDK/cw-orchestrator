use cw_orch::interface;

use cw20_base::contract;
pub use cw20_base::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
#[cfg(not(target_arch = "wasm32"))]
pub use interfaces::{AsyncQueryMsgInterfaceFns, ExecuteMsgInterfaceFns, QueryMsgInterfaceFns};

// TODO: cw20 Migrate doesn't implement Debug: https://github.com/CosmWasm/cw-plus/pull/910
#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, cosmwasm_std::Empty)]
pub struct Cw20Base;

#[cfg(not(target_arch = "wasm32"))]
use cw_orch::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: CwEnv> Uploadable for Cw20Base<Chain> {
    // Return the path to the wasm file
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("cw20_base")
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
            .with_migrate(contract::migrate),
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
/// Copy messages of the contract to implement cw-orch helpers on Execute([`cw_orch::ExecuteFns`]) and Query([`cw_orch::QueryFns`]) interfaces
mod interfaces {
    use super::*;

    #[derive(cw_orch::ExecuteFns, cw_orch_from_interface_derive::FromInterface)]

    pub enum ExecuteMsgInterface {
        /// Transfer is a base message to move tokens to another account without triggering actions
        Transfer {
            recipient: String,
            amount: cosmwasm_std::Uint128,
        },
        /// Burn is a base message to destroy tokens forever
        Burn { amount: cosmwasm_std::Uint128 },
        /// Send is a base message to transfer tokens to a contract and trigger an action
        /// on the receiving contract.
        Send {
            contract: String,
            amount: cosmwasm_std::Uint128,
            msg: cosmwasm_std::Binary,
        },
        /// Only with "approval" extension. Allows spender to access an additional amount tokens
        /// from the owner's (env.sender) account. If expires is Some(), overwrites current allowance
        /// expiration with this one.
        IncreaseAllowance {
            spender: String,
            amount: cosmwasm_std::Uint128,
            expires: Option<cw20::Expiration>,
        },
        /// Only with "approval" extension. Lowers the spender's access of tokens
        /// from the owner's (env.sender) account by amount. If expires is Some(), overwrites current
        /// allowance expiration with this one.
        DecreaseAllowance {
            spender: String,
            amount: cosmwasm_std::Uint128,
            expires: Option<cw20::Expiration>,
        },
        /// Only with "approval" extension. Transfers amount tokens from owner -> recipient
        /// if `env.sender` has sufficient pre-approval.
        TransferFrom {
            owner: String,
            recipient: String,
            amount: cosmwasm_std::Uint128,
        },
        /// Only with "approval" extension. Sends amount tokens from owner -> contract
        /// if `env.sender` has sufficient pre-approval.
        SendFrom {
            owner: String,
            contract: String,
            amount: cosmwasm_std::Uint128,
            msg: cosmwasm_std::Binary,
        },
        /// Only with "approval" extension. Destroys tokens forever
        BurnFrom {
            owner: String,
            amount: cosmwasm_std::Uint128,
        },
        /// Only with the "mintable" extension. If authorized, creates amount new tokens
        /// and adds to the recipient balance.
        Mint {
            recipient: String,
            amount: cosmwasm_std::Uint128,
        },
        /// Only with the "mintable" extension. The current minter may set
        /// a new minter. Setting the minter to None will remove the
        /// token's minter forever.
        UpdateMinter { new_minter: Option<String> },
        /// Only with the "marketing" extension. If authorized, updates marketing metadata.
        /// Setting None/null for any of these will leave it unchanged.
        /// Setting Some("") will clear this field on the contract storage
        UpdateMarketing {
            /// A URL pointing to the project behind this token.
            project: Option<String>,
            /// A longer description of the token and it's utility. Designed for tooltips or such
            description: Option<String>,
            /// The address (if any) who can update this data structure
            marketing: Option<String>,
        },
        /// If set as the "marketing" role on the contract, upload a new URL, SVG, or PNG for the token
        UploadLogo(cw20::Logo),
    }

    #[cosmwasm_schema::cw_serde]
    #[derive(
        cosmwasm_schema::QueryResponses,
        cw_orch::QueryFns,
        cw_orch_from_interface_derive::FromInterface,
    )]
    pub enum QueryMsgInterface {
        /// Returns the current balance of the given address, 0 if unset.
        #[returns(cw20::BalanceResponse)]
        Balance { address: String },
        /// Returns metadata on the contract - name, decimals, supply, etc.
        #[returns(cw20::TokenInfoResponse)]
        TokenInfo {},
        /// Only with "mintable" extension.
        /// Returns who can mint and the hard cap on maximum tokens after minting.
        #[returns(cw20::MinterResponse)]
        Minter {},
        /// Only with "allowance" extension.
        /// Returns how much spender can use from owner account, 0 if unset.
        #[returns(cw20::AllowanceResponse)]
        Allowance { owner: String, spender: String },
        /// Only with "enumerable" extension (and "allowances")
        /// Returns all allowances this owner has approved. Supports pagination.
        #[returns(cw20::AllAllowancesResponse)]
        AllAllowances {
            owner: String,
            start_after: Option<String>,
            limit: Option<u32>,
        },
        /// Only with "enumerable" extension (and "allowances")
        /// Returns all allowances this spender has been granted. Supports pagination.
        #[returns(cw20::AllSpenderAllowancesResponse)]
        AllSpenderAllowances {
            spender: String,
            start_after: Option<String>,
            limit: Option<u32>,
        },
        /// Only with "enumerable" extension
        /// Returns all accounts that have balances. Supports pagination.
        #[returns(cw20::AllAccountsResponse)]
        AllAccounts {
            start_after: Option<String>,
            limit: Option<u32>,
        },
        /// Only with "marketing" extension
        /// Returns more metadata on the contract to display in the client:
        /// - description, logo, project url, etc.
        #[returns(cw20::MarketingInfoResponse)]
        MarketingInfo {},
        /// Only with "marketing" extension
        /// Downloads the embedded logo data (if stored on chain). Errors if no logo data is stored for this
        /// contract.
        #[returns(cw20::DownloadLogoResponse)]
        DownloadLogo {},
    }
}
