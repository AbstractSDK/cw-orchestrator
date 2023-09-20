use cosmwasm_std::{coins, Addr, coin, Binary};
use cw_multi_test::AppResponse;
use cw_orch_core::{environment::TxHandler, CwEnvError};
use osmosis_std::types::cosmos::bank::v1beta1::{QueryBalanceRequest, QueryAllBalancesRequest};
use osmosis_test_tube::{Bank, Module};
use osmosis_std::{
    cosmwasm_to_proto_coins,
    types::cosmos::bank::v1beta1::MsgSend,
};

use crate::{prelude::OsmosisTestTube, osmosis_test_tube::map_err};

impl cw_orch_core::environment::modules::Bank for OsmosisTestTube{
    fn send(&self, recipient: Addr, funds: Vec<cosmwasm_std::Coin>) -> Result<AppResponse, CwEnvError> {
   
        let send_response = Bank::new(&*self.app.borrow())
            .send(
                MsgSend {
                    from_address: self.sender().to_string(),
                    to_address: recipient.to_string(),
                    amount: cosmwasm_to_proto_coins(funds),
                },
                &self.sender,
            )
            .map_err(map_err)?;

        Ok(AppResponse {
            data: Some(Binary(send_response.raw_data)),
            events: send_response.events,
        })
    }

    fn balance(&self, denom: Option<String>) -> Result<Vec<cosmwasm_std::Coin>, <Self as cw_orch_core::environment::TxHandler>::Error> {
        match denom{
            Some(denom) => {
                Ok(Bank::new(&*self.app.borrow())
                    .query_balance(&QueryBalanceRequest {
                        address: self.sender().to_string(),
                        denom: denom.clone(),
                    })
                    .unwrap()
                    .balance
                    .map(|b| coins(b.amount.parse().unwrap(), b.denom))
                    .unwrap_or(coins(0, denom.clone())))
            },
            None => {
                Ok(Bank::new(&*self.app.borrow())
                    .query_all_balances(&QueryAllBalancesRequest {
                        address: self.sender().to_string(),
                        pagination: None,
                    })
                    .unwrap()
                    .balances
                    .iter()
                    .map(|b| coin(b.amount.parse().unwrap(), b.denom.clone()))
                    .collect()
                )
            },
        }
    }
}

/*
impl WalletBalanceAssertion for OsmosisTestTube {
    fn _assert_wallet_balance(
        &self,
        gas: u64,
    ) -> Result<WalletBalanceAssertionResult, CwOrchError> {
        let fee = self.sender.fee_setting();

        let fee = match fee {
            FeeSetting::Auto {
                gas_price,
                gas_adjustment,
            } => coin(
                (((gas as f64) * gas_adjustment) as u128) * gas_price.amount.u128(),
                gas_price.denom.clone(),
            ),
            FeeSetting::Custom { amount, .. } => amount.clone(),
        };

        // Will be simplified when the bank trait will exist
        let balance = Bank::new(&*self.app.borrow())
            .query_balance(&QueryBalanceRequest {
                address: self.sender().to_string(),
                denom: fee.denom.clone(),
            })
            .unwrap()
            .balance
            .map(|b| coin(b.amount.parse().unwrap(), b.denom))
            .unwrap_or(coin(0, fee.denom.clone()));

        log::debug!(
            "Checking balance {} on chain {}, address {}. Expecting {}",
            balance.amount,
            "osmosis test tube",
            self.sender(),
            fee
        );

        Ok(WalletBalanceAssertionResult {
            expected: fee.clone(),
            current: balance.clone(),
            assertion: balance.amount >= fee.amount,
        })
    }
} */