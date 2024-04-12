use crate::common::B64;

use base64::Engine;
use keyring::Entry;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fs::{File, OpenOptions},
    path::PathBuf,
};

use super::cli_subdir::cli_path;

// Should be possible to remove this file in a feature.
// Tracking issue: https://github.com/hwchen/keyring-rs/issues/144
const ENTRIES_LIST_FILE: &str = "keys_entries.json";

#[derive(Default, Serialize, Deserialize)]
pub struct EntriesSet {
    pub entries: BTreeSet<String>,
}

fn entries_list_path() -> color_eyre::Result<PathBuf> {
    Ok(cli_path()?.join(ENTRIES_LIST_FILE))
}

pub fn read_entries() -> color_eyre::Result<EntriesSet> {
    let entries_list_file = entries_list_path()?;

    let maybe_file = OpenOptions::new()
        .read(true)
        .open(entries_list_file.as_path());
    // In case no file return empty
    let Ok(file) = maybe_file else {
        return Ok(Default::default());
    };
    let entries_set: EntriesSet = serde_json::from_reader(file)?;
    Ok(entries_set)
}

pub fn save_entry_if_required(entry: &str) -> color_eyre::Result<()> {
    let entries_list_file = entries_list_path()?;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(entries_list_file.as_path())?;
    let mut entries_set: EntriesSet = if file.metadata()?.len().eq(&0) {
        Default::default()
    } else {
        serde_json::from_reader(&file)?
    };
    let need_to_write = entries_set.entries.insert(entry.to_owned());

    if need_to_write {
        // write JSON data
        // use File::rewind so we don't append data to the file
        // but rather write all (because we have read the data before)

        serde_json::to_writer_pretty(File::create(entries_list_file)?, &entries_set)?;
    }

    Ok(())
}

pub fn remove_entry(entry: &str) -> color_eyre::Result<()> {
    let entries_list_file = entries_list_path()?;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(entries_list_file.as_path())?;
    let mut entries_set: EntriesSet = if file.metadata()?.len().eq(&0) {
        Default::default()
    } else {
        serde_json::from_reader(&file)?
    };
    let need_to_write = entries_set.entries.remove(entry);

    if need_to_write {
        // write JSON data
        // use File::rewind so we don't append data to the file
        // but rather write all (because we have read the data before)

        serde_json::to_writer_pretty(File::create(entries_list_file)?, &entries_set)?;
    }

    Ok(())
}

pub fn entry_for_seed(name: &str) -> keyring::Result<Entry> {
    Entry::new("cw-cli", name)
}

pub fn seed_phrase_for_id(name: &str) -> color_eyre::Result<String> {
    let entry = entry_for_seed(name)?;
    let password = entry.get_password()?;
    // Found password - so we can save entry
    save_entry_if_required(name)?;
    let phrase = String::from_utf8(B64.decode(password)?)?;
    Ok(phrase)
}
