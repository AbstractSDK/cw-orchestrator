use crate::DaemonError;
use file_lock::{FileLock, FileOptions};
use fs4::FileExt;
use serde_json::{from_reader, json, Value};
use std::{
    fs::File,
    io::{Seek, SeekFrom},
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

    let mut filelock = get_write_lock(filename);

    // return empty json object if file is empty
    // return file content if not
    let mut json: Value = if filelock.file.metadata().unwrap().len().eq(&0) {
        json!({})
    } else {
        from_reader(&filelock.file).unwrap()
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

    write_to_file(&mut filelock.file, json);

    filelock.file.unlock().unwrap()
}

pub fn write_to_file(lock: &mut File, json: Value) {
    // write JSON data
    // use set_len(0), so we don't happend data to the file
    // but rather write all (because we have read the data before)
    lock.set_len(0).unwrap();
    lock.seek(SeekFrom::Start(0)).unwrap(); // Seek to the beginning of the file
    serde_json::to_writer_pretty(lock, &json).unwrap();
}

pub fn read(filename: &String) -> Result<Value, DaemonError> {
    let filelock = get_read_lock(filename);

    let value = read_file(&filelock.file)?;
    filelock.file.unlock().unwrap();
    Ok(value)
}

pub fn read_file(file: &File) -> Result<Value, DaemonError> {
    let json: serde_json::Value = from_reader(file)?;
    Ok(json)
}

pub fn get_read_lock(filename: &String) -> FileLock {
    let options = FileOptions::new().read(true);

    // We lock for other processes
    let filelock = match FileLock::lock(filename, true, options) {
        Ok(lock) => lock,
        Err(err) => panic!("Error getting write lock: {}", err),
    };
    filelock.file.lock_exclusive().unwrap();
    filelock
}

pub fn get_write_lock(filename: &String) -> FileLock {
    let options = FileOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false);

    // We lock for other processes
    let filelock = match FileLock::lock(filename, true, options) {
        Ok(lock) => lock,
        Err(err) => panic!("Error getting write lock: {}", err),
    };

    // We lock for the current process
    filelock.file.lock_exclusive().unwrap();

    filelock
}
