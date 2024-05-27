use crate::DaemonError;
use serde_json::{from_reader, json, Value};
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
    str::FromStr,
};

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
    let file = File::open(filename)
        .map_err(|err| DaemonError::LoadingFile(filename.to_string(), err.to_string()))?;
    let json: serde_json::Value = from_reader(file)?;
    Ok(json)
}
