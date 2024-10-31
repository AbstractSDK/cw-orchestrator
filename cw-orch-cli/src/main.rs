use cw_orch::daemon::env::LOGS_ACTIVATION_MESSAGE_ENV_NAME;
use cw_orch_cli::{common, TLCommand};

use inquire::ui::{Attributes, RenderConfig, StyleSheet};
use interactive_clap::{ResultFromCli, ToCliArgs};

fn main() -> color_eyre::Result<()> {
    // We don't want to see cw-orch logs during cli
    std::env::set_var(LOGS_ACTIVATION_MESSAGE_ENV_NAME, "false");
    let render_config = RenderConfig {
        prompt: StyleSheet::new().with_attr(Attributes::BOLD),
        ..Default::default()
    };
    inquire::set_global_render_config(render_config);
    // TODO: add some configuration like default chain/signer/etc
    let cli_args = TLCommand::parse();

    let cw_cli_path = common::get_cw_cli_exec_path();
    let args = <TLCommand as interactive_clap::FromCli>::from_cli(Some(cli_args.clone()), ());

    match args {
        interactive_clap::ResultFromCli::Ok(cli_args) | ResultFromCli::Cancel(Some(cli_args)) => {
            println!(
                "Your console command: {}",
                shell_words::join(std::iter::once(cw_cli_path).chain(cli_args.to_cli_args()))
            );
            Ok(())
        }
        interactive_clap::ResultFromCli::Cancel(None) => {
            println!("Goodbye!");
            Ok(())
        }
        interactive_clap::ResultFromCli::Back => {
            unreachable!("TLCommand does not have back option");
        }
        interactive_clap::ResultFromCli::Err(cli_args, err) => {
            if let Some(cli_args) = cli_args {
                println!(
                    "\nYour console command: {}",
                    shell_words::join(std::iter::once(cw_cli_path).chain(cli_args.to_cli_args()))
                );
            }
            Err(err)
        }
    }
}
