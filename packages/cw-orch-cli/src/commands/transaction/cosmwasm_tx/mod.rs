mod execute;
mod query;
pub mod msg_type;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

use super::TxContext;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = TxContext)]
#[interactive_clap(output_context = CwActionContext)]
pub struct CwCommands {
    /// Contract address
    contract_addr: String,
    #[interactive_clap(subcommand)]
    action: CwAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = CwActionContext)]
/// Select cosmwasm action
pub enum CwAction {
    /// Execute
    #[strum_discriminants(strum(message = "Execute"))]
    Execute(execute::ExecuteCommands),
    /// Query
    #[strum_discriminants(strum(message = "Query"))]
    Query(query::QueryCommands),
}

#[derive(Clone)]
pub struct CwActionContext {
    chain_id: String,
    contract_addr: String,
}

impl CwActionContext {
    fn from_previous_context(
        previous_context: TxContext,
        scope:&<CwCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(CwActionContext {
            chain_id: previous_context.chain_id.clone(),
            contract_addr: scope.contract_addr.clone(),
        })
    }
}
