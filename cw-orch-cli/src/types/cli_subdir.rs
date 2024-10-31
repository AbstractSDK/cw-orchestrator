use std::path::PathBuf;

use cw_orch::env_vars::default_state_folder;

pub const CLI_FOLDER: &str = "cli";

pub fn cli_path() -> color_eyre::Result<PathBuf> {
    let cli_path = default_state_folder()?.join(CLI_FOLDER);
    std::fs::create_dir_all(cli_path.as_path())?;
    Ok(cli_path)
}
