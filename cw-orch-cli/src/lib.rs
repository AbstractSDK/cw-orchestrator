pub mod commands;
pub mod common;
pub mod fetch;
pub(crate) mod log;
pub(crate) mod types;

use cw_orch::daemon::DEFAULT_DEPLOYMENT;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = GlobalConfig)]
pub struct TLCommand {
    /// Verbose mode
    #[interactive_clap(short, long, global = true)]
    verbose: bool,
    /// Source cw-orch state file with address-book
    #[interactive_clap(short, long, global = true)]
    source_state_file: bool,
    /// Deployment id, that will be used for merging cw_orch_state
    #[interactive_clap(long, global = true)]
    #[interactive_clap(skip_interactive_input)]
    deployment_id: Option<String>,
    #[interactive_clap(subcommand)]
    top_level: commands::Commands,
}

#[derive(Debug, Clone)]
pub struct GlobalConfig {
    source_state_file: bool,
    deployment_id: String,
}

impl GlobalConfig {
    fn from_previous_context(
        _previous_context: (),
        scope: &<TLCommand as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        if scope.verbose {
            pretty_env_logger::init()
        }
        Ok(Self {
            source_state_file: scope.source_state_file,
            deployment_id: scope
                .deployment_id
                .clone()
                .unwrap_or(DEFAULT_DEPLOYMENT.to_owned()),
        })
    }
}

impl From<GlobalConfig> for () {
    fn from(_value: GlobalConfig) -> Self {}
}
