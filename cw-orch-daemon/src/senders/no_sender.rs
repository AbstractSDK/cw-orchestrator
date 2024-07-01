use crate::{error::DaemonError, tx_resp::CosmTxResponse, DaemonBase, GrpcChannel};

use cosmrs::{AccountId, Any};
use cosmwasm_std::Addr;
use cw_orch_core::environment::ChainInfoOwned;

use tonic::transport::Channel;

use super::{builder::SenderBuilder, query::QuerySender, tx::TxSender};

/// Daemon that does not support signing.
/// Will err on any attempt to sign a transaction or retrieve a sender address.
pub type QueryOnlyDaemon = DaemonBase<NoSender>;

/// Signer of the transactions and helper for address derivation
#[derive(Clone)]
pub struct NoSender {
    /// gRPC channel
    pub channel: Channel,
    /// Information about the chain
    pub chain_info: ChainInfoOwned,
}

impl SenderBuilder for NoSender {
    type Error = DaemonError;
    type Options = ();

    async fn build(
        chain_info: ChainInfoOwned,
        _sender_options: Self::Options,
    ) -> Result<Self, Self::Error> {
        let channel = GrpcChannel::from_chain_info(&chain_info).await?;

        Ok(NoSender {
            channel,
            chain_info,
        })
    }
}

impl QuerySender for NoSender {
    fn chain_info(&self) -> &ChainInfoOwned {
        &self.chain_info
    }

    fn grpc_channel(&self) -> Channel {
        self.channel.clone()
    }

    fn set_options(&mut self, _options: Self::Options) {}
}
