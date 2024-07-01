
use cosmrs::{tx::Msg, AccountId, Any};
use cosmwasm_std::Addr;
use cw_orch_core::environment::ChainInfoOwned;
use tonic::transport::Channel;

use crate::{CosmTxResponse, DaemonError};

pub trait QuerySender: Clone {
    type Error: Into<DaemonError> + std::error::Error + std::fmt::Debug + Send + Sync + 'static;
    /// Options for the sender
    type SenderOptions: Default + Clone;
    
    /// Build a new `Sender`.
    fn build(
        chain_info: ChainInfoOwned,
        grpc_channel: Channel,
        sender_options: Self::SenderOptions,
    ) -> Result<Self, Self::Error>;

    /// Set the Sender options
    fn set_options(&mut self, options: Self::SenderOptions);

    /// Get the address of the sender.
    fn address(&self) -> Result<Addr, Self::Error>;

    /// Get the chain_information for the sender
    fn chain_info(&self) -> &ChainInfoOwned;

    /// Get the channel for the sender (TODO: return mut ref to Retry Sender)
    fn grpc_channel(&self) -> Channel;

    /// Returns the `AccountId` of the sender.
    /// If an authz granter is set, returns the authz granter
    /// Else, returns the address associated with the current private key
    fn msg_sender(&self) -> Result<AccountId, Self::Error>;

    /// Commit a transaction to the chain using this sender.
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

    /// Commit a proto `Any` message to the chain using this sender.
    fn commit_tx_any(
        &self,
        msgs: Vec<Any>,
        memo: Option<&str>,
    ) -> impl std::future::Future<Output = Result<CosmTxResponse, Self::Error>> + Send;

}
