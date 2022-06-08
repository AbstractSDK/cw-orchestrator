use cosmwasm_std::{Binary, Uint128};
use cw20::*;

use cosm_script::{
    Deployment, contract::ContractInstance, error::CosmScriptError, sender::Wallet,
    traits::*, CosmTxResponse,
};
use cw20_base::msg::InstantiateMsg;

pub struct CW20<'a>(ContractInstance<'a>);

impl Interface for CW20<'_> {
    type Exec = Cw20ExecuteMsg;

    type Init = InstantiateMsg;

    type Query = Cw20QueryMsg;

    type Migrate = NotImplemented;
}

impl Instance for CW20<'_> {
    fn instance(&self) -> &ContractInstance {
        &self.0
    }
}

impl WasmContract<'_> for CW20<'_> {}

impl CW20<'_> {
    pub fn new<'a>(
        name: &'a str,
        sender: Wallet<'a>,
        deployment: &'a Deployment,
    ) -> anyhow::Result<CW20<'a>> {
        Ok(CW20(ContractInstance::new(name, sender, deployment)?))
    }

    pub async fn send(
        &self,
        msg: Binary,
        amount: u128,
        contract: String,
    ) -> Result<CosmTxResponse, CosmScriptError> {
        let msg = Cw20ExecuteMsg::Send {
            contract,
            amount: Uint128::new(amount),
            msg,
        };

        self.exec(&msg, None).await
    }
}
