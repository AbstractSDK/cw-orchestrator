mod cosmwasm_tx;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = TxContext)]
pub struct TxCommands {
    /// Chain id
    chain_id: String,
    #[interactive_clap(subcommand)]
    action: TxAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = TxContext)]
/// Select type of transaction
pub enum TxAction {
    /// Cosmwasm Action
    #[strum_discriminants(strum(message = "Perform CosmWasm action"))]
    Cw(cosmwasm_tx::CwCommands),
}

impl From<TxContext> for () {
    fn from(_value: TxContext) -> Self {
        ()
    }
}

#[derive(Clone)]
pub struct TxContext {
    chain_id: String,
}

impl TxContext {
    fn from_previous_context(
        _previous_context: (),
        scope:&<TxCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(TxContext {
            chain_id: scope.chain_id.clone(),
        })
    }
}
