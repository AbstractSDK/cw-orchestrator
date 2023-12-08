use cw_orch::{
    daemon::{networks::parse_network, DaemonAsync},
    tokio::runtime::Runtime,
};

use crate::commands::action::CosmosContext;

use super::ContractExecuteMsg;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = RenounceOwnershipOutput)]
pub struct RenounceOwnership {
    /// Contract address
    contract_addr: String,
    /// Signer id
    // TODO: should be possible to sign it from the seed phrase
    signer: String,
}

pub struct RenounceOwnershipOutput;

impl RenounceOwnershipOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<RenounceOwnership as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = parse_network(&previous_context.chain_id).unwrap();
        let sender_seed = crate::common::seed_phrase_for_id(&scope.signer)?;
        let action = cw_ownable::Action::RenounceOwnership {};
        let msg = serde_json::to_vec(&ContractExecuteMsg::UpdateOwnership(action))?;

        let rt = Runtime::new()?;
        rt.block_on(async {
            let daemon = DaemonAsync::builder()
                .chain(chain)
                .mnemonic(sender_seed)
                .build()
                .await?;

            let exec_msg = cosmrs::cosmwasm::MsgExecuteContract {
                sender: daemon.sender.pub_addr()?,
                contract: scope.contract_addr.parse()?,
                msg,
                funds: vec![],
            };

            let _res = daemon.sender.commit_tx(vec![exec_msg], None).await?;

            color_eyre::Result::<(), color_eyre::Report>::Ok(())
        })?;

        Ok(RenounceOwnershipOutput)
    }
}
