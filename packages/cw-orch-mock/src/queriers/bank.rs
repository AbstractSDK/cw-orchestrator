use std::{cell::RefCell, rc::Rc};

use cosmwasm_std::{Addr, Api, Coin};
use cw_orch_core::{
    environment::{
        QuerierGetter, StateInterface, {BankQuerier, Querier},
    },
    CwEnvError,
};

use crate::{core::MockApp, MockBase};

pub struct MockBankQuerier<A> {
    app: Rc<RefCell<MockApp<A>>>,
}

impl<A: Api> MockBankQuerier<A> {
    fn new<S: StateInterface>(mock: &MockBase<A, S>) -> Self {
        Self {
            app: mock.app.clone(),
        }
    }
}

impl<A: Api, S: StateInterface> QuerierGetter<MockBankQuerier<A>> for MockBase<A, S> {
    fn querier(&self) -> MockBankQuerier<A> {
        MockBankQuerier::new(self)
    }
}

impl<A: Api> Querier for MockBankQuerier<A> {
    type Error = CwEnvError;
}

impl<A: Api> BankQuerier for MockBankQuerier<A> {
    fn balance(
        &self,
        address: &Addr,
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
