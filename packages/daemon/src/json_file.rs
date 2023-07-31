use serde_json::{from_reader, json, Value};
use std::fs::{File, OpenOptions};

pub fn write(filename: &String, chain_id: &String, network_id: &String, deploy_id: &String) {
    // open file pointer set read/write permissions to true
    // create it if it does not exists
    // dont truncate it
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(filename)
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

pub fn read(filename: &String) -> Value {
    let file =
        File::open(filename).unwrap_or_else(|_| panic!("File should be present at {}", filename));
    let json: serde_json::Value = from_reader(file).unwrap();
    json
}
