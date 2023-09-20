use cosmwasm_std::{Coin, Addr};
use cw_orch_core::environment::modules::Bank;

use crate::{Daemon, DaemonError};

impl Bank for Daemon{
    fn send(&self, recipient: Addr, funds: Vec<cosmwasm_std::Coin>) -> Result<<Self as cw_orch_core::environment::TxHandler>::Response, DaemonError> {
        self.rt_handle.block_on(
            self.wallet().bank_send(recipient.as_ref(), funds)
        )
    }

    fn balance(&self, denom: Option<String>) -> Result<Vec<cosmwasm_std::Coin>, <Self as cw_orch_core::environment::TxHandler>::Error> {
        let bank = self.query_client::<crate::queriers::Bank>();
        let balance = self
            .rt_handle
            .block_on(bank.balance(self.daemon.sender(), denom))
            .unwrap()
            .clone();

        balance.iter().map(|c|Ok(Coin{
            amount: c.amount.parse()?,
            denom: c.denom.to_string()
        })).collect::<Result<Vec<_>,_>>()
    }
}