use cosmrs::{
    tx::{Msg, Raw},
    AccountId, Any,
};
use cosmwasm_std::Addr;

use crate::{CosmTxResponse, DaemonError, DaemonState};
use std::sync::Arc;

use super::base_sender::SenderOptions;

pub trait SenderTrait: Clone {
    type Error: Into<DaemonError> + std::error::Error + std::fmt::Debug + Send + Sync + 'static;

    // TODO: do we want to enforce sync on this function ?
    fn address(&self) -> Result<Addr, Self::Error>;

    // TODO: do we want to enforce sync on this function ?
    /// Returns the actual sender of every message sent.
    /// If an authz granter is set, returns the authz granter
    /// Else, returns the address associated with the current private key
    fn msg_sender(&self) -> Result<AccountId, Self::Error>;

    fn commit_tx<T: Msg>(
        &self,
        msgs: Vec<T>,
        memo: Option<&str>,
    ) -> impl std::future::Future<Output = Result<CosmTxResponse, Self::Error>> + Send {
        let msgs = msgs
            .into_iter()
            .map(Msg::into_any)
            .collect::<Result<Vec<Any>, _>>()
            .unwrap();

        self.commit_tx_any(msgs, memo)
    }

    fn commit_tx_any(
        &self,
        msgs: Vec<Any>,
        memo: Option<&str>,
    ) -> impl std::future::Future<Output = Result<CosmTxResponse, Self::Error>> + Send;

    fn broadcast_tx(
        &self,
        tx: Raw,
    ) -> impl std::future::Future<
        Output = Result<cosmrs::proto::cosmos::base::abci::v1beta1::TxResponse, Self::Error>,
    > + Send;

    fn build(sender_options: SenderOptions, state: &Arc<DaemonState>) -> Result<Self, Self::Error>;
}

impl<T: SenderTrait> SenderTrait for Arc<T> {
    type Error = T::Error;

    fn address(&self) -> Result<Addr, Self::Error> {
        (**self).address()
    }

    fn msg_sender(&self) -> Result<AccountId, Self::Error> {
        (**self).msg_sender()
    }

    fn commit_tx_any(
        &self,
        msgs: Vec<Any>,
        memo: Option<&str>,
    ) -> impl std::future::Future<Output = Result<CosmTxResponse, Self::Error>> + Send {
        (**self).commit_tx_any(msgs, memo)
    }

    fn broadcast_tx(
        &self,
        tx: Raw,
    ) -> impl std::future::Future<
        Output = Result<cosmrs::proto::cosmos::base::abci::v1beta1::TxResponse, Self::Error>,
    > + Send {
        (**self).broadcast_tx(tx)
    }

    fn build(sender_options: SenderOptions, state: &Arc<DaemonState>) -> Result<Self, Self::Error> {
        Ok(Arc::new(T::build(sender_options, state)?))
    }
}
