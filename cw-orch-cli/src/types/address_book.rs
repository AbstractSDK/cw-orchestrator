use crate::GlobalConfig;

use super::cli_subdir::cli_path;

// TODO: Three modes
// - Only alias (will allow making dropdown for addresses)
// - Only raw address
// - Hybrid (current)

const ADDRESS_BOOK_FILENAME: &str = "address_book.json";
pub const CW_ORCH_STATE_FILE_DAMAGED_ERROR: &str = "cw-orch state file is corrupted";

use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
    str::FromStr,
};

use color_eyre::eyre::Context;
use cosmrs::AccountId;
use cw_orch::environment::ChainInfo;
use serde_json::{json, Value};

fn address_book_path() -> color_eyre::Result<PathBuf> {
    Ok(cli_path()?.join(ADDRESS_BOOK_FILENAME))
}

/// Get account id only from address-book
pub fn get_account_id_address_book(
    chain: &ChainInfo,
    name_alias: &str,
) -> color_eyre::Result<Option<AccountId>> {
    let address_book_file = address_book_path()?;
    let chain_id = chain.chain_id;
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

/// Get account id from both address-book and cw-orch state(if merging enabled)
pub fn get_account_id(
    chain: &ChainInfo,
    global_config: &GlobalConfig,
    name_alias: &str,
) -> color_eyre::Result<Option<AccountId>> {
    let account_id_address_book = get_account_id_address_book(chain, name_alias)?;
    // If address found in address book or cw-orch state sourcing disabled - no need to read cw orch state
    if account_id_address_book.is_some() || !global_config.source_state_file {
        return Ok(account_id_address_book);
    }
    // Try to load cw orch state contract
    let cw_orch_contracts = cw_orch_state_contracts(chain, &global_config.deployment_id)?;
    if let Some(contract) = cw_orch_contracts.get(name_alias) {
        let contract_addr = contract
            .as_str()
            .ok_or(color_eyre::eyre::eyre!(CW_ORCH_STATE_FILE_DAMAGED_ERROR))?;
        // Ignore parse error, cw-orch can store non bech32 addresses which CLI does not support
        Ok(AccountId::from_str(contract_addr).ok())
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
        serde_json::from_reader(&file)?
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
    // use File::rewind so we don't append data to the file
    // but rather write all (because we have read the data before)
    serde_json::to_writer_pretty(File::create(address_book_file)?, &json)?;

    Ok(account_id)
}

pub fn try_insert_account_id(
    chain: &ChainInfo,
    alias: &str,
    address: &str,
) -> color_eyre::eyre::Result<()> {
    let maybe_account_id = get_account_id_address_book(chain, alias)?;

    if let Some(account_id) = maybe_account_id {
        let confirmed =
            inquire::Confirm::new(&format!("Override {}({account_id})?", alias)).prompt()?;
        if confirmed {
            return Ok(());
        }
    }

    let new_address = insert_account_id(chain.chain_id, alias, address)?;
    println!("Wrote successfully:\n{}:{}", alias, new_address);
    Ok(())
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

pub fn get_or_prompt_account_id(
    chain: &ChainInfo,
    global_config: &GlobalConfig,
    name_alias: &str,
) -> color_eyre::Result<AccountId> {
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
    if json.get(chain.chain_id).is_none() {
        json[chain.chain_id] = json!({});
    }

    // Try to retrieve existing alias
    if let Some(address) = json[chain.chain_id].get(name_alias) {
        return if let Some(Ok(account_id)) = address.as_str().map(AccountId::from_str) {
            Ok(account_id)
        } else {
            Err(color_eyre::eyre::eyre!(
                "Address Book file is damaged. Unable to read address for the [{name_alias}] alias"
            ))
        };
    }
    // Try to retrieve from cw-orch state if merging enabled
    if global_config.source_state_file {
        let cw_orch_contracts = cw_orch_state_contracts(chain, &global_config.deployment_id)?;
        if let Some(contract) = cw_orch_contracts.get(name_alias) {
            let contract_addr = contract
                .as_str()
                .ok_or(color_eyre::eyre::eyre!(CW_ORCH_STATE_FILE_DAMAGED_ERROR))?;
            // Ignore parse error, cw-orch can store non bech32 addresses which CLI does not support
            if let Ok(account_id) = AccountId::from_str(contract_addr) {
                return Ok(account_id);
            }
        }
    }

    // add name alias to chain_id path
    let message = format!("Write down the address for the [{name_alias}] alias");
    let account_id = loop {
        let address = inquire::Text::new(&message).prompt()?;
        if let Ok(account_id) = cosmrs::AccountId::from_str(&address) {
            break account_id;
        }

        eprintln!("Failed to parse bech32 address");
    };

    json[chain.chain_id][name_alias] = json!(account_id);

    // write JSON data
    // use File::create so we don't append data to the file
    // but rather write all (because we have read the data before)
    serde_json::to_writer_pretty(File::create(address_book_file)?, &json).unwrap();
    Ok(account_id)
}

pub fn select_alias(
    chain_info: &ChainInfo,
    global_config: &GlobalConfig,
) -> color_eyre::eyre::Result<Option<String>> {
    let chain_id = chain_info.chain_id;

    let cw_orch_contracts = if global_config.source_state_file {
        cw_orch_state_contracts(chain_info, &global_config.deployment_id)?
    } else {
        Default::default()
    };

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
        Some(aliases) => aliases.as_object().unwrap().clone(),
        None => Default::default(),
    };
    let aliases: Vec<_> = alias_map.keys().chain(cw_orch_contracts.keys()).collect();
    if aliases.is_empty() {
        return Err(color_eyre::eyre::eyre!("Aliases for {chain_id} is empty"));
    }
    let chosen = inquire::Select::new("Select Address Alias", aliases).prompt()?;
    Ok(Some(chosen.to_owned()))
}

fn read_cw_orch_state() -> color_eyre::Result<Value> {
    let state_file = cw_orch::daemon::DaemonState::state_file_path()?;

    let file =
        File::open(&state_file).context(format!("File should be present at {state_file}"))?;
    let json: Value = serde_json::from_reader(file)?;
    Ok(json)
}

pub fn cw_orch_state_contracts(
    chain: &ChainInfo,
    deployment_id: &str,
) -> color_eyre::Result<serde_json::Map<String, Value>> {
    let chain_name = chain.network_info.chain_name;
    let chain_id = chain.chain_id;

    let json = read_cw_orch_state()?;

    let chain_state = if let Some(chain_state) = json.get(chain_name) {
        // In case old state
        // TODO: should be able to remove in the future
        chain_state
    } else {
        &json
    };

    let Some(chain_id_state) = chain_state.get(chain_id) else {
        return Err(color_eyre::eyre::eyre!("State is empty for {chain_id}"));
    };

    let Some(deployment) = chain_id_state.get(deployment_id) else {
        return Err(color_eyre::eyre::eyre!(
            "State is empty for {chain_id}.{deployment_id}"
        ));
    };

    let contracts = deployment
        .as_object()
        .ok_or(color_eyre::eyre::eyre!(CW_ORCH_STATE_FILE_DAMAGED_ERROR))?;
    Ok(contracts.clone())
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
            return Err("Address alias can't be empty".to_owned());
        }

        Ok(Self(s.to_owned()))
    }
}

impl interactive_clap::ToCli for CliAddress {
    type CliVariant = CliAddress;
}

impl CliAddress {
    pub fn account_id(
        self,
        chain_info: &ChainInfo,
        global_config: &GlobalConfig,
    ) -> color_eyre::Result<AccountId> {
        match Address::new(self.0, chain_info)? {
            Address::Bech32(account_id) => Ok(account_id),
            Address::Alias(alias) => get_or_prompt_account_id(chain_info, global_config, &alias),
        }
    }
}

impl std::fmt::Display for CliAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
