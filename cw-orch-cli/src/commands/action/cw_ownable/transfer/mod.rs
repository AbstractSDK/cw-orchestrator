use cw_orch::{
    daemon::{DaemonAsync, Wallet},
    tokio::runtime::Runtime,
};

use crate::{
    commands::action::CosmosContext,
    common::parse_expiration,
    types::{CliAddress, CliExpiration, CliSkippable},
};

use super::ContractExecuteMsg;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = TransferOwnershipOutput)]
pub struct TransferOwnership {
    /// Contract Address or alias from address-book
    contract: CliAddress,
    /// New owner Address or alias from address-book
    new_owner: CliAddress,
    /// Expiration
    #[interactive_clap(skip_default_input_arg)]
    expiration: CliExpiration,
    /// Signer id
    // TODO: should be possible to sign it from the seed phrase
    signer: String,
    /// New owner signer id, leave empty to skip auto-claim
    new_signer: CliSkippable<String>,
}

impl TransferOwnership {
    fn input_expiration(_: &CosmosContext) -> color_eyre::eyre::Result<Option<CliExpiration>> {
        let expiration = parse_expiration()?;
        Ok(Some(CliExpiration(expiration)))
    }
}

pub struct TransferOwnershipOutput;

impl TransferOwnershipOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<TransferOwnership as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let contract = scope.contract.clone().account_id(chain.chain_info())?;
        let new_owner = scope.new_owner.clone().account_id(chain.chain_info())?;

        let sender_seed = crate::common::seed_phrase_for_id(&scope.signer)?;
        let receiver_seed = scope
            .new_signer
            .0
            .as_deref()
            .map(crate::common::seed_phrase_for_id)
            .transpose()?;
        let action = cw_ownable::Action::TransferOwnership {
            new_owner: new_owner.to_string(),
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
                contract: contract.clone(),
                msg,
                funds: vec![],
            };

            let _res = daemon.sender.commit_tx(vec![exec_msg], None).await?;

            // TODO: logging

            if let Some(seed) = receiver_seed {
                let receiver_sender =
                    cw_orch::daemon::sender::Sender::from_mnemonic(&daemon.state, &seed)?;
                daemon.set_sender(&Wallet::new(receiver_sender));
                let action = cw_ownable::Action::AcceptOwnership {};
                let msg = serde_json::to_vec(&ContractExecuteMsg::UpdateOwnership(action))?;
                let exec_msg = cosmrs::cosmwasm::MsgExecuteContract {
                    sender: daemon.sender.pub_addr()?,
                    contract,
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
