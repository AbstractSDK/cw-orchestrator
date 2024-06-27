use crate::{error::DaemonError, tx_resp::CosmTxResponse, DaemonBase};

use cosmrs::{AccountId, Any};
use cosmwasm_std::Addr;
use cw_orch_core::environment::ChainInfoOwned;

use tonic::transport::Channel;

use super::sender_trait::SenderTrait;

pub type QuerierDaemon = DaemonBase<NoSender>;

/// Signer of the transactions and helper for address derivation
/// This is the main interface for simulating and signing transactions
#[derive(Clone)]
pub struct NoSender {
    /// gRPC channel
    pub grpc_channel: Channel,
    /// Information about the chain
    pub chain_info: ChainInfoOwned,
}

impl SenderTrait for NoSender {
    type Error = DaemonError;
    type SenderOptions = ();

    async fn commit_tx_any(
        &self,
        _msgs: Vec<Any>,
        _memo: Option<&str>,
    ) -> Result<CosmTxResponse, DaemonError> {
        unimplemented!("You used the DaemonQuerier, which can't send transactions");
    }

    fn address(&self) -> Result<Addr, DaemonError> {
        unimplemented!("You used the DaemonQuerier, which doesn't have an associated address");
    }

    fn msg_sender(&self) -> Result<AccountId, DaemonError> {
        unimplemented!("You used the DaemonQuerier, which doesn't have an associated msg sender");
    }

    fn chain_info(&self) -> &ChainInfoOwned {
        &self.chain_info
    }

    fn grpc_channel(&self) -> Channel {
        self.grpc_channel.clone()
    }

    fn set_options(&mut self, _options: Self::SenderOptions) {}

    fn build(
        chain_info: ChainInfoOwned,
        grpc_channel: Channel,
        _sender_options: Self::SenderOptions,
    ) -> Result<Self, Self::Error> {
        Ok(NoSender {
            grpc_channel,
            chain_info,
        })
    }
}
