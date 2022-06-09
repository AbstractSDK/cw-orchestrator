use cosmwasm_std::{Binary, Uint128};

use cosm_script::{
    Deployment, contract::ContractInstance, error::CosmScriptError, sender::Wallet,
    traits::*, CosmTxResponse,
};
use cw20::{MinterResponse, Cw20Coin};
use cw20_base::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

pub struct CW20<'a>(ContractInstance<'a>);

impl Interface for CW20<'_> {
    type Exec = ExecuteMsg;

    type Init = InstantiateMsg;

    type Query = QueryMsg;

    type Migrate = NotImplemented;
}

impl Instance for CW20<'_> {
    fn instance(&self) -> &ContractInstance {
        &self.0
    }
}

impl WasmContract<'_> for CW20<'_> {}

impl CW20<'_> {
    /// Create a new CW20 ContractInstance. Uses "cw20" as code-id key.
    pub fn new<'a>(
        name: &'a str,
        sender: Wallet<'a>,
        deployment: &'a Deployment,
    ) -> anyhow::Result<CW20<'a>> {
        let mut instance = ContractInstance::new(name, sender, deployment)?;
        // We want all our CW20 tokens to use the same contract!
        instance.overwrite_code_id_key("cw20");
        Ok(CW20(instance))
    }
    /// Send tokens to a contract allong with a contract call
    pub async fn send(
        &self,
        msg: Binary,
        amount: u128,
        contract: String,
    ) -> Result<CosmTxResponse, CosmScriptError> {
        let msg = ExecuteMsg::Send {
            contract,
            amount: Uint128::new(amount),
            msg,
        };

        self.exec(&msg, None).await
    }

    pub async fn create_new<T: Into<Uint128>>(
        &self,
        minter: String,
        balance: T,
    ) -> Result<CosmTxResponse, CosmScriptError> {

        let msg = InstantiateMsg {
            decimals: 6,
            mint: Some(MinterResponse {
                cap: None,
                minter: minter.clone(),
            }),
            symbol: "TESTING".to_string(),
            name: self.instance().name.to_string(),
            initial_balances: vec![Cw20Coin {
                address: minter.clone(),
                amount: balance.into(),
            }],
            marketing: None,
        };

        self.init(msg, Some(minter), None).await
    }
}
