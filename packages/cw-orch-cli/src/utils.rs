use keyring::Entry;

pub fn entry_for_seed(name: &str) -> keyring::Result<Entry> {
    Entry::new_with_target("cw-orch", "cw-cli", name)
}

pub fn get_cw_cli_exec_path() -> String {
    std::env::args().next().unwrap()
}
