use cosmwasm_std::{Binary, Uint128};

use cosm_script::{
    contract::ContractInstance, error::CosmScriptError, sender::Wallet, traits::*, CosmTxResponse,
    Deployment,
};
use cw20::{Cw20Coin, MinterResponse};
use cw20_base::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// Wrapper stuct around the contract instance.
pub struct CW20<'a>(ContractInstance<'a>);

// Interface and instance traits allow for an auto-implementation of our Cosm traits.
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

impl CW20<'_> {
    /// Create a new CW20 ContractInstance. Uses "cw20" as code-id key.
    pub fn new<'a>(
        name: &'a str,
        sender: Wallet<'a>,
        deployment: &'a Deployment,
    ) -> anyhow::Result<CW20<'a>> {
        let mut instance = ContractInstance::new(name, sender, deployment)?;
        // We want all our CW20 tokens to use the same contract (code-id)!
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

    /// Instantiate a new token instance with some initial balance given to the minter
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
            symbol: self.instance().name.to_ascii_uppercase(),
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
