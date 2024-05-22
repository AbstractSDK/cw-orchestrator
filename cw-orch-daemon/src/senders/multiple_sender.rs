use crate::Wallet;

use crate::{error::DaemonError, state::DaemonState, tx_resp::CosmTxResponse};

use cosmrs::{tx::Raw, AccountId, Any};
use cosmwasm_std::Addr;

use std::sync::{Arc, Mutex};

use super::{base_sender::SenderOptions, sender_trait::SenderTrait};

/// Signer of the transactions and helper for address derivation
/// This is the main interface for simulating and signing transactions
#[derive(Clone)]
pub struct MultipleSender {
    pub msgs: Arc<Mutex<Vec<Any>>>,
    pub sender: Wallet,
}

impl SenderTrait for MultipleSender {
    type Error = DaemonError;

    async fn commit_tx_any(
        &self,
        msgs: Vec<Any>,
        _memo: Option<&str>,
    ) -> Result<CosmTxResponse, DaemonError> {
        let mut msg_storage = self.msgs.lock().unwrap();
        msg_storage.extend(msgs);

        Ok(CosmTxResponse::default())
    }

    async fn broadcast_tx(
        &self,
        _tx: Raw,
    ) -> Result<cosmrs::proto::cosmos::base::abci::v1beta1::TxResponse, Self::Error> {
        unimplemented!()
    }

    fn address(&self) -> Result<Addr, DaemonError> {
        self.sender.address()
    }

    fn msg_sender(&self) -> Result<AccountId, DaemonError> {
        self.sender.msg_sender()
    }

    fn build(sender_options: SenderOptions, state: &Arc<DaemonState>) -> Result<Self, Self::Error> {
        Ok(Self {
            msgs: Default::default(),
            sender: Wallet::build(sender_options, state)?,
        })
    }
}

impl MultipleSender {
    pub async fn flush(&self, memo: Option<&str>) -> Result<CosmTxResponse, DaemonError> {
        let msgs = self.msgs.lock().unwrap().to_vec();
        self.sender.commit_tx_any(msgs, memo).await
    }
}
