use color_eyre::eyre::{self, Context};
use cw_orch::{
    daemon::{networks::parse_network_safe, DaemonAsync},
    tokio::runtime::Runtime,
};

use crate::types::CliCoins;

use super::CosmosContext;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = TransferNativeOutput)]
pub struct NativeTransferCommands {
    #[interactive_clap(skip_default_input_arg)]
    /// Input coins
    coins: CliCoins,
    /// Recipient
    to_address: String,
    /// Signer id
    signer: String,
}

impl NativeTransferCommands {
    fn input_coins(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<CliCoins>> {
        crate::common::parse_coins()
            .map(|c| Some(CliCoins(c)))
            .wrap_err("Bad coins input")
    }
}

pub struct TransferNativeOutput;

impl TransferNativeOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope: &<NativeTransferCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain =
            parse_network_safe(&previous_context.chain_id).map_err(|err| eyre::eyre!(err))?;
        let seed = crate::common::seed_phrase_for_id(&scope.signer)?;
        let coins: Vec<cosmrs::Coin> = (&scope.coins).try_into()?;

        let rt = Runtime::new()?;

        rt.block_on(async {
            let daemon = DaemonAsync::builder()
                .chain(chain)
                .mnemonic(seed)
                .no_warning()
                .build()
                .await?;

            let transfer_msg = cosmrs::bank::MsgSend {
                from_address: daemon.sender.pub_addr()?,
                to_address: scope.to_address.parse()?,
                amount: coins,
            };
            let _res = daemon.sender.commit_tx(vec![transfer_msg], None).await?;

            color_eyre::Result::<(), color_eyre::Report>::Ok(())
        })?;

        Ok(TransferNativeOutput)
    }
}
