use cw_orch::{
    daemon::{CosmTxResponse, DaemonAsync},
    tokio::runtime::Runtime,
};

use crate::{
    commands::action::CosmosContext,
    log::LogOutput,
    types::{keys::seed_phrase_for_id, CliAddress},
};

use super::ContractExecuteMsg;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = AcceptOwnershipOutput)]
pub struct AcceptOwnership {
    /// Contract Address or alias from address-book
    contract: CliAddress,
    #[interactive_clap(skip_default_input_arg)]
    signer: String,
}

impl AcceptOwnership {
    fn input_signer(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<String>> {
        crate::common::select_signer()
    }
}

pub struct AcceptOwnershipOutput;

impl AcceptOwnershipOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<AcceptOwnership as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let contract = scope
            .contract
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;

        let sender_seed = seed_phrase_for_id(&scope.signer)?;
        let action = cw_ownable::Action::AcceptOwnership {};
        let msg = serde_json::to_vec(&ContractExecuteMsg::UpdateOwnership(action))?;

        let rt = Runtime::new()?;
        let resp = rt.block_on(async {
            let daemon = DaemonAsync::builder()
                .chain(chain)
                .mnemonic(sender_seed)
                .build()
                .await?;

            let exec_msg = cosmrs::cosmwasm::MsgExecuteContract {
                sender: daemon.sender.pub_addr()?,
                contract,
                msg,
                funds: vec![],
            };

            let resp = daemon.sender.commit_tx(vec![exec_msg], None).await?;
            color_eyre::Result::<CosmTxResponse, color_eyre::Report>::Ok(resp)
        })?;
        resp.log(chain.chain_info());

        Ok(AcceptOwnershipOutput)
    }
}
