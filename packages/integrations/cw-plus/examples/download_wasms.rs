use cw_orch::anyhow;
use std::{io::Cursor, path::PathBuf, str::FromStr};

pub const CW_PLUS_REPO_OWNER: &str = "CosmWasm";
pub const CW_PLUS_REPO_NAME: &str = "cw-plus";
pub const CW_PLUS_RELEASE_TAG: &str = "v2.0.0";

pub const ALL_CONTRACTS: &[&str] = &[
    "cw1_subkeys",
    "cw1_whitelist",
    "cw3_fixed_multisig",
    "cw3_flex_multisig",
    "cw4_group",
    "cw4_stake",
    "cw20_base",
    "cw20_ics20",
];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let artifacts_dir = PathBuf::from_str(crate_dir)?.join("artifacts");

    // We create the artifacts directory if non-existent
    std::fs::create_dir_all(&artifacts_dir)?;

    // We get the release, common for all artifacts
    let release = octocrab::instance()
        .repos(CW_PLUS_REPO_OWNER, CW_PLUS_REPO_NAME)
        .releases()
        .get_by_tag(CW_PLUS_RELEASE_TAG)
        .await?;

    for contract in ALL_CONTRACTS {
        let release_file_name = format!("{contract}.wasm");
        let file_name = artifacts_dir.join(&release_file_name);

        let wasm_asset = release
            .assets
            .iter()
            .find(|asset| asset.name.eq(&release_file_name))
            .unwrap();

        let response = reqwest::get(wasm_asset.browser_download_url.clone()).await?;
        let mut file = std::fs::File::create(file_name)?;
        let mut content = Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
    }
    Ok(())
}
