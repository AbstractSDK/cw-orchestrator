use cosmrs::{tx::Msg, AccountId, Any};
use cosmwasm_std::Addr;
use cw_orch_core::environment::ChainInfoOwned;
use tonic::transport::Channel;

use crate::{CosmTxResponse, DaemonError};
use std::sync::Arc;

pub trait SenderTrait: Clone {
    type Error: Into<DaemonError> + std::error::Error + std::fmt::Debug + Send + Sync + 'static;
    type SenderOptions: Default + Clone;

    // TODO: do we want to enforce sync on this function ?
    fn address(&self) -> Result<Addr, Self::Error>;

    fn chain_info(&self) -> &ChainInfoOwned;

    fn grpc_channel(&self) -> Channel;

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

    fn set_options(&mut self, options: Self::SenderOptions);

    fn build(
        chain_info: ChainInfoOwned,
        grpc_channel: Channel,
        sender_options: Self::SenderOptions,
    ) -> Result<Self, Self::Error>;
}

impl<T: SenderTrait> SenderTrait for Arc<T> {
    type Error = T::Error;
    type SenderOptions = T::SenderOptions;

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

    fn chain_info(&self) -> &ChainInfoOwned {
        (**self).chain_info()
    }

    fn grpc_channel(&self) -> Channel {
        (**self).grpc_channel()
    }

    fn build(
        chain_info: ChainInfoOwned,
        grpc_channel: Channel,
        sender_options: Self::SenderOptions,
    ) -> Result<Self, Self::Error> {
        Ok(Arc::new(T::build(
            chain_info,
            grpc_channel,
            sender_options,
        )?))
    }

    fn set_options(&mut self, options: Self::SenderOptions) {
        (**self).set_options(options)
    }
}
