mod cosmwasm_tx;
mod cw_ownable;
mod transfer_tx;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = CosmosContext)]
pub struct CosmosCommands {
    #[interactive_clap(skip_default_input_arg)]
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
    #[strum_discriminants(strum(message = "CosmWasm"))]
    Cw(cosmwasm_tx::CwCommands),
    // TODO: rename and expand to bank?
    // /// Transfer Action
    #[strum_discriminants(strum(message = "Transfer"))]
    Transfer(transfer_tx::TransferCommands),
    /// CW-Ownable Action
    #[strum_discriminants(strum(message = "CW-Ownable"))]
    CwOwnable(cw_ownable::CwOwnableCommands),
}

impl CosmosCommands {
    fn input_chain_id(_context: &()) -> color_eyre::eyre::Result<Option<String>> {
        crate::common::select_chain()
    }
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
