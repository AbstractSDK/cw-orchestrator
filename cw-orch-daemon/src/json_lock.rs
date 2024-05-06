use crate::DaemonError;
use file_lock::{FileLock, FileOptions};
use serde_json::{from_reader, json, Value};
use std::{fs::File, io::Seek};

/// State file reader and writer
/// Mainly used by [`crate::Daemon`] and [`crate::DaemonAsync`], but could also be used for tests or custom edits of the state
#[derive(Debug)]
pub struct JsonLockedState {
    lock: FileLock,
    json: Value,
}

impl JsonLockedState {
    /// Lock a state files
    /// Other process won't be able to lock it
    pub fn new(filename: &str) -> Self {
        // open file pointer set read/write permissions to true
        // create it if it does not exists
        // dont truncate it

        let options = FileOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false);

        // Lock file, non blocking so it errors in case someone else already holding lock of it
        let lock: FileLock = FileLock::lock(filename, false, options)
            .unwrap_or_else(|_| panic!("Was not able to receive {filename} state lock"));

        // return empty json object if file is empty
        // return file content if not
        let json: Value = if lock.file.metadata().unwrap().len().eq(&0) {
            json!({})
        } else {
            from_reader(&lock.file).unwrap()
        };

        JsonLockedState { lock, json }
    }

    /// Prepare json for further writes
    pub fn prepare(&mut self, chain_id: &str, network_id: &str, deploy_id: &str) {
        let json = &mut self.json;
        // check and add network_id path if it's missing
        if json.get(network_id).is_none() {
            json[network_id] = json!({});
        }

        // add deployment_id to chain_id path
        if json[network_id].get(chain_id).is_none() {
            json[network_id][chain_id] = json!({
                deploy_id: {},
                "code_ids": {}
            });
        }
    }

    pub fn state(&self) -> Value {
        self.json.clone()
    }

    /// Get a value for read
    pub fn get(&self, chain_id: &str, network_id: &str) -> &Value {
        &self.json[network_id][chain_id]
    }

    /// Give a value to write
    pub fn get_mut(&mut self, chain_id: &str, network_id: &str) -> &mut Value {
        self.json[network_id].get_mut(chain_id).unwrap()
    }

    /// Force write to a file
    pub fn force_write(&mut self) {
        self.lock.file.rewind().unwrap();
        serde_json::to_writer_pretty(&self.lock.file, &self.json).unwrap();
    }
}

// Write json when dropping
impl Drop for JsonLockedState {
    fn drop(&mut self) {
        self.force_write()
    }
}

pub fn read(filename: &String) -> Result<Value, DaemonError> {
    let file =
        File::open(filename).unwrap_or_else(|_| panic!("File should be present at {}", filename));
    let json: serde_json::Value = from_reader(file)?;
    Ok(json)
}
