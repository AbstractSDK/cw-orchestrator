use color_eyre::eyre::Context;
use cw_orch::{
    daemon::{CosmTxResponse, DaemonAsync},
    tokio::runtime::Runtime,
};

use crate::{log::LogOutput, types::CliCoins};

use super::CosmosContext;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = SendNativeOutput)]
pub struct SendNativeCommands {
    #[interactive_clap(skip_default_input_arg)]
    /// Input coins
    coins: CliCoins,
    /// Recipient
    to_address: String,
    /// Signer id
    signer: String,
}

impl SendNativeCommands {
    fn input_coins(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<CliCoins>> {
        crate::common::parse_coins()
            .map(|c| Some(CliCoins(c)))
            .wrap_err("Bad coins input")
    }
}

pub struct SendNativeOutput;

impl SendNativeOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope: &<SendNativeCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let seed = crate::common::seed_phrase_for_id(&scope.signer)?;
        let coins: Vec<cosmrs::Coin> = (&scope.coins).try_into()?;

        let rt = Runtime::new()?;

        let resp = rt.block_on(async {
            let daemon = DaemonAsync::builder()
                .chain(chain)
                .mnemonic(seed)
                .build()
                .await?;

            let transfer_msg = cosmrs::bank::MsgSend {
                from_address: daemon.sender.pub_addr()?,
                to_address: scope.to_address.parse()?,
                amount: coins,
            };
            let resp = daemon.sender.commit_tx(vec![transfer_msg], None).await?;

            color_eyre::Result::<CosmTxResponse, color_eyre::Report>::Ok(resp)
        })?;

        resp.log();

        Ok(SendNativeOutput)
    }
}
