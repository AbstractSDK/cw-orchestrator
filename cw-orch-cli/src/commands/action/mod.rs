mod asset;
mod cosmwasm;
mod cw_ownable;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

use crate::types::CliLockedChain;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = CosmosContext)]
pub struct CosmosCommands {
    #[interactive_clap(skip_default_input_arg)]
    /// Chain id
    chain_id: CliLockedChain,
    #[interactive_clap(subcommand)]
    action: CosmosAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = CosmosContext)]
/// Select type of cosmos action
pub enum CosmosAction {
    /// Cosmwasm Action: store, instantiate, execute or query cosmwasm contract
    #[strum_discriminants(strum(message = "🔮 CosmWasm"))]
    Cw(cosmwasm::CwCommands),
    /// Asset Action
    #[strum_discriminants(strum(message = "🏦 Asset"))]
    Asset(asset::AssetCommands),
    /// CW-Ownable Action
    #[strum_discriminants(strum(message = "👑 CW-Ownable"))]
    CwOwnable(cw_ownable::CwOwnableCommands),
}

impl CosmosCommands {
    fn input_chain_id(_context: &()) -> color_eyre::eyre::Result<Option<CliLockedChain>> {
        crate::common::select_chain()
    }
}

impl From<CosmosContext> for () {
    fn from(_value: CosmosContext) -> Self {}
}

#[derive(Clone)]
pub struct CosmosContext {
    pub chain: CliLockedChain,
}

impl CosmosContext {
    fn from_previous_context(
        _previous_context: (),
        scope:&<CosmosCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(CosmosContext {
            chain: scope.chain_id,
        })
    }
}