use crate::Wallet;

use crate::{error::DaemonError, tx_resp::CosmTxResponse};

use cosmrs::{AccountId, Any};
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
    type SenderOptions = SenderOptions;

    async fn commit_tx_any(
        &self,
        msgs: Vec<Any>,
        _memo: Option<&str>,
    ) -> Result<CosmTxResponse, DaemonError> {
        let mut msg_storage = self.msgs.lock().unwrap();
        msg_storage.extend(msgs);

        Ok(CosmTxResponse::default())
    }

    fn address(&self) -> Result<Addr, DaemonError> {
        self.sender.address()
    }

    fn msg_sender(&self) -> Result<AccountId, DaemonError> {
        self.sender.msg_sender()
    }

    fn chain_info(&self) -> &cw_orch_core::environment::ChainInfoOwned {
        self.sender.chain_info()
    }

    fn grpc_channel(&self) -> tonic::transport::Channel {
        self.sender.grpc_channel()
    }

    fn build(
        chain_info: cw_orch_core::environment::ChainInfoOwned,
        grpc_channel: tonic::transport::Channel,
        sender_options: Self::SenderOptions,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            msgs: Default::default(),
            sender: Wallet::build(chain_info, grpc_channel, sender_options)?,
        })
    }

    fn set_options(&mut self, options: Self::SenderOptions) {
        self.sender.set_options(options)
    }
}

impl MultipleSender {
    pub async fn flush(&self, memo: Option<&str>) -> Result<CosmTxResponse, DaemonError> {
        let msgs = self.msgs.lock().unwrap().to_vec();
        self.sender.commit_tx_any(msgs, memo).await
    }
}
