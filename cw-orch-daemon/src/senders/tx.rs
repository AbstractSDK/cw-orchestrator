use std::io::Write;

use cosmrs::{tx::Msg, AccountId, Any};
use cosmwasm_std::Addr;
use cw_orch_core::contract::WasmPath;
use flate2::{write, Compression};

use crate::{CosmTxResponse, DaemonError};

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

    /// Uploads the `WasmPath` path specifier on chain.
    /// The resulting code_id can be extracted from the Transaction result using [cw_orch_core::environment::IndexResponse::uploaded_code_id] and returns the resulting code_id
    fn upload_wasm(
        &self,
        wasm_path: WasmPath,
    ) -> impl std::future::Future<Output = Result<CosmTxResponse, DaemonError>> + Send {
        async move {
            let file_contents = std::fs::read(wasm_path.path())?;
            let mut e = write::GzEncoder::new(Vec::new(), Compression::default());
            e.write_all(&file_contents)?;
            let wasm_byte_code = e.finish()?;
            let store_msg = cosmrs::cosmwasm::MsgStoreCode {
                sender: self.account_id(),
                wasm_byte_code,
                instantiate_permission: None,
            };

            self.commit_tx(vec![store_msg], None)
                .await
                .map_err(Into::into)
        }
    }
}
