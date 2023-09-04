use base64::{engine::general_purpose::STANDARD as B64, Engine};
use color_eyre::eyre::Context;
use cw_orch::{
    prelude::{networks::parse_network, Daemon, DaemonBuilder},
    tokio::runtime::{Handle, Runtime},
};

use interactive_clap::{ResultFromCli, ToCliArgs};
use strum::{EnumDiscriminants, EnumIter, EnumMessage};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = Handle)]
#[interactive_clap(output_context = ChainDaemonContext)]
pub struct TxCommands {
    chain_id: String,
    #[interactive_clap(subcommand)]
    action: CwAction,
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = Handle)]
pub struct KeyCommands {
    #[interactive_clap(subcommand)]
    key_actions: KeyAction,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = Handle)]
pub enum KeyAction {
    /// Add key to keyring
    #[strum_discriminants(strum(message = "Add key to the keyring"))]
    AddKey(AddKeyCommand),
    /// Show key
    #[strum_discriminants(strum(message = "Show key of given id from the keyring"))]
    ShowKey,
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = Handle)]
#[interactive_clap(output_context = AddKeyContext)]
pub struct AddKeyCommand {
    // TODO: add checker for repetition
    // #[interactive_clap(skip_default_input_arg)]
    name: String,
    #[interactive_clap(subcommand)]
    key_actions: AddKeyActions,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(input_context = AddKeyContext)]
#[interactive_clap(output_context = AddKeyOutput)]
pub enum AddKeyActions {
    /// Generate new random key
    #[strum_discriminants(strum(message = "Generate new random key"))]
    New,
    /// Recover key from the seed phrase
    #[strum_discriminants(strum(message = "Recover key from the seed phrase"))]
    FromSeed,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = Handle)]
///To construct a transaction you will need to provide information about sender (signer) and receiver accounts, and actions that needs to be performed.
///Do you want to derive some information required for transaction construction automatically querying it online?
pub enum Commands {
    /// Create transaction
    #[strum_discriminants(strum(message = "Create transaction"))]
    Tx(TxCommands),
    /// Key
    #[strum_discriminants(strum(message = "Input key"))]
    Key(KeyCommands),
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(context = Handle)]
pub struct TLCommand {
    #[interactive_clap(subcommand)]
    top_level: Commands,
}

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

#[derive(Clone)]
pub struct AddKeyContext(String);

impl AddKeyContext {
    fn from_previous_context(
        _previous_context: Handle,
        scope:&<AddKeyCommand as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(AddKeyContext(scope.name.clone()))
    }
}

pub struct AddKeyOutput;

impl AddKeyOutput {
    fn from_previous_context(
        previous_context: AddKeyContext,
        scope:&<AddKeyActions as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let name = previous_context.0;
        let mnemonic = match scope {
            AddKeyActionsDiscriminants::New => bip39::Mnemonic::generate(24),
            AddKeyActionsDiscriminants::FromSeed => {
                let mnemonic_seed =
                    rpassword::prompt_password("Mnemonic 🔑: ").context("unable to read")?;
                bip39::Mnemonic::parse_in(bip39::Language::English, mnemonic_seed)
            }
        }?;
        let entry = keyring::Entry::new("cw-cli", &name)?;

        let entropy = mnemonic.to_entropy();
        let password = B64.encode(&entropy);
        entry.set_password(&password)?;
        Ok(AddKeyOutput)
    }
}

pub struct ChainDaemonContext(Daemon);

impl From<ChainDaemonContext> for Daemon {
    fn from(value: ChainDaemonContext) -> Self {
        value.0
    }
}

impl ChainDaemonContext {
    fn from_previous_context(
        previous_context: Handle,
        scope:&<TxCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        // TODO: implement no-panic parse_network
        let chain = parse_network(&scope.chain_id);
        let daemon = DaemonBuilder::default()
            .handle(&previous_context)
            .chain(chain)
            .build()?;
        Ok(ChainDaemonContext(daemon))
    }
}

fn main() -> color_eyre::Result<()> {
    dotenv::dotenv().ok();
    let cli_args = TLCommand::parse();
    let runtime = Runtime::new()?;

    loop {
        let args = <TLCommand as interactive_clap::FromCli>::from_cli(
            Some(cli_args.clone()),
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
            interactive_clap::ResultFromCli::Back => {}
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
