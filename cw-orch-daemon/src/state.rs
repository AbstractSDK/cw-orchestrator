use super::error::DaemonError;
use crate::env::{default_state_folder, DaemonEnvVars};
use crate::{json_lock::JsonLockedState, networks::ChainKind};

use cosmwasm_std::Addr;
use cw_orch_core::environment::ChainInfoOwned;
use cw_orch_core::{environment::StateInterface, log::local_target, CwEnvError};
use once_cell::sync::Lazy;
use serde::Serialize;
use serde_json::{json, Value};
use std::ffi::OsString;
use std::sync::Arc;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::Mutex,
};

/// Global state to track which files are already open by other daemons from other threads
/// This is necessary because File lock will allow same process to lock file how many times as process wants
pub(crate) static LOCKED_FILES: Lazy<Mutex<HashSet<String>>> =
    Lazy::new(|| Mutex::new(HashSet::new()));

/// Stores the chain information and deployment state.
/// Uses a simple JSON file to store the deployment information locally.
#[derive(Debug, Clone)]
pub struct DaemonState {
    pub json_state: DaemonStateFile,
    /// Deployment identifier
    pub deployment_id: String,
    /// Information about the chain
    pub chain_data: Arc<ChainInfoOwned>,
    /// Whether to write on every change of the state
    pub write_on_change: bool,
}

impl Drop for DaemonState {
    fn drop(&mut self) {
        if let DaemonStateFile::FullAccess { json_file_state } = &self.json_state {
            let json_lock = json_file_state.lock().unwrap();
            let mut locked_files = LOCKED_FILES.lock().unwrap();
            locked_files.remove(json_lock.path());
        }
    }
}

#[derive(Debug, Clone)]
pub enum DaemonStateFile {
    ReadOnly {
        path: String,
    },
    FullAccess {
        json_file_state: Arc<Mutex<JsonLockedState>>,
    },
}

