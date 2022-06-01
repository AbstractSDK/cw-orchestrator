use std::rc::Rc;

use cosmwasm_std::{Binary, Empty, Uint128};
use cw20::*;
use secp256k1::All;

use terra_rust_api::client::tx_types::{TXResultSync, V1TXResult};
use terra_rust_script::{
    contract::ContractInstance,
    error::TerraRustScriptError,
    sender::{GroupConfig, Wallet},
    traits::{Instance, Interface, WasmExecute},
};
use terraswap::token::InstantiateMsg;

pub struct CW20<'a>(ContractInstance<'a>);

impl Interface for CW20<'_> {
    type E = Cw20ExecuteMsg;

    type I = InstantiateMsg;

    type Q = Cw20QueryMsg;

    type M = Empty;
}

impl Instance for CW20<'_> {
    fn instance(&self) -> &ContractInstance {
        &self.0
    }
}

impl CW20<'_> {
    pub fn new<'a>(
        name: &'a str,
        sender: &'a Rc<terra_rust_script::sender::Sender<All>>,
        group_config: &'a GroupConfig,
    ) -> anyhow::Result<CW20<'a>> {
        Ok(CW20(ContractInstance::new(name, sender, group_config)?))
    }

    pub async fn send(
        &self,
        msg: Binary,
        amount: u128,
        contract: String,
    ) -> Result<V1TXResult, TerraRustScriptError> {
        let msg = Cw20ExecuteMsg::Send {
            contract,
            amount: Uint128::new(amount),
            msg,
        };

        self.exec(&msg, None).await
    }
}
