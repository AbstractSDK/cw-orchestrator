pub use base64::prelude::BASE64_STANDARD as B64;
use base64::Engine;
use color_eyre::eyre::Context;
use inquire::{error::InquireResult, InquireError};
use interactive_clap::ToCli;
use keyring::Entry;

pub fn entry_for_seed(name: &str) -> keyring::Result<Entry> {
    Entry::new_with_target("cw-orch", "cw-cli", name)
}

#[derive(Default, PartialEq, Eq, Debug, Clone)]
pub struct CliCoins(pub cosmwasm_std::Coins);

impl TryFrom<&CliCoins> for Vec<cosmrs::Coin> {
    type Error = color_eyre::Report;

    fn try_from(value: &CliCoins) -> Result<Self, Self::Error> {
        value
            .0
            .iter()
            .map(|cosmwasm_std::Coin { amount, denom }| {
                Ok(cosmrs::Coin {
                    amount: amount.u128(),
                    denom: denom.parse()?,
                })
            })
            .collect()
    }
}

impl std::fmt::Display for CliCoins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for CliCoins {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coins = cosmwasm_std::Coins::from_str(s).map_err(|e| e.to_string())?;
        Ok(CliCoins(coins))
    }
}

impl ToCli for CliCoins {
    type CliVariant = CliCoins;
}

pub fn seed_phrase_for_id(name: &str) -> color_eyre::Result<String> {
    let entry = entry_for_seed(&name)?;
    let password = entry.get_password()?;
    let phrase = String::from_utf8(B64.decode(password)?)?;
    Ok(phrase)
}

pub fn get_cw_cli_exec_path() -> String {
    std::env::args().next().unwrap()
}

pub fn parse_coins() -> InquireResult<cosmwasm_std::Coins> {
    let mut coins = cosmwasm_std::Coins::default();
    loop {
        let coin = inquire::CustomType::<cosmwasm_std::Coin>::new("Add coin to transaction")
            .with_placeholder("5ucosm")
            .with_help_message("Press ESC to finish adding coins")
            .prompt_skippable()?;
        if let Some(c) = coin {
            coins
                .add(c)
                .map_err(|e| InquireError::Custom(Box::new(e)))?
        } else {
            break;
        }
    }
    Ok(coins)
}
