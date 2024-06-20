use cw_orch::{
    daemon::{DaemonAsync, Wallet},
    tokio::runtime::Runtime,
};

use crate::{
    commands::action::CosmosContext,
    common::parse_expiration,
    log::LogOutput,
    types::{keys::seed_phrase_for_id, CliAddress, CliExpiration, CliSkippable},
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
    #[interactive_clap(skip_default_input_arg)]
    signer: String,
    /// New owner signer id, leave empty to skip auto-claim
    new_signer: CliSkippable<String>,
}

impl TransferOwnership {
    fn input_expiration(_: &CosmosContext) -> color_eyre::eyre::Result<Option<CliExpiration>> {
        let expiration = parse_expiration()?;
        Ok(Some(CliExpiration(expiration)))
    }

    fn input_signer(_context: &CosmosContext) -> color_eyre::eyre::Result<Option<String>> {
        crate::common::select_signer()
    }
}

pub struct TransferOwnershipOutput;

impl TransferOwnershipOutput {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<TransferOwnership as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;
        let contract = scope
            .contract
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;
        let new_owner = scope
            .new_owner
            .clone()
            .account_id(chain.chain_info(), &previous_context.global_config)?;

        let sender_seed = seed_phrase_for_id(&scope.signer)?;
        let receiver_seed = scope
            .new_signer
            .0
            .as_deref()
            .map(seed_phrase_for_id)
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

            let resp = daemon.sender.commit_tx(vec![exec_msg], None).await?;

            resp.log(chain.chain_info());
            println!("Successfully transferred ownership, waiting for approval by {new_owner}",);

            if let Some(seed) = receiver_seed {
                let receiver_sender = cw_orch::daemon::sender::Sender::from_mnemonic(
                    chain.into(),
                    daemon.channel(),
                    &seed,
                )?;
                daemon.set_sender(&Wallet::new(receiver_sender));
                let action = cw_ownable::Action::AcceptOwnership {};
                let msg = serde_json::to_vec(&ContractExecuteMsg::UpdateOwnership(action))?;
                let exec_msg = cosmrs::cosmwasm::MsgExecuteContract {
                    sender: daemon.sender.pub_addr()?,
                    contract,
                    msg,
                    funds: vec![],
                };
                let resp = daemon.sender.commit_tx(vec![exec_msg], None).await?;
                resp.log(chain.chain_info());
                println!("{new_owner} successfully accepted ownership");
            }
            color_eyre::Result::<(), color_eyre::Report>::Ok(())
        })?;

        Ok(TransferOwnershipOutput)
    }
}
