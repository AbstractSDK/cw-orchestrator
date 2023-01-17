use boot_core::prelude::*;
use cosmwasm_std::{Addr, Binary, Empty, Uint128};
use cw_multi_test::ContractWrapper;

use cw20::{BalanceResponse, Cw20Coin, MinterResponse, Expiration};
use cw20_base::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use serde::Serialize;

#[boot_contract(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw20;

// implement chain-generic functions
impl<Chain: BootEnvironment> Cw20<Chain> {
    pub fn new(id: &str, chain: Chain) -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let file_path = &format!("{}{}", crate_path, "/cw-artifacts/cw20_base.wasm");
        Self(
            Contract::new(id, chain)
                .with_mock(Box::new(ContractWrapper::new_with_empty(
                    cw20_base::contract::execute,
                    cw20_base::contract::instantiate,
                    cw20_base::contract::query,
                )))
                .with_wasm_path(file_path),
        )
    }

    // Find a way to generate these functions with a macro!!!
    pub fn send(
        &self,
        msg: &impl Serialize,
        amount: u128,
        contract: String,
    ) -> Result<TxResponse<Chain>, BootError> {
        let msg = ExecuteMsg::Send {
            contract,
            amount: Uint128::new(amount),
            msg: cosmwasm_std::to_binary(msg)?,
        };

        self.execute(&msg, None)
    }
    pub fn transfer(
        &self,
        amount: u128,
        recipient: String,
    ) -> Result<TxResponse<Chain>, BootError> {
        let msg = ExecuteMsg::Transfer {
            recipient,
            amount: amount.into(),
        };
        self.execute(&msg, None)
    }

    pub fn create_new<T: Into<Uint128>>(
        &self,
        minter: &Addr,
        balance: T,
    ) -> Result<TxResponse<Chain>, BootError> {
        let msg = InstantiateMsg {
            decimals: 6,
            mint: Some(MinterResponse {
                cap: None,
                minter: minter.to_string(),
            }),
            symbol: "TEST".into(),
            name: self.0.id.to_string(),
            initial_balances: vec![Cw20Coin {
                address: minter.to_string(),
                amount: balance.into(),
            }],
            marketing: None,
        };

        self.instantiate(&msg, Some(minter), None)
    }

    pub fn balance(&self, address: &Addr) -> Result<Uint128, BootError> {
        let bal: BalanceResponse = self.query(&QueryMsg::Balance {
            address: address.to_string(),
        })?;
        Ok(bal.balance)
    }

    pub fn mint(&self, recipient: impl Into<String>, amount: u128) -> Result<TxResponse<Chain>, BootError> {
        let msg = ExecuteMsg::Mint {
            recipient: recipient.into(),
            amount: Uint128::new(amount),
        };
        self.execute(&msg, None)
    }

    pub fn increase_allowance(
        &self,
        spender: impl Into<String>,
        amount: u128,
        expires: Option<Expiration>,
    ) -> Result<TxResponse<Chain>, BootError> {
        let msg = ExecuteMsg::IncreaseAllowance {
            spender: spender.into(),
            amount: Uint128::new(amount),
            expires,
        };
        self.execute(&msg, None)
    }
}
// Todo: make into derive macro
pub trait Cw20Send<Chain: BootEnvironment>: BootExecute<Chain, ExecuteMsg = ExecuteMsg> {
    fn send(
        &self,
        msg: Binary,
        amount: u128,
        contract: String,
    ) -> Result<TxResponse<Chain>, BootError>;
}

impl<T, Chain: BootEnvironment> Cw20Send<Chain> for T
where
    T: BootExecute<Chain, ExecuteMsg = ExecuteMsg>,
{
    fn send(
        &self,
        msg: Binary,
        amount: u128,
        contract: String,
    ) -> Result<TxResponse<Chain>, BootError> {
        let msg = ExecuteMsg::Send {
            contract,
            amount: Uint128::new(amount),
            msg,
        };
        self.execute(&msg, None)
    }
}
