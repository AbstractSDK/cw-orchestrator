use std::rc::Rc;

use cw_orch::{
    daemon::{networks::parse_network, DaemonAsync},
    tokio::runtime::Runtime,
};

use crate::{
    commands::action::CosmosContext,
    common::parse_expiration,
    types::{CliExpiration, CliSkippable},
};

use super::ContractExecuteMsg;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = TransferOwnershipOutput)]
pub struct TransferOwnership {
    /// Contract address
    contract_addr: String,
    /// New owner address
    new_owner: String,
    /// Expiration
    #[interactive_clap(skip_default_input_arg)]
    expiration: CliExpiration,
    /// Signer id
    // TODO: should be possible to sign it from the seed phrase
    signer: String,
    /// New owner signer id
    #[interactive_clap(skip_default_input_arg)]
    new_signer: CliSkippable<String>,
}

impl TransferOwnership {
    fn input_expiration(_: &CosmosContext) -> color_eyre::eyre::Result<Option<CliExpiration>> {
        let expiration = parse_expiration()?;
        Ok(Some(CliExpiration(expiration)))
    }

    fn input_new_signer(
        _: &CosmosContext,
    ) -> color_eyre::eyre::Result<Option<CliSkippable<String>>> {
        let new_signer = inquire::Text::new("New signer id")
            .with_help_message("Press ESC to skip auto-claim")
            .prompt_skippable()?;
        Ok(Some(CliSkippable(new_signer)))
    }
}

pub struct TransferOwnershipOutput;

impl TransferOwnershipOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<TransferOwnership as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = parse_network(&previous_context.chain_id);
        let sender_seed = crate::common::seed_phrase_for_id(&scope.signer)?;
        let receiver_seed = scope
            .new_signer
            .0
            .as_deref()
            .map(|new_signer| crate::common::seed_phrase_for_id(new_signer))
            .transpose()?;
        let action = cw_ownable::Action::TransferOwnership {
            new_owner: scope.new_owner.clone(),
            expiry: Some(scope.expiration.0),
        };
        let msg = serde_json::to_vec(&ContractExecuteMsg::UpdateOwnership(action))?;

        let rt = Runtime::new()?;
        rt.block_on(async {
            let mut daemon = DaemonAsync::builder()
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

            if let Some(seed) = receiver_seed {
                let receiver_sender =
                    cw_orch::daemon::sender::Sender::from_mnemonic(&daemon.state, &seed)?;
                daemon.set_sender(&Rc::new(receiver_sender));
                let action = cw_ownable::Action::AcceptOwnership {};
                let msg = serde_json::to_vec(&ContractExecuteMsg::UpdateOwnership(action))?;
                let exec_msg = cosmrs::cosmwasm::MsgExecuteContract {
                    sender: daemon.sender.pub_addr()?,
                    contract: scope.contract_addr.parse()?,
                    msg,
                    funds: vec![],
                };
                let _res = daemon.sender.commit_tx(vec![exec_msg], None).await?;
            }
            color_eyre::Result::<(), color_eyre::Report>::Ok(())
        })?;

        Ok(TransferOwnershipOutput)
    }
}
