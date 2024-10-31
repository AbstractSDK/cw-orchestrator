use super::CosmosContext;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod query_cw20;
mod query_native;
mod send_cw20;
mod send_native;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = CosmosContext)]
pub struct AssetCommands {
    #[interactive_clap(subcommand)]
    action: AssetAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = CosmosContext)]
/// Select asset action
pub enum AssetAction {
    /// Native or factory coin send
    #[strum_discriminants(strum(message = "Send native coins"))]
    SendNative(send_native::SendNativeCommands),
    /// Cw20 coin transfer
    #[strum_discriminants(strum(message = "Send cw20 coin"))]
    SendCw20(send_cw20::Cw20TransferCommands),
    /// Native or factory coins query
    #[strum_discriminants(strum(message = "Query native coins"))]
    QueryNative(query_native::QueryNativeCommands),
    /// Cw20 coin query
    #[strum_discriminants(strum(message = "Query cw20 coins"))]
    QueryCw20(query_cw20::QueryCw20Commands),
    // TODO: cw720?
}