impl DaemonState {
    /// Creates a new state from the given chain data and deployment id.
    /// Attempts to connect to any of the provided gRPC endpoints.
    pub fn new(
        mut json_file_path: String,
        chain_data: &Arc<ChainInfoOwned>,
        deployment_id: String,
        read_only: bool,
        write_on_change: bool,
    ) -> Result<DaemonState, DaemonError> {
        let chain_id = &chain_data.chain_id;
        let chain_name = &chain_data.network_info.chain_name;

        log::debug!(target: &local_target(), "Using state file : {}", json_file_path);

        // if the network we are connecting is a local kind, add it to the fn
        if chain_data.kind == ChainKind::Local {
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

        let json_state = if read_only {
            DaemonStateFile::ReadOnly {
                path: json_file_path,
            }
        } else {
            log::info!(
                target: &local_target(),
                "Writing daemon state JSON file: {json_file_path:#?}",
            );

            let mut lock = LOCKED_FILES.lock().unwrap();
            if lock.contains(&json_file_path) {
                return Err(DaemonError::StateAlreadyLocked(json_file_path));
            }
            let mut json_file_state = JsonLockedState::new(&json_file_path);
            // Insert file to a locked files list and drop global mutex lock asap
            lock.insert(json_file_path);
            drop(lock);

            json_file_state.prepare(chain_id, chain_name, &deployment_id);
            if write_on_change {
                json_file_state.force_write();
            }
            DaemonStateFile::FullAccess {
                json_file_state: Arc::new(Mutex::new(json_file_state)),
            }
        };

        Ok(DaemonState {
            json_state,
            deployment_id,
            chain_data: chain_data.clone(),
            write_on_change,
        })
    }

    /// Returns the path of the file where the state of `cw-orchestrator` is stored.
    pub fn state_file_path() -> Result<String, DaemonError> {
        // check if STATE_FILE en var is configured, default to state.json
        let env_file_path = DaemonEnvVars::state_file();
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

    /// Retrieve a stateful value using the chainId and networkId
    pub fn get(&self, key: &str) -> Result<Value, DaemonError> {
        let json = match &self.json_state {
            DaemonStateFile::ReadOnly { path } => {
                let j = crate::json_lock::read(path)?;

                j[&self.chain_data.network_info.chain_name][&self.chain_data.chain_id].clone()
            }
            DaemonStateFile::FullAccess { json_file_state } => json_file_state
                .lock()
                .unwrap()
                .get(
                    &self.chain_data.network_info.chain_name,
                    &self.chain_data.chain_id,
                )
                .clone(),
        };
        Ok(json[key].clone())
    }

    /// Set a stateful value using the chainId and networkId
    pub fn set<T: Serialize>(
        &mut self,
        key: &str,
        contract_id: &str,
        value: T,
    ) -> Result<(), DaemonError> {
        let json_file_state = match &mut self.json_state {
            DaemonStateFile::ReadOnly { path } => {
                return Err(DaemonError::StateReadOnly(path.clone()))
            }
            DaemonStateFile::FullAccess { json_file_state } => json_file_state,
        };

        let mut json_file_lock = json_file_state.lock().unwrap();
        let val = json_file_lock.get_mut(
            &self.chain_data.network_info.chain_name,
            &self.chain_data.chain_id,
        );
        val[key][contract_id] = json!(value);

        if self.write_on_change {
            json_file_lock.force_write();
        }

        Ok(())
    }

    /// Remove a stateful value using the chainId and networkId
    pub fn remove(&mut self, key: &str, contract_id: &str) -> Result<(), DaemonError> {
        let json_file_state = match &mut self.json_state {
            DaemonStateFile::ReadOnly { path } => {
                return Err(DaemonError::StateReadOnly(path.clone()))
            }
            DaemonStateFile::FullAccess { json_file_state } => json_file_state,
        };

        let mut json_file_lock = json_file_state.lock().unwrap();
        let val = json_file_lock.get_mut(
            &self.chain_data.network_info.chain_name,
            &self.chain_data.chain_id,
        );
        val[key][contract_id] = Value::Null;

        if self.write_on_change {
            json_file_lock.force_write();
        }

        Ok(())
    }

    /// Forcefully write current json to a file
    pub fn force_write(&mut self) -> Result<(), DaemonError> {
        let json_file_state = match &mut self.json_state {
            DaemonStateFile::ReadOnly { path } => {
                return Err(DaemonError::StateReadOnly(path.clone()))
            }
            DaemonStateFile::FullAccess { json_file_state } => json_file_state,
        };
        json_file_state.lock().unwrap().force_write();
        Ok(())
    }

    /// Flushes all the state related to the current chain
    /// Only works on Local networks
    pub fn flush(&mut self) -> Result<(), DaemonError> {
        if self.chain_data.kind != ChainKind::Local {
            panic!("Can only flush local chain state");
        }
        let json_file_state = match &mut self.json_state {
            DaemonStateFile::ReadOnly { path } => {
                return Err(DaemonError::StateReadOnly(path.clone()))
            }
            DaemonStateFile::FullAccess { json_file_state } => json_file_state,
        };

        let mut json_file_lock = json_file_state.lock().unwrap();
        let json = json_file_lock.get_mut(
            &self.chain_data.network_info.chain_name,
            &self.chain_data.chain_id,
        );

        *json = json!({});

        if self.write_on_change {
            json_file_lock.force_write();
        }
        Ok(())
    }
}

impl StateInterface for DaemonState {
    /// Read address for contract in deployment id from state file
    fn get_address(&self, contract_id: &str) -> Result<Addr, CwEnvError> {
        let value = self
            .get(&self.deployment_id)
            .ok()
            .and_then(|v| v.get(contract_id).cloned())
            .ok_or_else(|| CwEnvError::AddrNotInStore(contract_id.to_owned()))?
            .clone();
        Ok(Addr::unchecked(value.as_str().unwrap()))
    }

    /// Set address for contract in deployment id in state file
    fn set_address(&mut self, contract_id: &str, address: &Addr) {
        let deployment_id = self.deployment_id.clone();
        self.set(&deployment_id, contract_id, address.as_str())
            .unwrap();
    }

    fn remove_address(&mut self, contract_id: &str) {
        let deployment_id = self.deployment_id.clone();
        self.remove(&deployment_id, contract_id).unwrap();
    }

    /// Get the locally-saved version of the contract's version on this network
    fn get_code_id(&self, contract_id: &str) -> Result<u64, CwEnvError> {
        let value = self
            .get("code_ids")
            .ok()
            .and_then(|v| v.get(contract_id).cloned())
            .ok_or_else(|| CwEnvError::CodeIdNotInStore(contract_id.to_owned()))?
            .clone();
        Ok(value.as_u64().unwrap())
    }

    /// Set the locally-saved version of the contract's latest version on this network
    fn set_code_id(&mut self, contract_id: &str, code_id: u64) {
        self.set("code_ids", contract_id, code_id).unwrap();
    }
    fn remove_code_id(&mut self, contract_id: &str) {
        self.remove("code_ids", contract_id).unwrap();
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

// copied from `tempfile::util::tmpname` implementation
pub(crate) fn gen_temp_file_path() -> std::path::PathBuf {
    let mut env_dir = std::env::temp_dir();
    let rand_len = 8;
    let mut file_name = OsString::with_capacity(rand_len);
    let mut char_buf = [0u8; 4];
    for c in std::iter::repeat_with(fastrand::alphanumeric).take(rand_len) {
        file_name.push(c.encode_utf8(&mut char_buf));
    }
    env_dir.push(file_name);
    env_dir
}

#[cfg(test)]
pub mod test {
    use std::env;

    use crate::{env::STATE_FILE_ENV_NAME, DaemonState};

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

        std::env::remove_var(STATE_FILE_ENV_NAME);
        Ok(())
    }
}
