pub use base64::prelude::BASE64_STANDARD as B64;
use base64::Engine;
use cosmwasm_std::Coin;
use inquire::error::InquireResult;
use keyring::Entry;

pub fn entry_for_seed(name: &str) -> keyring::Result<Entry> {
    Entry::new_with_target("cw-orch", "cw-cli", name)
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

pub fn parse_coins() -> InquireResult<Vec<Coin>> {
    let mut coins = Vec::new();
    loop {
        let coin = inquire::CustomType::<Coin>::new("Add coin to transaction")
            .with_placeholder("5ucosm")
            .with_help_message("Press ESC to finish adding coins")
            .prompt_skippable()?;
        if let Some(c) = coin {
            coins.push(c)
        } else {
            break;
        }
    }
    Ok(coins)
}
