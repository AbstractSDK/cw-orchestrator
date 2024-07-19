use color_eyre::eyre::Context;
use cw_orch::{daemon::Daemon, tokio::runtime::Runtime};

use crate::{
    log::LogOutput,
    types::{keys::seed_phrase_for_id, CliAddress, CliCoins},
};

use super::CosmosContext;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = SendNativeOutput)]
pub struct SendNativeCommands {
    #[interactive_clap(skip_default_input_arg)]
    /// Input coins
    coins: CliCoins,
    /// Recipient Address or alias from address-book
    to_address: CliAddress,
    #[interactive_clap(skip_default_input_arg)]
    signer: String,
}

impl SendNativeCommands {
    fn input_coins(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<CliCoins>> {
        crate::common::parse_coins()
            .map(|c| Some(CliCoins(c)))
            .wrap_err("Bad coins input")
    }

    fn input_signer(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<String>> {
        crate::common::select_signer()
    }
}

pub struct SendNativeOutput;

impl SendNativeOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope: &<SendNativeCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let to_address = scope
            .to_address
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;

        let seed = seed_phrase_for_id(&scope.signer)?;
        let coins = scope.coins.clone().into();

        let rt = Runtime::new()?;

        let daemon = Daemon::builder(chain).mnemonic(seed).build()?;

        let resp = rt.block_on(daemon.sender().bank_send(to_address.as_ref(), coins))?;
        resp.log(chain.chain_info());

        Ok(SendNativeOutput)
    }
}
