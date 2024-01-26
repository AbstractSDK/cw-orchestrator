// TODO: Three modes
// - Only alias (will allow making dropdown for addresses)
// - Only raw address
// - Hybrid (current)

const ADDRESS_BOOK_FILENAME: &str = "address_book.json";
const CLI_FOLDER: &str = "cli";

use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
    str::FromStr,
};

use cosmrs::AccountId;
use cw_orch::daemon::ChainInfo;
use cw_orch_core::env::default_state_folder;
use serde_json::{json, Value};

fn cli_path() -> color_eyre::Result<PathBuf> {
    let cli_path = default_state_folder()?.join(CLI_FOLDER);
    std::fs::create_dir_all(cli_path.as_path())?;
    Ok(cli_path)
}

fn address_book_path() -> color_eyre::Result<PathBuf> {
    Ok(cli_path()?.join(ADDRESS_BOOK_FILENAME))
}

pub fn get_account_id(chain_id: &str, name_alias: &str) -> color_eyre::Result<Option<AccountId>> {
    let address_book_file = address_book_path()?;
    // open file pointer set read permissions to true
    let file_result = OpenOptions::new()
        .read(true)
        .open(address_book_file.as_path());
    let file = match file_result {
        Ok(file) => file,
        // Unable to read/open file
        Err(_) => return Ok(None),
    };

    let json: Value = serde_json::from_reader(file)?;

    if let Some(address) = json.get(chain_id).and_then(|chain| chain.get(name_alias)) {
        if let Some(Ok(account_id)) = address.as_str().map(AccountId::from_str) {
            Ok(Some(account_id))
        } else {
            Err(color_eyre::eyre::eyre!(
                "Address Book file is damaged. Unable to read address for the [{name_alias}] alias"
            ))
        }
    } else {
        Ok(None)
    }
}

pub fn insert_account_id(
    chain_id: &str,
    name_alias: &str,
    address: &str,
) -> color_eyre::Result<AccountId> {
    // Before doing anything - validate if address is valid
    let account_id = AccountId::from_str(address)?;
    let address_book_file = address_book_path()?;
    // open file pointer set read/write permissions to true
    // create it if it does not exists
    // don't truncate it
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(address_book_file.as_path())?;
    // return empty json object if file is empty
    // return file content if not
    let mut json: Value = if file.metadata()?.len().eq(&0) {
        json!({})
    } else {
        serde_json::from_reader(file)?
    };

    // check and add chain_id path if it's missing
    if json.get(chain_id).is_none() {
        json[chain_id] = json!({
            name_alias: account_id
        });
    } else {
        json[chain_id][name_alias] = json!(account_id);
    }

    // write JSON data
    // use File::create so we don't append data to the file
    // but rather write all (because we have read the data before)
    serde_json::to_writer_pretty(File::create(address_book_file)?, &json).unwrap();

    Ok(account_id)
}

pub fn remove_account_id(chain_id: &str, name_alias: &str) -> color_eyre::Result<Option<Value>> {
    let address_book_file = address_book_path()?;
    // open file pointer set read/write permissions to true
    // create it if it does not exists
    // don't truncate it
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(false)
        .open(address_book_file.as_path())?;

    let mut json: serde_json::Map<String, Value> = serde_json::from_reader(file)?;
    let aliases_map = match json.get_mut(chain_id) {
        Some(aliases) => aliases.as_object_mut().unwrap(),
        None => return Ok(None),
    };
    let removed = aliases_map.remove(name_alias);
    if aliases_map.is_empty() {
        // Last alias - remove chain entry
        json.remove(chain_id);
    }
    // write JSON data
    // use File::create so we don't append data to the file
    // but rather write all (because we have read the data before)
    serde_json::to_writer_pretty(File::create(address_book_file)?, &json)?;
    Ok(removed)
}

