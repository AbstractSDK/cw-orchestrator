use super::error::DaemonError;
use crate::{channel::GrpcChannel, json_file::JsonFileState, networks::ChainKind};

use cosmwasm_std::Addr;
use cw_orch_core::{
    env::default_state_folder,
    environment::StateInterface,
    log::{connectivity_target, local_target},
    CwEnvError, CwOrchEnvVars,
};
use ibc_chain_registry::chain::ChainData;
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::{json, Value};
use std::{collections::HashMap, path::Path, sync::Mutex};
use tonic::transport::Channel;

/// Global state for sharing json files between daemons
/// Key: path to state file
/// Value: (DaemonState count that points to that state file, Json file)
pub(crate) static GLOBAL_WRITE_STATE: Lazy<Mutex<HashMap<String, (u64, JsonFileState)>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Stores the chain information and deployment state.
/// Uses a simple JSON file to store the deployment information locally.
#[derive(Debug)]
pub struct DaemonState {
    /// this is passed via env var STATE_FILE
    json_file_path: String,
    /// Deployment identifier
    pub deployment_id: String,
    /// gRPC channel
    pub grpc_channel: Channel,
    /// Information about the chain
    pub chain_data: ChainData,
    /// Flag to set the daemon state readonly and not pollute the env file
    read_only: bool,
}

// On clone increase lock count so we track how many daemon states using the file
impl Clone for DaemonState {
    fn clone(&self) -> Self {
        let new_self = Self {
            json_file_path: self.json_file_path.clone(),
            deployment_id: self.deployment_id.clone(),
            grpc_channel: self.grpc_channel.clone(),
            chain_data: self.chain_data.clone(),
            read_only: self.read_only,
        };

        // Increase DaemonStates count for this file
        if !self.read_only {
            let mut lock = GLOBAL_WRITE_STATE.lock().unwrap();
            let (count, _) = lock.get_mut(&self.json_file_path).unwrap();
            *count += 1;
        }

        new_self
    }
}

