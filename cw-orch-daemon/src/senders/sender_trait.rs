use cosmrs::{
    tx::{Msg, Raw},
    AccountId, Any,
};
use cosmwasm_std::Addr;

use crate::{CosmTxResponse, DaemonError};

pub trait SenderTrait: Clone {
    type Error: Into<DaemonError> + std::error::Error + std::fmt::Debug + Send + Sync + 'static;

    // TODO: do we want to enforce sync on this function ?
    fn address(&self) -> Result<Addr, Self::Error>;

    // TODO: do we want to enforce sync on this function ?
    /// Returns the actual sender of every message sent.
    /// If an authz granter is set, returns the authz granter
    /// Else, returns the address associated with the current private key
    fn msg_sender(&self) -> Result<AccountId, Self::Error>;

    async fn commit_tx<T: Msg>(
        &self,
        msgs: Vec<T>,
        memo: Option<&str>,
    ) -> Result<CosmTxResponse, Self::Error> {
        let msgs = msgs
            .into_iter()
            .map(Msg::into_any)
            .collect::<Result<Vec<Any>, _>>()
            .unwrap();

        self.commit_tx_any(msgs, memo).await
    }

    async fn commit_tx_any(
        &self,
        msgs: Vec<Any>,
        memo: Option<&str>,
    ) -> Result<CosmTxResponse, Self::Error>;

    async fn broadcast_tx(
        &self,
        tx: Raw,
    ) -> Result<cosmrs::proto::cosmos::base::abci::v1beta1::TxResponse, Self::Error>;
}
