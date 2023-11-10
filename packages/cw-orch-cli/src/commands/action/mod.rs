mod cosmwasm_tx;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = CosmosContext)]
pub struct CosmosCommands {
    /// Chain id
    chain_id: String,
    #[interactive_clap(subcommand)]
    action: CosmosAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = CosmosContext)]
/// Select type of cosmos action
pub enum CosmosAction {
    /// Cosmwasm Action
    #[strum_discriminants(strum(message = "Perform CosmWasm action"))]
    Cw(cosmwasm_tx::CwCommands),
}

impl From<CosmosContext> for () {
    fn from(_value: CosmosContext) -> Self {}
}

#[derive(Clone)]
pub struct CosmosContext {
    chain_id: String,
}

impl CosmosContext {
    fn from_previous_context(
        _previous_context: (),
        scope:&<CosmosCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(CosmosContext {
            chain_id: scope.chain_id.clone(),
        })
    }
}
