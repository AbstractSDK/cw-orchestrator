use super::{error::DaemonError, queriers::CosmWasm};
use crate::{contract::ContractCodeReference, BootError, Contract, Daemon, TxResponse};
use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    env,
    fmt::{self, Debug},
    fs,
    path::Path,
};

impl Contract<Daemon> {
    /// Only upload the contract if it is not uploaded yet
    pub fn upload_if_needed(&mut self) -> Result<Option<TxResponse<Daemon>>, BootError> {
        let upload = |se: &mut Contract<Daemon>| {
            log::info!("{} is not uploaded, uploading...", se.id);
            match se.upload() {
                Ok(ok) => Ok(Some(ok)),
                Err(err) => Err(err),
            }
        };

        match self.latest_is_uploaded() {
            Ok(_) => {
                log::info!("{} is already uploaded", self.id);
                Ok(None)
            }
            Err(err) => match err {
                BootError::CodeIdNotInStore(_) => upload(self),
                BootError::DaemonError(err) => match err.to_string().as_str() {
                    "not found" => upload(self),
                    _ => Err(BootError::DaemonError(err)),
                },
                _ => Err(err),
            },
        }
    }

    /// Returns a bool whether the checksum of the wasm file matches the checksum of the previously uploaded code
    pub fn latest_is_uploaded(&self) -> Result<bool, BootError> {
        let latest_uploaded_code_id = self.code_id()?;
        let chain = self.get_chain();
        let on_chain_hash = chain.runtime.block_on(
            chain
                .query::<CosmWasm>()
                .code_id_hash(latest_uploaded_code_id),
        )?;
        let local_hash = self.source.checksum(&self.id)?;

        Ok(local_hash == on_chain_hash)
    }

    /// Only migrate the contract if it is not on the latest code-id yet
    pub fn migrate_if_needed(
        &mut self,
        migrate_msg: &(impl Serialize + Debug),
    ) -> Result<Option<TxResponse<Daemon>>, BootError> {
        if self.is_running_latest()? {
            log::info!("{} is already running the latest code", self.id);
            Ok(None)
        } else {
            Some(self.migrate(migrate_msg, self.code_id()?)).transpose()
        }
    }

    /// Returns a bool whether the contract is running the latest uploaded code for it
    pub fn is_running_latest(&self) -> Result<bool, BootError> {
        let latest_uploaded_code_id = self.code_id()?;
        let chain = self.get_chain();
        let info = chain
            .runtime
            .block_on(chain.query::<CosmWasm>().contract_info(self.address()?))?;
        Ok(latest_uploaded_code_id == info.code_id)
    }
}

impl<ExecT, QueryT> ContractCodeReference<ExecT, QueryT>
where
    ExecT: Clone + fmt::Debug + PartialEq + JsonSchema + DeserializeOwned + 'static,
    QueryT: CustomQuery + DeserializeOwned + 'static,
{
    /// Checks the environment for the wasm dir configuration and returns the path to the wasm file
    /// If the path does not contain a .wasm file, we assume it is in the artifacts dir where it's searched by name.
    /// If the path contains a .wasm file, we assume it is the path to the wasm file.
    pub fn get_wasm_code_path(&self) -> Result<String, DaemonError> {
        let wasm_code_path = self
            .wasm_code_path
            .as_ref()
            .ok_or_else(|| DaemonError::MissingWasmPath)?;

        let wasm_code_path = if wasm_code_path.contains(".wasm") {
            wasm_code_path.to_string()
        } else {
            // If the path does not contain a .wasm file, we assume it is in the artifacts dir
            // find the wasm file with the name of the contract
            let artifacts_dir = env::var("ARTIFACTS_DIR").expect("ARTIFACTS_DIR is not set");
            let artifacts_dir = Path::new(&artifacts_dir);
            find_wasm_with_name_in_artifacts(artifacts_dir, wasm_code_path).ok_or_else(|| {
                DaemonError::StdErr(format!(
                    "Could not find wasm file with name {} in artifacts dir",
                    wasm_code_path
                ))
            })?
        };

        Ok(wasm_code_path)
    }

    /// Calculate the checksum of the wasm file to compare against previous uploads
    pub fn checksum(&self, _id: &str) -> Result<String, DaemonError> {
        let wasm_code_path = &self.get_wasm_code_path()?;

        // Compute hash
        let wasm_code = Path::new(wasm_code_path);
        let checksum = sha256::try_digest(wasm_code)?;

        Ok(checksum)
    }
}

/// Get the wasm file with the name of the contract
fn find_wasm_with_name_in_artifacts(dir_path: &Path, target_string: &str) -> Option<String> {
    fs::read_dir(dir_path)
        .ok()?
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();
            if path.is_file()
                && path.extension().unwrap_or_default() == "wasm"
                && file_name.contains(target_string)
            {
                Some(file_name.into_owned())
            } else {
                None
            }
        })
        .next()
}