// TODO: do we save alias on failed tx?
// I think yes, assuming only tx was wrong and address got checked already
// In the worst case user can edit address book
pub fn get_or_prompt_account_id(chain_id: &str, name_alias: &str) -> color_eyre::Result<AccountId> {
    let address_book_file = address_book_path()?;
    // open file pointer set read/write permissions to true
    // create it if it does not exists
    // don't truncate it
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(address_book_file.as_path())?;
    // return empty json object if file is empty
    // return file content if not
    let mut json: Value = if file.metadata()?.len().eq(&0) {
        json!({})
    } else {
        serde_json::from_reader(file)?
    };

    // check and add chain_id path if it's missing
    if json.get(chain_id).is_none() {
        json[chain_id] = json!({});
    }

    // retrieve existing alias
    if let Some(address) = json[chain_id].get(name_alias) {
        return if let Some(Ok(account_id)) = address.as_str().map(AccountId::from_str) {
            Ok(account_id)
        } else {
            Err(color_eyre::eyre::eyre!(
                "Address Book file is damaged. Unable to read address for the [{name_alias}] alias"
            ))
        };
    }

    // add name alias to chain_id path
    let message = format!("Write down the address for the [{name_alias}] alias");
    let account_id = loop {
        let address = inquire::Text::new(&message).prompt()?;
        if let Ok(account_id) = cosmrs::AccountId::from_str(&address) {
            break account_id;
        } else {
            eprintln!("Failed to parse bech32 address");
        }
    };

    json[chain_id][name_alias] = json!(account_id);

    // write JSON data
    // use File::create so we don't append data to the file
    // but rather write all (because we have read the data before)
    serde_json::to_writer_pretty(File::create(address_book_file)?, &json).unwrap();
    Ok(account_id)
}

pub fn select_alias(chain_id: &str) -> color_eyre::eyre::Result<Option<String>> {
    let address_book_file = address_book_path()?;

    let file = OpenOptions::new()
        .read(true)
        .open(address_book_file.as_path())
        .map_err(|_| color_eyre::eyre::eyre!("Must have at least one address in address book"))?;

    let json: Value = serde_json::from_reader(file)?;
    let chain_map = json
        .as_object()
        .ok_or(color_eyre::eyre::eyre!("Address Book file is damaged."))?;
    let alias_map = match chain_map.get(chain_id) {
        Some(aliases) => aliases.as_object().unwrap(),
        None => return Err(color_eyre::eyre::eyre!("Aliases for {chain_id} is empty")),
    };
    let aliases: Vec<&String> = alias_map.keys().collect();
    let chosen = inquire::Select::new("Select Address Alias", aliases).prompt()?;
    Ok(Some(chosen.to_owned()))
}

/// Address or alias to the address
#[derive(Debug, Clone)]
pub enum Address {
    Bech32(AccountId),
    Alias(String),
}

impl Address {
    // TODO: handle CLI config
    pub fn new(bech_or_addr: String, chain_info: &ChainInfo) -> color_eyre::Result<Self> {
        match cosmrs::AccountId::from_str(&bech_or_addr) {
            // Raw address
            Ok(account_id) => {
                if account_id.prefix() != chain_info.network_info.pub_address_prefix {
                    // Not recoverable at this point assuming user chose wrong chain
                    Err(color_eyre::eyre::eyre!(
                        "Prefix of bech32 address don't match for {}, expected_prefix: {}",
                        chain_info.chain_id,
                        chain_info.network_info.pub_address_prefix
                    ))
                } else {
                    Ok(Address::Bech32(account_id))
                }
            }
            // Name alias
            Err(_) => Ok(Address::Alias(bech_or_addr)),
        }
    }
}

#[derive(Debug, Clone, derive_more::AsRef, derive_more::From, derive_more::Into)]
#[as_ref(forward)]
pub struct CliAddress(String);

impl FromStr for CliAddress {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Address alias be empty".to_owned());
        }

        Ok(Self(s.to_owned()))
    }
}

impl interactive_clap::ToCli for CliAddress {
    type CliVariant = CliAddress;
}

impl CliAddress {
    pub fn account_id(self, chain_info: &ChainInfo) -> color_eyre::Result<AccountId> {
        match Address::new(self.0, chain_info)? {
            Address::Bech32(account_id) => Ok(account_id),
            Address::Alias(alias) => get_or_prompt_account_id(chain_info.chain_id, &alias),
        }
    }
}

impl std::fmt::Display for CliAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
