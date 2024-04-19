use crate::DaemonError;
use file_lock::{FileLock, FileOptions};
use serde_json::{from_reader, json, Value};
use std::{
    fs::{File, OpenOptions},
    io::Seek,
};

pub struct JsonFileState {
    lock: FileLock,
    json: Value,
}

impl JsonFileState {
    /// Lock a new file
    pub fn new(filename: &str) -> Self {
        // open file pointer set read/write permissions to true
        // create it if it does not exists
        // dont truncate it

        let options = FileOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false);

        let lock: FileLock = FileLock::lock(filename, true, options).unwrap();

        // return empty json object if file is empty
        // return file content if not
        let json: Value = if lock.file.metadata().unwrap().len().eq(&0) {
            json!({})
        } else {
            from_reader(&lock.file).unwrap()
        };

        JsonFileState { lock, json }
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
    // pub fn get(&self, chain_id: &str, network_id: &str) -> &Value {
    //     self.json[network_id].get(chain_id).unwrap()
    // }

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
impl Drop for JsonFileState {
    fn drop(&mut self) {
        self.force_write()
    }
}

pub fn write(filename: &String, chain_id: &String, network_id: &String, deploy_id: &String) {
    // open file pointer set read/write permissions to true
    // create it if it does not exists
    // dont truncate it
    // Create the directory if they do not exist
    let file_buf = PathBuf::from_str(filename).unwrap();
    if let Some(parent) = file_buf.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(filename.clone())
        .unwrap();

    // return empty json object if file is empty
    // return file content if not
    let mut json: Value = if file.metadata().unwrap().len().eq(&0) {
        json!({})
    } else {
        from_reader(file).unwrap()
    };

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

    // write JSON data
    // use File::create so we dont append data to the file
    // but rather write all (because we have read the data before)
    serde_json::to_writer_pretty(File::create(filename).unwrap(), &json).unwrap();
}

pub fn read(filename: &String) -> Result<Value, DaemonError> {
    let file =
        File::open(filename).unwrap_or_else(|_| panic!("File should be present at {}", filename));
    let json: serde_json::Value = from_reader(file)?;
    Ok(json)
}