impl DaemonState {
    /// Creates a new state from the given chain data and deployment id.
    /// Attempts to connect to any of the provided gRPC endpoints.
    pub async fn new(
        mut chain_data: ChainData,
        deployment_id: String,
        read_only: bool,
    ) -> Result<DaemonState, DaemonError> {
        if chain_data.apis.grpc.is_empty() {
            return Err(DaemonError::GRPCListIsEmpty);
        }

        log::debug!(target: &connectivity_target(), "Found {} gRPC endpoints", chain_data.apis.grpc.len());

        // find working grpc channel
        let grpc_channel =
            GrpcChannel::connect(&chain_data.apis.grpc, chain_data.chain_id.as_str()).await?;

        // If the path is relative, we dis-ambiguate it and take the root at $HOME/$CW_ORCH_STATE_FOLDER
        let mut json_file_path = Self::state_file_path()?;

        log::debug!(target: &local_target(), "Using state file : {}", json_file_path);

        // if the network we are connecting is a local kind, add it to the fn
        if chain_data.network_type == ChainKind::Local.to_string() {
            let name = Path::new(&json_file_path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();
            let folder = Path::new(&json_file_path)
                .parent()
                .unwrap()
                .to_str()
                .unwrap();

            json_file_path = format!("{folder}/{name}_local.json");
        }

        // Try to get the standard fee token (probably shortest denom)
        let shortest_denom_token = chain_data.fees.fee_tokens.iter().fold(
            chain_data.fees.fee_tokens[0].clone(),
            |acc, item| {
                if item.denom.len() < acc.denom.len() {
                    item.clone()
                } else {
                    acc
                }
            },
        );
        // set a single fee token
        chain_data.fees.fee_tokens = vec![shortest_denom_token];

        // build daemon state
        let state = DaemonState {
            json_file_path: json_file_path.clone(),
            deployment_id,
            grpc_channel,
            chain_data,
            read_only,
        };

        if !read_only {
            log::info!(
                target: &local_target(),
                "Writing daemon state JSON file: {:#?}",
                state.json_file_path
            );

            let mut lock = GLOBAL_WRITE_STATE.lock().unwrap();
            // Lock file if first time write
            let file_state = match lock.entry(json_file_path.clone()) {
                // Increase count if already locked this file
                std::collections::hash_map::Entry::Occupied(o) => {
                    let (count, lock) = o.into_mut();
                    *count += 1;
                    lock
                }
                // Insert as 1 count of DaemonStates pointing to this file if it's first open
                std::collections::hash_map::Entry::Vacant(v) => {
                    let (_, lock) = v.insert((1, JsonFileState::new(&json_file_path)));
                    lock
                }
            };
            file_state.prepare(
                state.chain_data.chain_id.as_str(),
                &state.chain_data.chain_name,
                &state.deployment_id,
            );
        }

        // finish
        Ok(state)
    }

    /// Returns the path of the file where the state of `cw-orchestrator` is stored.
    pub fn state_file_path() -> Result<String, DaemonError> {
        // check if STATE_FILE en var is configured, default to state.json
        let env_file_path = CwOrchEnvVars::load()?.state_file;
        let state_file_path = if env_file_path.is_relative() {
            // If it's relative, we check if it start with "."
            let first_path_component = env_file_path
                .components()
                .map(|comp| comp.as_os_str().to_owned().into_string().unwrap())
                .next();
            if first_path_component == Some(".".to_string()) {
                let current_dir = std::env::current_dir()?;
                let actual_relative_path = env_file_path.strip_prefix("./")?;
                current_dir.join(actual_relative_path)
            } else if first_path_component == Some("..".to_string()) {
                let current_dir = std::env::current_dir()?;
                current_dir.join(env_file_path)
            } else {
                let state_folder = default_state_folder()?;

                // We need to create the default state folder if it doesn't exist
                std::fs::create_dir_all(state_folder.clone())?;

                state_folder.join(env_file_path)
            }
        } else {
            env_file_path
        }
        .into_os_string()
        .into_string()
        .unwrap();

        Ok(state_file_path)
    }

    /// Get the chain state as json
    fn chain_state(&self) -> Result<serde_json::Value, DaemonError> {
        // Check if already open in write mode {
        let lock = GLOBAL_WRITE_STATE.lock().unwrap();
        if let Some((_, j)) = lock.get(&self.json_file_path) {
            Ok(j.get(
                &self.chain_data.chain_name,
                self.chain_data.chain_id.as_str(),
            )
            .clone())
        } else {
            // drop guard if not found, since reading may take a while
            drop(lock);
            // Or just read it from a file
            crate::json_file::read(&self.json_file_path)
                .map(|j| j[self.chain_data.chain_id.as_str()][&self.chain_data.chain_name].clone())
        }
    }

    /// Retrieve a stateful value using the chainId and networkId
    pub fn get(&self, key: &str) -> Result<Value, DaemonError> {
        let json = self.chain_state()?;
        Ok(json[key].clone())
    }

    /// Set a stateful value using the chainId and networkId
    pub fn set<T: Serialize>(
        &self,
        key: &str,
        contract_id: &str,
        value: T,
    ) -> Result<(), DaemonError> {
        if self.read_only {
            return Err(DaemonError::StateReadOnly);
        }

        let mut lock = GLOBAL_WRITE_STATE.lock().unwrap();

        let (_, file_state) = lock.get_mut(&self.json_file_path).unwrap();
        let val = file_state.get_mut(
            self.chain_data.chain_id.as_str(),
            &self.chain_data.chain_name,
        );
        val[key][contract_id] = json!(value);

        Ok(())
    }

    /// Forcefully write current json to a file
    pub fn force_write(&self) -> Result<(), DaemonError> {
        let mut lock = GLOBAL_WRITE_STATE.lock().unwrap();
        let (_, file_state) = lock.get_mut(&self.json_file_path).unwrap();
        file_state.force_write();
        Ok(())
    }
}

// Manual drop implementation to write state when no daemon states uses the file
impl Drop for DaemonState {
    fn drop(&mut self) {
        let mut lock = GLOBAL_WRITE_STATE.lock().unwrap();

        // Decrease open count
        let (count, _) = lock.get_mut(&self.json_file_path).unwrap();
        *count -= 1;

        // If we get to zero count - write to a file
        if *count == 0 {
            lock.remove(&self.json_file_path);
        }
    }
}

impl StateInterface for DaemonState {
    /// Read address for contract in deployment id from state file
    fn get_address(&self, contract_id: &str) -> Result<Addr, CwEnvError> {
        let value = self
            .get(&self.deployment_id)?
            .get(contract_id)
            .ok_or_else(|| CwEnvError::AddrNotInStore(contract_id.to_owned()))?
            .clone();
        Ok(Addr::unchecked(value.as_str().unwrap()))
    }

