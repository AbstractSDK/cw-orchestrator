use crate::commands::action::CosmosContext;

use cw_orch::{daemon::DaemonBuilder, environment::ChainInfoOwned, prelude::*};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = QueryCodeOutput)]
pub struct QueryCodeCommands {
    /// Enter code id
    code_id: u64,
}

pub struct QueryCodeOutput;

impl QueryCodeOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<QueryCodeCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;

        let chain_data: ChainInfoOwned = chain.into();
        let daemon = DaemonBuilder::new(chain_data.clone()).build_sender(())?;
        let code_info = daemon.wasm_querier().code(scope.code_id)?;
        println!("{}", serde_json::to_string_pretty(&code_info)?);

        Ok(QueryCodeOutput)
    }
}
