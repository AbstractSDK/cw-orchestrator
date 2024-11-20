use crate::{DaemonBase, INSTANTIATE_2_TYPE_URL};
use crate::{error::DaemonError, tx_resp::CosmTxResponse};
use cosmrs::proto::cosmwasm::wasm::v1::{MsgInstantiateContract, MsgStoreCode};
use cosmrs::{AccountId, Any};
use cosmwasm_std::Addr;
use cw_orch_core::environment::ChainInfoOwned;
use cw_orch_core::log::transaction_target;
use options::CosmosBatchOptions;
use prost::Name;
use cosmrs::bank::MsgSend;
use std::sync::{Arc, Mutex};
use super::builder::SenderBuilder;
use super::cosmos::Wallet;
use super::query::QuerySender;
use super::tx::TxSender;
use crate::parse_cw_coins;
use std::str::FromStr;

pub type BatchDaemon = DaemonBase<CosmosBatchSender>;

pub mod options {
    use super::super::CosmosOptions;

    #[derive(Clone, Default)]
    pub struct CosmosBatchOptions(pub(crate) CosmosOptions);

    impl From<CosmosOptions> for CosmosBatchOptions {
        fn from(options: CosmosOptions) -> Self {
            Self(options)
        }
    }

    impl CosmosBatchOptions {
        pub fn new(options: CosmosOptions) -> Self {
            Self(options)
        }
    }
}

/// Signer of Message batch transactions
/// This is a wrapper around the `Wallet` struct, with the addition of a `msgs` field that cache messages before they are sent.
#[derive(Clone)]
pub struct CosmosBatchSender {
    /// Contains the different messages to broadcast
    pub msgs: Arc<Mutex<Vec<Any>>>,
    pub sender: Wallet,
}

impl CosmosBatchSender {
    /// Broadcast the cached messages in a transaction.
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

impl SenderBuilder for CosmosBatchOptions {
    type Error = DaemonError;
    type Sender = CosmosBatchSender;

    async fn build(&self, chain_info: &Arc<ChainInfoOwned>) -> Result<Self::Sender, Self::Error> {
        Ok(CosmosBatchSender {
            msgs: Default::default(),
            sender: self.0.build(chain_info).await?,
        })
    }
}

impl QuerySender for CosmosBatchSender {
    type Error = DaemonError;
    type Options = CosmosBatchOptions;

    fn channel(&self) -> tonic::transport::Channel {
        self.sender.channel()
    }
}

impl TxSender for CosmosBatchSender {
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

    fn address(&self) -> Addr {
        self.sender.address()
    }

    fn account_id(&self) -> AccountId {
        self.sender.account_id()
    }

    async fn bank_send(
        &self,
        recipient: &Addr,
        coins: &[cosmwasm_std::Coin],
    ) -> Result<CosmTxResponse, DaemonError> {
        let acc_id = self.msg_sender()?;

        let msg_send = MsgSend {
            from_address: acc_id,
            to_address: AccountId::from_str(recipient.as_str())?,
            amount: parse_cw_coins(coins)?,
        };

        self.commit_tx(vec![msg_send], Some("sending tokens")).await
    }
}
