pub use base64::prelude::BASE64_STANDARD as B64;
use base64::Engine;
use cw_orch::daemon::networks::SUPPORTED_NETWORKS as NETWORKS;
use inquire::{error::InquireResult, InquireError, Select};
use keyring::Entry;

pub fn entry_for_seed(name: &str) -> keyring::Result<Entry> {
    Entry::new("cw-cli", name)
}

pub fn seed_phrase_for_id(name: &str) -> color_eyre::Result<String> {
    let entry = entry_for_seed(name)?;
    let password = entry.get_password()?;
    let phrase = String::from_utf8(B64.decode(password)?)?;
    Ok(phrase)
}

pub fn get_cw_cli_exec_path() -> String {
    std::env::args().next().unwrap()
}

pub fn select_chain() -> color_eyre::eyre::Result<Option<String>> {
    let chain_ids: Vec<_> = NETWORKS
        .iter()
        .map(|network| {
            format!(
                "{} {}({})",
                network.network_info.id.to_uppercase(),
                network.kind.to_string().to_uppercase(),
                network.chain_id
            )
        })
        .collect();
    let selected = Select::new("Select chain", chain_ids).raw_prompt()?;
    let chain_id = NETWORKS[selected.index].chain_id.to_owned();
    Ok(Some(chain_id))
}

pub fn parse_coins() -> InquireResult<cosmwasm_std::Coins> {
    let mut coins = cosmwasm_std::Coins::default();
    loop {
        let coin = inquire::Text::new("Add coin to transaction")
            .with_placeholder("0ucoin")
            .with_help_message("Press ESC to finish adding coins")
            .prompt()?;
        if !coin.is_empty() {
            match coin.parse() {
                Ok(c) => coins
                    .add(c)
                    .map_err(|e| InquireError::Custom(Box::new(e)))?,
                Err(e) => {
                    println!("Failed to add coin: {e}")
                }
            }
        } else {
            break;
        }
    }
    println!("attached coins: {coins}");
    Ok(coins)
}

#[derive(Clone, Copy, strum::EnumIter, strum::EnumString, derive_more::Display)]
pub enum ExpirationType {
    AtHeight,
    AtTime,
    Never,
}

impl ExpirationType {
    const VARIANTS: &'static [ExpirationType] = &[Self::AtHeight, Self::AtTime, Self::Never];
}

pub fn parse_expiration() -> InquireResult<cw_utils::Expiration> {
    let locked = inquire::Select::new("Choose expiration type", ExpirationType::VARIANTS.to_vec())
        .prompt_skippable()?
        .unwrap_or(ExpirationType::Never);

    let expiration = match locked {
        ExpirationType::AtHeight => {
            let block_height = inquire::CustomType::<u64>::new("Input block height").prompt()?;
            cw_utils::Expiration::AtHeight(block_height)
        }
        ExpirationType::AtTime => {
            let timestamp_nanos =
                inquire::CustomType::<u64>::new("Input timestamp in nanos").prompt()?;
            let timestamp = cosmwasm_std::Timestamp::from_nanos(timestamp_nanos);
            cw_utils::Expiration::AtTime(timestamp)
        }
        ExpirationType::Never => cw_utils::Expiration::Never {},
    };
    Ok(expiration)
}
