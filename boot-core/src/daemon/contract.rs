use super::error::DaemonError;
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
    /// Only upload the contract if it is not uploaded yet (checksum does not match)
    pub fn upload_if_needed(&mut self) -> Result<Option<TxResponse<Daemon>>, BootError> {
        if self.latest_is_uploaded()? {
            log::info!("{} is already uploaded", self.id);
            Ok(None)
        } else {
            log::info!("{} is not uploaded, uploading...", self.id);
            Some(self.upload()).transpose()
        }
    }

    /// Returns a bool whether the checksum of the wasm file matches the checksum of the previously uploaded code
    pub fn latest_is_uploaded(&self) -> Result<bool, BootError> {
        let latest_uploaded_code_id = self.code_id()?;
        let chain = self.get_chain();

        let query_response =
            chain
                .runtime
                .block_on(super::querier::DaemonQuerier::code_id_hash(
                    chain.sender.channel(),
                    latest_uploaded_code_id,
                ))?;

        let local_hash = self.source.checksum(&self.id)?;

        Ok(local_hash == query_response)
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
            .block_on(super::querier::DaemonQuerier::contract_info(
                chain.sender.channel(),
                self.address()?,
            ))?;

        Ok(latest_uploaded_code_id == info.code_id)
    }
}

impl<ExecT, QueryT> ContractCodeReference<ExecT, QueryT>
where
    ExecT: Clone + fmt::Debug + PartialEq + JsonSchema + DeserializeOwned + 'static,
    QueryT: CustomQuery + DeserializeOwned + 'static,
{
    /// Checks the environment for the wasm dir configuration and returns the path to the wasm file
    pub fn get_wasm_code_path(&self) -> Result<String, DaemonError> {
        let wasm_code_path = self.wasm_code_path.as_ref().ok_or_else(|| {
            DaemonError::StdErr("Wasm file is required to determine hash.".into())
        })?;

        let wasm_code_path = if wasm_code_path.contains(".wasm") {
            wasm_code_path.to_string()
        } else {
            format!(
                "{}/{}.wasm",
                env::var("ARTIFACTS_DIR").expect("ARTIFACTS_DIR is not set"),
                wasm_code_path
            )
        };

        Ok(wasm_code_path)
    }

    /// Calculate the checksum of the wasm file to compare against previous uploads
    pub fn checksum(&self, id: &str) -> Result<String, DaemonError> {
        let wasm_code_path = &self.get_wasm_code_path()?;

        if wasm_code_path.contains("artifacts") {
            // get_wasm_code_path always returns the .wasm on the path
            let folder = Path::new(wasm_code_path);
            let folder = folder.parent().unwrap().to_str().unwrap().to_string();

            // Now get local hash from optimization script
            let checksum_path = format!("{}/checksums.txt", folder);

            let contents =
                fs::read_to_string(checksum_path).expect("Something went wrong reading the file");

            let parsed: Vec<&str> = contents.rsplit(".wasm").collect();
            let name = id.split(':').last().unwrap();
            let containing_line = parsed.iter().find(|line| line.contains(name)).unwrap();

            let local_hash = containing_line
                .trim_start_matches('\n')
                .split_whitespace()
                .next()
                .unwrap();

            log::debug!("checksum: {:?}", local_hash);

            return Ok(local_hash.into());
        }

        // Compute hash
        let wasm_code = Path::new(wasm_code_path);
        let checksum = sha256::try_digest(wasm_code)?;

        Ok(checksum)
    }
}
