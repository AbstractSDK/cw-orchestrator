use cosmrs::{tx::Msg, AccountId, Any};
use cosmwasm_std::Addr;

use crate::CosmTxResponse;

use super::query::QuerySender;

pub trait TxSender: QuerySender {
    /// Returns the `AccountId` of the sender that commits the transaction.
    fn account_id(&self) -> AccountId;

    /// Commit a proto `Any` message to the chain using this sender.
    fn commit_tx_any(
        &self,
        msgs: Vec<Any>,
        memo: Option<&str>,
    ) -> impl std::future::Future<Output = Result<CosmTxResponse, Self::Error>> + Send;

    /// Get the address of the sender.
    fn address(&self) -> Addr {
        Addr::unchecked(self.account_id().to_string())
    }

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
}
