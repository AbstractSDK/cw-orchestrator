use color_eyre::eyre;
use cosmwasm_std::Uint128;
use cw_orch::{
    daemon::{networks::parse_network_safe, DaemonAsync},
    tokio::runtime::Runtime,
};

use super::CosmosContext;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = TransferCw20Output)]
pub struct Cw20TransferCommands {
    /// Cw20 Address
    cw20_address: String,
    /// Cw20 Amount
    amount: u128,
    /// Recipient
    to_address: String,
    /// Signer id
    signer: String,
}

pub struct TransferCw20Output;

impl TransferCw20Output {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope: &<Cw20TransferCommands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain =
            parse_network_safe(&previous_context.chain_id).map_err(|err| eyre::eyre!(err))?;
        let seed = crate::common::seed_phrase_for_id(&scope.signer)?;
        let cw20_msg = cw20::Cw20ExecuteMsg::Transfer {
            recipient: scope.to_address.clone(),
            amount: Uint128::new(scope.amount),
        };
        let msg = serde_json::to_vec(&cw20_msg)?;
        let rt = Runtime::new()?;

        rt.block_on(async {
            let daemon = DaemonAsync::builder()
                .chain(chain)
                .mnemonic(seed)
                .build()
                .await?;

            let exec_msg = cosmrs::cosmwasm::MsgExecuteContract {
                sender: daemon.sender.pub_addr()?,
                contract: scope.cw20_address.parse()?,
                msg,
                funds: vec![],
            };
            let _res = daemon.sender.commit_tx(vec![exec_msg], None).await?;

            color_eyre::Result::<(), color_eyre::Report>::Ok(())
        })?;

        Ok(TransferCw20Output)
    }
}
