use boot_core::{
    contract::{Contract, ContractCodeReference},
    index_response::IndexResponse,
    state::StateInterface,
    tx_handler::{TxHandler, TxResponse},
    BootError, Daemon, Mock,
};
use cosmwasm_std::{Addr, Binary, Empty, Uint128};
use serde::Serialize;
use std::{fmt::Debug, ops::Deref};
use crate::MyProjectName;

pub type Cw20<Chain> = CwPlusContract<Chain, ExecuteMsg, InstantiateMsg, QueryMsg, Empty>;

// implement chain-generic functions
impl<Chain: BootEnvironment + Clone> Cw20<Chain>
where
TxResponse<Chain>: IndexResponse,
{
    pub fn new(name: &str, chain: &Chain) -> Self {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let file_path = &format!("{}{}",crate_path,"/cw-artifacts/cw20_base.wasm");
        Self(Contract::new(name, chain).with_mock(Box::new(
            ContractWrapper::new_with_empty(
                cw20_base::contract::execute,
                cw20_base::contract::instantiate,
                cw20_base::contract::query,
            ),
        )).with_wasm_path(file_path))
    }

    // If you have access to the msgs these can be generated for you with a macro!
    pub fn send(
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
    pub fn transfer(&self,
        amount: u128,
        recipient: String) -> Result<TxResponse<Chain>, BootError> {
        let msg = ExecuteMsg::Transfer { recipient, amount: amount.into() };
        self.execute(&msg,None)
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
            name: self.0.name.to_string(),
            initial_balances: vec![Cw20Coin {
                address: minter.to_string(),
                amount: balance.into(),
            }],
            marketing: None,
        };

        self.instantiate(&msg, Some(minter), None)
    }

    pub fn balance(&self, address: &Addr) -> Result<Uint128, BootError> {
        let bal: BalanceResponse =self.query(&QueryMsg::Balance {
            address: address.to_string(),
        })?;
        Ok(bal.balance)
    }

    pub fn test_generic(&self, sender: &Addr) -> Result<(),BootError> {
        // Instantiate the contract using a custom function
        let resp = self.create_new(sender, 420u128)?;
        // Access the execution result
        println!("events: {:?}", resp.events());
        // get the user balance and assert for testing purposes
        let new_balance = self.balance(sender)?;
        // balance == mint balance
        assert_eq!(420u128, new_balance.u128());
        // BURNNNN
        self.execute(
            &cw20::Cw20ExecuteMsg::Burn {
                amount: 96u128.into(),
            },
            None,
        )?;
        let token_info: cw20::TokenInfoResponse =
            self.query(&cw20_base::msg::QueryMsg::TokenInfo {})?;
        println!("token_info: {:?}", token_info);
        Ok(())
    }
}