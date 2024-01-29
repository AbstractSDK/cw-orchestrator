use std::{cell::RefCell, rc::Rc};

use cosmwasm_std::coin;
use cw_orch_core::{
    environment::{
        queriers::bank::{BankQuerier, BankQuerierGetter},
        StateInterface, TxHandler,
    },
    CwEnvError,
};
use osmosis_std::try_proto_to_cosmwasm_coins;
use osmosis_std::types::cosmos::bank::v1beta1::{QuerySupplyOfRequest, QuerySupplyOfResponse};
use osmosis_test_tube::{Bank, Module, OsmosisTestApp, Runner};

use crate::osmosis_test_tube::{map_err, OsmosisTestTube};
use osmosis_test_tube::osmosis_std::types::cosmos::bank::v1beta1::{
    QueryAllBalancesRequest, QueryBalanceRequest,
};
pub struct MockBankQuerier {
    app: Rc<RefCell<OsmosisTestApp>>,
}

impl MockBankQuerier {
    fn new<S: StateInterface>(mock: &OsmosisTestTube<S>) -> Self {
        Self {
            app: mock.app.clone(),
        }
    }
}

impl<S: StateInterface> BankQuerierGetter<<Self as TxHandler>::Error> for OsmosisTestTube<S> {
    type Querier = MockBankQuerier;

    fn bank_querier(&self) -> Self::Querier {
        MockBankQuerier::new(self)
    }
}

impl BankQuerier for MockBankQuerier {
    type Error = CwEnvError;

    fn balance(
        &self,
        address: impl Into<String>,
        denom: Option<String>,
    ) -> Result<Vec<cosmwasm_std::Coin>, Self::Error> {
        if let Some(denom) = denom {
            let amount = Bank::new(&*self.app.borrow())
                .query_balance(&QueryBalanceRequest {
                    address: address.into(),
                    denom: denom.to_string(),
                })
                .map_err(map_err)?
                .balance
                .map(|c| {
                    let coins = try_proto_to_cosmwasm_coins(vec![c])?[0].clone();
                    Ok::<_, CwEnvError>(coins)
                })
                .transpose()?
                .unwrap_or(coin(0, &denom));
            Ok(vec![amount])
        } else {
            let amount = Bank::new(&*self.app.borrow())
                .query_all_balances(&QueryAllBalancesRequest {
                    address: address.into(),
                    pagination: None,
                })
                .map_err(map_err)?
                .balances;

            Ok(try_proto_to_cosmwasm_coins(amount)?)
        }
    }

    fn supply_of(&self, denom: impl Into<String>) -> Result<cosmwasm_std::Coin, Self::Error> {
        let denom: String = denom.into();
        let supply_of_result: QuerySupplyOfResponse = self
            .app
            .borrow()
            .query(
                "/cosmos.bank.v1beta1.Query/SupplyOf",
                &QuerySupplyOfRequest {
                    denom: denom.clone(),
                },
            )
            .map_err(map_err)?;

        Ok(supply_of_result
            .amount
            .map(|c| {
                // Ok::<_, StdError>(cosmwasm_std::Coin {
                //     amount: c.amount.parse()?,
                //     denom: c.denom,
                // })
                Ok::<_, CwEnvError>(try_proto_to_cosmwasm_coins(vec![c])?[0].clone())
            })
            .transpose()?
            .unwrap_or(coin(0, &denom)))
    }

    fn total_supply(&self) -> Result<Vec<cosmwasm_std::Coin>, Self::Error> {
        unimplemented!()
    }
}
