use cw_orch::{prelude::{DaemonAsync, networks::parse_network, DaemonAsyncBuilder}, tokio::{task, runtime::Handle}};
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = ())]
#[interactive_clap(output_context = ChainDaemonContext)]
pub struct TxCommands {
    chain_id: String,
    #[interactive_clap(subcommand)]
    action: CwAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = DaemonAsync)]
/// Select cosmwasm action
pub enum CwAction {
    /// Execute
    #[strum_discriminants(strum(message = "Execute cosmwasm message"))]
    Execute,
    /// Query
    #[strum_discriminants(strum(message = "Query cosmwasm message"))]
    Query,
}

pub struct ChainDaemonContext(DaemonAsync);

impl From<ChainDaemonContext> for DaemonAsync {
    fn from(value: ChainDaemonContext) -> Self {
        value.0
    }
}

impl ChainDaemonContext {
    fn from_previous_context(
        _previous_context: (),
        scope:&<TxCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        // TODO: implement no-panic parse_network
        let chain = parse_network(&scope.chain_id);

        let daemon = task::block_in_place(move || {
            Handle::current().block_on(async move {
                {
                    DaemonAsyncBuilder::default()
                        .chain(chain)
                        .build()
                        .await
                        .unwrap()
                }
            })
        });

        Ok(ChainDaemonContext(daemon))
    }
}