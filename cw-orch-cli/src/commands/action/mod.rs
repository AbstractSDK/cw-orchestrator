use crate::{types::CliLockedChain, GlobalConfig};

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod asset;
mod cosmwasm;
mod cw3;
mod cw_ownable;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = GlobalConfig)]
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
    /// CW3 Action
    #[strum_discriminants(strum(message = "🤝 CW3"))]
    Cw3(cw3::Cw3Commands),
}

impl CosmosCommands {
    fn input_chain_id(_context: &GlobalConfig) -> color_eyre::eyre::Result<Option<CliLockedChain>> {
        crate::common::select_chain()
    }
}

#[derive(Clone)]
pub struct CosmosContext {
    pub chain: CliLockedChain,
    pub global_config: GlobalConfig,
}

impl CosmosContext {
    fn from_previous_context(
        previous_context: GlobalConfig,
        scope:&<CosmosCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(CosmosContext {
            chain: scope.chain_id,
            global_config: previous_context,
        })
    }
}
