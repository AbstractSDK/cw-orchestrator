use cw_orch_environment::{
    contract::interface_traits::{CwOrchMigrate, CwOrchUpload},
    environment::TxResponse,
};

use crate::{queriers::CosmWasm, Daemon, DaemonError};

/// Helper methods for conditional uploading of a contract.
pub trait ConditionalUpload: CwOrchUpload<Daemon> {
    /// Only upload the contract if it is not uploaded yet (checksum does not match)
    fn upload_if_needed(&self) -> Result<Option<TxResponse<Daemon>>, DaemonError> {
        if self.latest_is_uploaded()? {
            Ok(None)
        } else {
            Some(self.upload()).transpose().map_err(Into::into)
        }
    }

    /// Returns whether the checksum of the WASM file matches the checksum of the latest uploaded code for this contract.
    fn latest_is_uploaded(&self) -> Result<bool, DaemonError> {
        let Some(latest_uploaded_code_id) = self.code_id().ok() else {
            return Ok(false);
        };

        let chain = self.get_chain();
        let on_chain_hash = chain.rt_handle.block_on(
            chain
                .query_client::<CosmWasm>()
                .code_id_hash(latest_uploaded_code_id),
        )?;
        let local_hash = self.wasm().checksum()?;

        Ok(local_hash == on_chain_hash)
    }

    /// Returns whether the contract is running the latest uploaded code for it
    fn is_running_latest(&self) -> Result<bool, DaemonError> {
        let Some(latest_uploaded_code_id) = self.code_id().ok() else {
            return Ok(false);
        };
        let chain = self.get_chain();
        let info = chain.rt_handle.block_on(
            chain
                .query_client::<CosmWasm>()
                .contract_info(self.address()?),
        )?;
        Ok(latest_uploaded_code_id == info.code_id)
    }
}

impl<T> ConditionalUpload for T where T: CwOrchUpload<Daemon> {}

/// Helper methods for conditional migration of a contract.
pub trait ConditionalMigrate: CwOrchMigrate<Daemon> + ConditionalUpload {
    /// Only migrate the contract if it is not on the latest code-id yet
    fn migrate_if_needed(
        &self,
        migrate_msg: &Self::MigrateMsg,
    ) -> Result<Option<TxResponse<Daemon>>, DaemonError> {
        if self.is_running_latest()? {
            log::info!("{} is already running the latest code", self.id());
            Ok(None)
        } else {
            Some(self.migrate(migrate_msg, self.code_id()?))
                .transpose()
                .map_err(Into::into)
        }
    }
    /// Uploads the contract if the local contract hash is different from the latest on-chain code hash.
    /// Proceeds to migrates the contract if the contract is not running the latest code.
    fn upload_and_migrate_if_needed(
        &self,
        migrate_msg: &Self::MigrateMsg,
    ) -> Result<Option<Vec<TxResponse<Daemon>>>, DaemonError> {
        let mut txs = Vec::with_capacity(2);

        if let Some(tx) = self.upload_if_needed()? {
            txs.push(tx);
        };

        if let Some(tx) = self.migrate_if_needed(migrate_msg)? {
            txs.push(tx);
        };

        if txs.is_empty() {
            Ok(None)
        } else {
            Ok(Some(txs))
        }
    }
}

impl<T> ConditionalMigrate for T where T: CwOrchMigrate<Daemon> + CwOrchUpload<Daemon> {}
