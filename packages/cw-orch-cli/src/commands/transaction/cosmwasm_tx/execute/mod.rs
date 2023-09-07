use cw_orch::prelude::{networks::parse_network, DaemonBuilder, TxHandler};
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = ExecuteWasmOutput)]
pub struct ExecuteCommands {
    // TODO: verify it exists?
    contract_address: String,
    #[interactive_clap(subcommand)]
    action: ExecuteAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = ExecuteWasmOutput)]
/// Select cosmwasm action
pub enum ExecuteAction {
    /// Query
    #[strum_discriminants(strum(message = "Query cosmwasm message"))]
    Query,
}

pub struct ExecuteWasmOutput;

impl ExecuteWasmOutput {
    fn from_previous_context(
        _previous_context: (),
        scope:&<ExecuteAction as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = parse_network("uni-6");
        let seed = crate::utils::seed_phrase_for_id("aloha")?;
        let coins = crate::utils::parse_coins()?;

        let d = DaemonBuilder::default().chain(chain).build()?;
        d.execute(exec_msg, &coins, contract_address);
        Ok(ExecuteWasmOutput)
    }
}
