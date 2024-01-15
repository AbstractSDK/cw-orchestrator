use std::{cell::RefCell, rc::Rc};

use cosmwasm_std::{Coin, Empty};
use cw_multi_test::BasicApp;
use cw_orch_core::{
    environment::{
        queriers::bank::{BankQuerier, BankQuerierGetter},
        StateInterface, TxHandler,
    },
    CwEnvError,
};

use crate::Mock;

pub struct MockBankQuerier {
    app: Rc<RefCell<BasicApp<Empty, Empty>>>,
}

impl MockBankQuerier {
    fn new<S: StateInterface>(mock: &Mock<S>) -> Self {
        Self {
            app: mock.app.clone(),
        }
    }
}

impl<S: StateInterface> BankQuerierGetter<<Self as TxHandler>::Error> for Mock<S> {
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
            let amount = self
                .app
                .borrow()
                .wrap()
                .query_balance(address, denom.clone())?
                .amount;
            Ok(vec![Coin { amount, denom }])
        } else {
            let amount = self.app.borrow().wrap().query_all_balances(address)?;
            Ok(amount)
        }
    }

    fn supply_of(&self, denom: impl Into<String>) -> Result<cosmwasm_std::Coin, Self::Error> {
        Ok(self.app.borrow().wrap().query_supply(denom)?)
    }

    fn total_supply(&self) -> Result<Vec<cosmwasm_std::Coin>, Self::Error> {
        unimplemented!()
    }
}
