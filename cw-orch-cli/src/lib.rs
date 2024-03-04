pub mod commands;
pub mod common;
pub(crate) mod log;
pub(crate) mod types;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = GlobalConfig)]
pub struct TLCommand {
    /// Verbose mode
    #[interactive_clap(short, long)]
    verbose: bool,
    /// Merge cw-orch state file in address-book
    #[interactive_clap(short, long)]
    cw_orch_merge_state: bool,
    #[interactive_clap(subcommand)]
    top_level: commands::Commands,
}

#[derive(Debug, Clone)]
pub struct GlobalConfig {
    cw_orch_merged_state: bool,
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
            cw_orch_merged_state: scope.cw_orch_merge_state,
        })
    }
}

impl From<GlobalConfig> for () {
    fn from(_value: GlobalConfig) -> Self {}
}
