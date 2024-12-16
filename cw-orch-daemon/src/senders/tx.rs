use cosmrs::{
    tx::{Msg, Raw},
    AccountId, Any,
};
use cosmwasm_std::Addr;

use crate::{cosmos_modules, CosmTxResponse, DaemonError};

use super::query::QuerySender;

pub trait TxSender: QuerySender + Sync {
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

    /// Actual sender of the messages.
    ///
    /// For example, this can be different when using authz capabilites
    fn msg_sender(&self) -> Result<AccountId, Self::Error> {
        Ok(self.account_id())
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

    /// Transaction broadcasting for Tendermint Transactions
    fn broadcast_tx(
        &self,
        tx: Raw,
    ) -> impl std::future::Future<
        Output = Result<cosmrs::proto::cosmos::base::abci::v1beta1::TxResponse, DaemonError>,
    > + Send {
        async move {
            let mut client = cosmos_modules::tx::service_client::ServiceClient::new(self.channel());
            let commit = client
                .broadcast_tx(cosmos_modules::tx::BroadcastTxRequest {
                    tx_bytes: tx.to_bytes()?,
                    mode: cosmos_modules::tx::BroadcastMode::Sync.into(),
                })
                .await?;

            let commit = commit.into_inner().tx_response.unwrap();
            Ok(commit)
        }
    }

    // Send funds using the bank module
    fn bank_send(
        &self,
        _receiver: &Addr,
        _amount: &[cosmwasm_std::Coin],
    ) -> impl std::future::Future<Output = Result<CosmTxResponse, Self::Error>> + Send {
        async { unimplemented!() }
    }
}