    /// Set address for contract in deployment id in state file
    fn set_address(&mut self, contract_id: &str, address: &Addr) {
        self.set(&self.deployment_id, contract_id, address.as_str())
            .unwrap();
    }

    /// Get the locally-saved version of the contract's version on this network
    fn get_code_id(&self, contract_id: &str) -> Result<u64, CwEnvError> {
        let value = self
            .get("code_ids")?
            .get(contract_id)
            .ok_or_else(|| CwEnvError::CodeIdNotInStore(contract_id.to_owned()))?
            .clone();
        Ok(value.as_u64().unwrap())
    }

    /// Set the locally-saved version of the contract's latest version on this network
    fn set_code_id(&mut self, contract_id: &str, code_id: u64) {
        self.set("code_ids", contract_id, code_id).unwrap();
    }

    /// Get all addresses for deployment id from state file
    fn get_all_addresses(&self) -> Result<HashMap<String, Addr>, CwEnvError> {
        let mut store = HashMap::new();
        let addresses = self.get(&self.deployment_id)?;
        let value = addresses.as_object().cloned().unwrap_or_default();
        for (id, addr) in value {
            store.insert(id, Addr::unchecked(addr.as_str().unwrap()));
        }
        Ok(store)
    }

    fn get_all_code_ids(&self) -> Result<HashMap<String, u64>, CwEnvError> {
        let mut store = HashMap::new();
        let code_ids = self.get("code_ids")?;
        let value = code_ids.as_object().cloned().unwrap_or_default();
        for (id, code_id) in value {
            store.insert(id, code_id.as_u64().unwrap());
        }
        Ok(store)
    }
}

#[cfg(test)]
pub mod test {
    use std::env;

    use cw_orch_core::env::STATE_FILE_ENV_NAME;

    use crate::DaemonState;

    #[test]
    fn test_env_variable_state_path() -> anyhow::Result<()> {
        let absolute_path = "/usr/var/file.json";
        let relative_path = "folder/file.json";
        let dotted_relative_path = format!("./{}", relative_path);
        let parent_and_relative_path = format!("../{}", relative_path);

        std::env::set_var(STATE_FILE_ENV_NAME, absolute_path);
        let absolute_state_path = DaemonState::state_file_path()?;
        assert_eq!(absolute_path.to_string(), absolute_state_path);

        std::env::set_var(STATE_FILE_ENV_NAME, dotted_relative_path);
        let relative_state_path = DaemonState::state_file_path()?;
        assert_eq!(
            env::current_dir()?
                .join(relative_path)
                .into_os_string()
                .into_string()
                .unwrap(),
            relative_state_path
        );

        std::env::set_var(STATE_FILE_ENV_NAME, relative_path);
        let relative_state_path = DaemonState::state_file_path()?;
        assert_eq!(
            dirs::home_dir()
                .unwrap()
                .join(".cw-orchestrator")
                .join(relative_path)
                .into_os_string()
                .into_string()
                .unwrap(),
            relative_state_path
        );

        std::env::set_var(STATE_FILE_ENV_NAME, parent_and_relative_path);
        let parent_and_relative_state_path = DaemonState::state_file_path()?;
        assert_eq!(
            env::current_dir()?
                .join("../")
                .join(relative_path)
                .into_os_string()
                .into_string()
                .unwrap(),
            parent_and_relative_state_path
        );

        Ok(())
    }
}
