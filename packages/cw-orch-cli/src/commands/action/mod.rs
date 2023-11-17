mod cosmwasm_tx;
mod transfer_tx;

use cw_orch::daemon::networks::NETWORKS;
use inquire::Select;
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
    #[strum_discriminants(strum(message = "Perform CosmWasm action"))]
    Cw(cosmwasm_tx::CwCommands),
    // /// Transfer Action
    #[strum_discriminants(strum(message = "Perform Transfer action"))]
    Transfer(transfer_tx::TransferCommands),
}

impl CosmosCommands {
    fn input_chain_id(_context: &()) -> color_eyre::eyre::Result<Option<String>> {
        let chain_ids: Vec<_> = NETWORKS
            .iter()
            .map(|network| {
                format!(
                    "{} {}({})",
                    network.network_info.id.to_uppercase(),
                    network.kind.to_string().to_uppercase(),
                    network.chain_id
                )
            })
            .collect();
        let selected = Select::new("Select chain", chain_ids).raw_prompt()?;
        let chain_id = NETWORKS[selected.index].chain_id.to_owned();
        Ok(Some(chain_id))
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
