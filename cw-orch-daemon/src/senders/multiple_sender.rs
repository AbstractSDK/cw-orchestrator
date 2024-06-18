use crate::{DaemonBase, Wallet, INSTANTIATE_2_TYPE_URL};

use crate::{error::DaemonError, tx_resp::CosmTxResponse};

use cosmrs::proto::cosmwasm::wasm::v1::{MsgInstantiateContract, MsgStoreCode};
use cosmrs::{AccountId, Any};
use cosmwasm_std::Addr;
use cw_orch_core::log::transaction_target;
use prost::Name;

use std::sync::{Arc, Mutex};

use super::{base_sender::SenderOptions, sender_trait::SenderTrait};

pub type MultiDaemon = DaemonBase<MultipleSender>;

/// Signer of the transactions and helper for address derivation
/// This is the main interface for simulating and signing transactions
#[derive(Clone)]
pub struct MultipleSender {
    /// Contains the different messages to broadcast
    /// These are behind an Arc Mutex, because `commit_tx_any function` doesn't have access to a mutable reference to the object
    pub msgs: Arc<Mutex<Vec<Any>>>,
    pub sender: Wallet,
}

impl SenderTrait for MultipleSender {
    type Error = DaemonError;
    type SenderOptions = SenderOptions;

    async fn commit_tx_any(
        &self,
        msgs: Vec<Any>,
        memo: Option<&str>,
    ) -> Result<CosmTxResponse, DaemonError> {
        // We check the type URLS. We can safely put them inside the lock if they DON'T correspond to the following:
        // - Code Upload
        // - Contract Instantiation (1 and 2)

        let broadcast_immediately_type_urls = [
            MsgStoreCode::type_url(),
            MsgInstantiateContract::type_url(),
            INSTANTIATE_2_TYPE_URL.to_string(),
        ];

        let broadcast_immediately = msgs
            .iter()
            .any(|msg| broadcast_immediately_type_urls.contains(&msg.type_url));

        if broadcast_immediately {
            self.sender.commit_tx_any(msgs, memo).await
        } else {
            log::info!(
                target: &transaction_target(),
                "Transaction not sent, use `DaemonBase::wallet().broadcast(), to broadcast the batched transactions",
            );
            let mut msg_storage = self.msgs.lock().unwrap();
            msg_storage.extend(msgs);

            Ok(CosmTxResponse::default())
        }
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
    pub async fn broadcast(&self, memo: Option<&str>) -> Result<CosmTxResponse, DaemonError> {
        let msgs = self.msgs.lock().unwrap().to_vec();
        log::info!(
            target: &transaction_target(),
            "[Broadcast] {} msgs in a single transaction",
            msgs.len()
        );
        let tx_result = self.sender.commit_tx_any(msgs, memo).await?;
        log::info!(
            target: &transaction_target(),
            "[Broadcasted] Success: {}",
            tx_result.txhash
        );

        let mut msgs_to_empty = self.msgs.lock().unwrap();
        *msgs_to_empty = vec![];

        Ok(tx_result)
    }
}
