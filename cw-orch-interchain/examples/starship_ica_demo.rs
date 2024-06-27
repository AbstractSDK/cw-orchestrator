use common::ica_demo::full_ica_test;
use cw_orch_interchain::prelude::*;

// Integrating the test inside the example
mod common {
    include!("../tests/common/mod.rs");
}

pub const JUNO: &str = "juno-1";
pub const STARGAZE: &str = "stargaze-1";
pub const JUNO_FUNDS_DENOM: &str = "ujuno";

fn main() {
    dotenv().ok();
    use dotenv::dotenv;
    env_logger::init();

    // We use starship to create channels
    let starship = Starship::new(None).unwrap();
    let interchain = starship.interchain_env();

    full_ica_test(&interchain, JUNO, STARGAZE, JUNO_FUNDS_DENOM).unwrap();
}
