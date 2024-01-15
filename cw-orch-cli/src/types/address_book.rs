// TODO: Three modes
// - Only alias (will allow making dropdown for addresses)
// - Only raw address
// - Hybrid (current)

const ADDRESS_BOOK_FILENAME: &str = "address_book.json";
const CLI_FOLDER: &str = "cli";

use std::{
    fs::{File, OpenOptions},
    str::FromStr,
};

use cosmrs::AccountId;
use cw_orch::daemon::ChainInfo;
use cw_orch_core::env::default_state_folder;
use serde_json::{json, Value};

// TODO: do we save alias on failed tx?
// I think yes, assuming only tx was wrong and address got checked already
// In the worst case user can edit address book
fn get_or_insert_account_id(chain_id: &str, name_alias: &str) -> color_eyre::Result<AccountId> {
    // open file pointer set read/write permissions to true
    // create it if it does not exists
    // don't truncate it
    let cli_path = default_state_folder()?.join(CLI_FOLDER);
    std::fs::create_dir_all(cli_path.as_path())?;

    let address_book_file = cli_path.join(ADDRESS_BOOK_FILENAME);

    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(address_book_file.as_path())?;

    // return empty json object if file is empty
    // return file content if not
    let mut json: Value = if file.metadata().unwrap().len().eq(&0) {
        json!({})
    } else {
        serde_json::from_reader(file).unwrap()
    };

    // check and add chain_id path if it's missing
    if json.get(chain_id).is_none() {
        json[chain_id] = json!({});
    }

    // add name alias to chain_id path
    if let Some(address) = json[chain_id].get(name_alias) {
        return if let Some(Ok(account_id)) = address.as_str().map(AccountId::from_str) {
            Ok(account_id)
        } else {
            Err(color_eyre::eyre::eyre!(
                "Address Book file is damaged, unable to read address for the provided alias"
            ))
        };
    }

    let account_id = loop {
        let address = inquire::Text::new(&format!(
            "Write down the address for the [{name_alias}] alias"
        ))
        .prompt()?;
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
    serde_json::to_writer_pretty(File::create(address_book_file).unwrap(), &json).unwrap();
    Ok(account_id)
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
            Address::Alias(alias) => get_or_insert_account_id(chain_info.chain_id, &alias),
        }
    }
}

impl std::fmt::Display for CliAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
