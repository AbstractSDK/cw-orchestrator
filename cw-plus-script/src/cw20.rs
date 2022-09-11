use cosm_script::{
    contract::{Contract, ContractCodeReference, ContractSource},
    index_response::IndexResponse,
    state::StateInterface,
    tx_handler::{TxHandler, TxResponse},
    CosmScriptError, Daemon, Mock,
};
use cosmwasm_std::{Addr, Binary, Empty, Uint128};

use cw20::{Cw20Coin, MinterResponse};
use cw20_base::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cw_multi_test::ContractWrapper;

use crate::CwPlusContract;

pub type Cw20<Chain> = CwPlusContract<Chain, ExecuteMsg, InstantiateMsg, QueryMsg, Empty>;

// implement chain-generic functions
impl<Chain: TxHandler + Clone> Cw20<Chain>
where
    TxResponse<Chain>: IndexResponse,
{
    pub fn new(name: &str, chain: &Chain) -> Self {
        Self {
            contract: Contract::new(name, chain),
        }
    }
    pub fn send(
        &self,
        msg: Binary,
        amount: u128,
        contract: String,
    ) -> Result<TxResponse<Chain>, CosmScriptError> {
        let msg = ExecuteMsg::Send {
            contract,
            amount: Uint128::new(amount),
            msg,
        };

        self.execute(&msg, None)
    }

    pub fn create_new<T: Into<Uint128>>(
        &self,
        minter: &Addr,
        balance: T,
    ) -> Result<TxResponse<Chain>, CosmScriptError> {
        let msg = InstantiateMsg {
            decimals: 6,
            mint: Some(MinterResponse {
                cap: None,
                minter: minter.to_string(),
            }),
            symbol: "TEST".into(),
            name: self.contract.name.to_string(),
            initial_balances: vec![Cw20Coin {
                address: minter.to_string(),
                amount: balance.into(),
            }],
            marketing: None,
        };

        self.instantiate(&msg, Some(minter), None)
    }

    pub fn balance(&self, address: &Addr) -> Result<Uint128, CosmScriptError> {
        self.query(&QueryMsg::Balance {
            address: address.to_string(),
        })
    }
}

impl ContractSource for Cw20<Daemon> {
    fn source(&self) -> ContractCodeReference {
        ContractCodeReference::WasmCodePath("cw20_base")
    }
}
impl ContractSource for Cw20<Mock> {
    fn source(&self) -> ContractCodeReference {
        cosm_script::contract::ContractCodeReference::ContractEndpoints(Box::new(
            ContractWrapper::new_with_empty(
                cw20_base::contract::execute,
                cw20_base::contract::instantiate,
                cw20_base::contract::query,
            ),
        ))
    }
}
// impl <S:StateInterface>Cw20<Mock<S>>
// {
//     pub fn source(&self) -> ContractCodeReference<Empty> {
//         let cw20_token_contract = Box::new(ContractWrapper::new_with_empty(
//             cw20_base::contract::execute,
//             cw20_base::contract::instantiate,
//             cw20_base::contract::query,
//         ));
//         ContractCodeReference::ContractEndpoints(cw20_token_contract)
//     }
// }

// impl<Chain: TxRe> CW20<Chain> {
//     /// Send tokens to a contract allong with a contract call
//     pub async fn send(
//         &self,
//         msg: Binary,
//         amount: u128,
//         contract: String,
//     ) -> Result<CosmTxResponse, CosmScriptError> {
//         let msg = ExecuteMsg::Send {
//             contract,
//             amount: Uint128::new(amount),
//             msg,
//         };

//         self.exec(&msg, None).await
//     }

//     /// Instantiate a new token instance with some initial balance given to the minter
//     pub async fn create_new<T: Into<Uint128>>(
//         &self,
//         minter: String,
//         balance: T,
//     ) -> Result<CosmTxResponse, CosmScriptError> {
//         let msg = InstantiateMsg {
//             decimals: 6,
//             mint: Some(MinterResponse {
//                 cap: None,
//                 minter: minter.clone(),
//             }),
//             symbol: self.instance().name.to_ascii_uppercase(),
//             name: self.instance().name.to_string(),
//             initial_balances: vec![Cw20Coin {
//                 address: minter.clone(),
//                 amount: balance.into(),
//             }],
//             marketing: None,
//         };

//         self.init(msg, Some(minter), None).await
//     }
// }
