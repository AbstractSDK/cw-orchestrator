use cosmrs::{tx::Msg, AccountId, Any};
use cosmwasm_std::Addr;
use cw_orch_core::environment::ChainInfoOwned;
use tonic::transport::Channel;

use crate::{CosmTxResponse, DaemonError};

use super::builder::SenderBuilder;

/// A sender that can query information over a connection.
pub trait QuerySender: SenderBuilder {
    /// Set the Sender options
    fn set_options(&mut self, options: Self::Options);

    /// Get the chain_information for the sender
    fn chain_info(&self) -> &ChainInfoOwned;

    /// Get the channel for the sender (TODO: return mut ref to Retry Sender)
    fn grpc_channel(&self) -> Channel;
}
