use cw_orch::{
    prelude::{networks::parse_network, Daemon, DaemonBuilder},
    tokio::runtime::{Handle, Runtime},
};

use interactive_clap::{ResultFromCli, ToCliArgs};
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = Handle)]
#[interactive_clap(output_context = ChainDaemonContext)]
pub struct Commands {
    chain_id: String,
    #[interactive_clap(subcommand)]
    action: CwAction,
}

pub struct InitialContext<'a>(&'a Handle);

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = Daemon)]
pub enum CwAction {
    /// Execute
    #[strum_discriminants(strum(message = "Execute cosmwasm action"))]
    Execute,
    /// Query
    #[strum_discriminants(strum(message = "Query cosmwasm action"))]
    Query,
}

pub struct ChainDaemonContext(Daemon);

impl From<ChainDaemonContext> for Daemon {
    fn from(value: ChainDaemonContext) -> Self {
        value.0
    }
}

impl ChainDaemonContext {
    fn from_previous_context(
        _previous_context: Handle,
        scope:&<Commands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = parse_network(&scope.chain_id);
        let daemon = DaemonBuilder::default()
            .handle(&_previous_context)
            .chain(chain)
            .build()?;
        Ok(ChainDaemonContext(daemon))
    }
}

fn main() -> color_eyre::Result<()> {
    dotenv::dotenv().ok();
    let mut cli_args = Commands::parse();
    let runtime = Runtime::new()?;

    loop {
        let args = <Commands as interactive_clap::FromCli>::from_cli(
            Some(cli_args),
            runtime.handle().clone(),
        );
        match args {
            interactive_clap::ResultFromCli::Ok(cli_args)
            | ResultFromCli::Cancel(Some(cli_args)) => {
                println!(
                    "Your console command:  {}",
                    shell_words::join(cli_args.to_cli_args())
                );
                return Ok(());
            }
            interactive_clap::ResultFromCli::Cancel(None) => {
                println!("Goodbye!");
                return Ok(());
            }
            interactive_clap::ResultFromCli::Back => {
                cli_args = Default::default();
            }
            interactive_clap::ResultFromCli::Err(cli_args, err) => {
                if let Some(cli_args) = cli_args {
                    println!(
                        "Your console command: {}",
                        shell_words::join(cli_args.to_cli_args())
                    );
                }
                return Err(err);
            }
        }
    }
}
