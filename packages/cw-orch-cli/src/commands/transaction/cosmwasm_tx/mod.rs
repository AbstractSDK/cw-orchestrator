mod execute;
pub mod msg_type;

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = CwActionContext)]
pub struct CwCommands {
    /// Contract addr
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
    #[strum_discriminants(strum(message = "Execute cosmwasm message"))]
    Execute(execute::ExecuteCommands),
    /// Query
    #[strum_discriminants(strum(message = "Query cosmwasm message"))]
    Query,
}

#[derive(Clone)]
pub struct CwActionContext {
    contract_addr: String,
}

impl CwActionContext {
    fn from_previous_context(
        _previous_context: (),
        scope:&<CwCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(CwActionContext {
            contract_addr: scope.contract_addr.clone(),
        })
    }
}
