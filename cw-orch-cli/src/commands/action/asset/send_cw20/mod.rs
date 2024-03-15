use cosmwasm_std::Uint128;
use cw_orch::{
    daemon::{CosmTxResponse, DaemonAsync},
    tokio::runtime::Runtime,
};

use crate::{
    log::LogOutput,
    types::{keys::seed_phrase_for_id, CliAddress},
};

use super::CosmosContext;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = SendCw20Output)]
pub struct Cw20TransferCommands {
    /// Cw20 Address or alias from address-book
    cw20_address: CliAddress,
    /// Cw20 Amount
    amount: u128,
    /// Recipient address or alias from address-book
    to_address: CliAddress,
    #[interactive_clap(skip_default_input_arg)]
    signer: String,
}

impl Cw20TransferCommands {
    fn input_signer(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<String>> {
        crate::common::select_signer()
    }
}

pub struct SendCw20Output;

impl SendCw20Output {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope: &<Cw20TransferCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let to_address_account_id = scope
            .to_address
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;
        let cw20_account_id = scope
            .cw20_address
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;

        let seed = seed_phrase_for_id(&scope.signer)?;
        let cw20_msg = cw20::Cw20ExecuteMsg::Transfer {
            recipient: to_address_account_id.to_string(),
            amount: Uint128::new(scope.amount),
        };
        let msg = serde_json::to_vec(&cw20_msg)?;
        let rt = Runtime::new()?;

        let resp = rt.block_on(async {
            let daemon = DaemonAsync::builder()
                .chain(chain)
                .mnemonic(seed)
                .build()
                .await?;

            let exec_msg = cosmrs::cosmwasm::MsgExecuteContract {
                sender: daemon.sender.pub_addr()?,
                contract: cw20_account_id,
                msg,
                funds: vec![],
            };
            let resp = daemon.sender.commit_tx(vec![exec_msg], None).await?;

            color_eyre::Result::<CosmTxResponse, color_eyre::Report>::Ok(resp)
        })?;

        resp.log();

        Ok(SendCw20Output)
    }
}
