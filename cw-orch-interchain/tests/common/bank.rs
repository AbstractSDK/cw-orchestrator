use cosmwasm_std::{BankMsg, Coin, CosmosMsg};
use cw_orch::prelude::*;

pub trait BankModule: TxHandler {
    fn send(
        &self,
        receiver: &Addr,
        funds: Vec<Coin>,
    ) -> Result<<Self as TxHandler>::Response, <Self as TxHandler>::Error>;
}

impl BankModule for Mock {
    fn send(
        &self,
        receiver: &Addr,
        funds: Vec<Coin>,
    ) -> Result<<Self as TxHandler>::Response, <Self as TxHandler>::Error> {
        let app_responses = self
            .app
            .borrow_mut()
            .execute_multi(
                self.sender.clone(),
                vec![CosmosMsg::Bank(BankMsg::Send {
                    to_address: receiver.to_string(),
                    amount: funds,
                })],
            )
            .unwrap();
        Ok(app_responses[0].clone().into())
    }
}

impl BankModule for Daemon {
    fn send(
        &self,
        recipient: &Addr,
        funds: Vec<Coin>,
    ) -> Result<<Self as TxHandler>::Response, <Self as TxHandler>::Error> {
        self.rt_handle
            .block_on(self.sender().bank_send(recipient, funds))
    }
}
