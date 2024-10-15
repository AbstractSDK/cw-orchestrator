use crate::{fetch::explorers::Explorers, types::CliLockedChain};
pub use base64::prelude::BASE64_STANDARD as B64;
use cw_orch::{
    daemon::networks::SUPPORTED_NETWORKS as NETWORKS,
    environment::{ChainInfo, ChainKind},
};
use ibc_chain_registry::fetchable::Fetchable;
use inquire::{error::InquireResult, InquireError, Select};

pub fn get_cw_cli_exec_path() -> String {
    std::env::args().next().unwrap()
}

pub fn select_chain() -> color_eyre::eyre::Result<Option<CliLockedChain>> {
    let chain_ids: Vec<_> = NETWORKS
        .iter()
        .map(|network| {
            format!(
                "{} {}({})",
                network.network_info.chain_name.to_uppercase(),
                network.kind.to_string().to_uppercase(),
                network.chain_id
            )
        })
        .collect();
    let selected = Select::new("Select chain", chain_ids).raw_prompt()?;
    let locked_chain = CliLockedChain::new(selected.index);
    Ok(Some(locked_chain))
}

pub fn select_signer() -> color_eyre::eyre::Result<Option<String>> {
    let entries_set_result = crate::types::keys::read_entries();
    let signer_id = match entries_set_result {
        // We have a file access and it has at least one signer
        Ok(entries_set) if !entries_set.entries.is_empty() => {
            let options = entries_set.entries.into_iter().collect();
            Select::new("Select signer id", options)
                .with_help_message("Use CLI mode to add signer from previous version")
                .prompt()?
        }
        // We don't have access or it's empty
        _ => inquire::Text::new("Signer id").prompt()?,
    };
    Ok(Some(signer_id))
}

pub fn parse_coins() -> InquireResult<cosmwasm_std::Coins> {
    let mut coins = cosmwasm_std::Coins::default();
    loop {
        let coin = inquire::Text::new("Add coin to transaction")
            .with_help_message("Leave empty to stop adding coins")
            .with_placeholder("0ucoin")
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
        .prompt()?;

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

pub async fn show_addr_explorer(chain_info: ChainInfo, addr: &str) -> color_eyre::eyre::Result<()> {
    if let ChainKind::Mainnet = chain_info.kind {
        let Explorers { explorers } =
            Explorers::fetch(chain_info.network_info.chain_name.to_owned(), None).await?;
        for explorer in explorers {
            if let Some(tx_page) = explorer.account_page {
                let url = tx_page.replace("${accountAddress}", addr);
                println!("Explorer: {url}");
                break;
            }
        }
    }
    Ok(())
}
