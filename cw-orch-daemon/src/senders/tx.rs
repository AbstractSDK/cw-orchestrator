use cosmrs::{tx::Msg, AccountId, Any};
use cosmwasm_std::Addr;

use crate::CosmTxResponse;

use super::query::QuerySender;

pub trait TxSender: QuerySender {
    /// Get the address of the sender.
    fn address(&self) -> Result<Addr, Self::Error>;

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
