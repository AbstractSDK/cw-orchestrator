use clap::Parser;
use clap::ValueEnum;

use common::ica_demo::full_ica_test;
use cw_orch::tokio;
use cw_orch_interchain::prelude::*;

// Integrating the test inside the example
mod common {
    include!("../tests/common/mod.rs");
}
#[derive(ValueEnum, Debug, Clone)] // ArgEnum here
#[clap(rename_all = "kebab_case")]
enum ChannelCreationType {
    Starship,
    Manual,
}

impl Default for ChannelCreationType {
    fn default() -> Self {
        Self::Manual
    }
}

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// execution type, (starship or manual channel creation)
    #[arg(short, long, value_enum)]
    channel_creation_type: ChannelCreationType,
}

pub const JUNO: &str = "juno-1";
pub const STARGAZE: &str = "stargaze-1";
pub const JUNO_FUNDS_DENOM: &str = "ujuno";

fn main() {
    dotenv().ok();
    use dotenv::dotenv;
    env_logger::init();

    // Depending on binary arguments, we se starship or a rpc based solution (with manual channel creation)
    let args = Arguments::parse();

    let rt: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
    let starship = Starship::new(rt.handle(), None).unwrap();
    if let Err(ref err) = match args.channel_creation_type {
        ChannelCreationType::Starship => {
            let interchain = starship.interchain_env();
            full_ica_test(&interchain, JUNO, STARGAZE, JUNO_FUNDS_DENOM)
        }
        ChannelCreationType::Manual => {
            let interchain = DaemonInterchainEnv::from_daemons(
                rt.handle(),
                starship.daemons.values().cloned().collect(),
                &ChannelCreationValidator,
            );
            full_ica_test(&interchain, JUNO, STARGAZE, JUNO_FUNDS_DENOM)
        }
    } {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));
        ::std::process::exit(1);
    }
}
