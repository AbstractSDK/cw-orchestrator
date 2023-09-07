mod commands;
mod utils;
use interactive_clap::{ResultFromCli, ToCliArgs};

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
pub struct TLCommand {
    #[interactive_clap(subcommand)]
    top_level: commands::Commands,
}

fn main() -> color_eyre::Result<()> {
    // TODO: add some configuration like default chain/signer/etc
    let cli_args = TLCommand::parse();

    let cw_cli_path = utils::get_cw_cli_exec_path();
    loop {
        let args = <TLCommand as interactive_clap::FromCli>::from_cli(Some(cli_args.clone()), ());
        match args {
            interactive_clap::ResultFromCli::Ok(cli_args)
            | ResultFromCli::Cancel(Some(cli_args)) => {
                println!(
                    "Your console command: {}",
                    shell_words::join(std::iter::once(cw_cli_path).chain(cli_args.to_cli_args()))
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
                        shell_words::join(
                            std::iter::once(cw_cli_path).chain(cli_args.to_cli_args())
                        )
                    );
                }
                return Err(err);
            }
        }
    }
}
