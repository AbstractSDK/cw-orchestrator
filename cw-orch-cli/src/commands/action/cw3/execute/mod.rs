use crate::{log::LogOutput, types::keys::seed_phrase_for_id};

use super::{Cw3Context, Cw3ProposalCli};

use cw3::ProposalListResponse;
use cw_orch::prelude::*;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = Cw3Context)]
#[interactive_clap(output_context = Cw3ExecuteOutput)]
pub struct ExecuteProposal {
    #[interactive_clap(skip_default_input_arg)]
    signer: String,
}

impl ExecuteProposal {
    fn input_signer(_context: &Cw3Context) -> color_eyre::eyre::Result<Option<String>> {
        crate::common::select_signer()
    }
}

pub struct Cw3ExecuteOutput;

impl Cw3ExecuteOutput {
    fn from_previous_context(
        previous_context: Cw3Context,
        scope:&<ExecuteProposal as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;

        let contract_addr = Addr::unchecked(previous_context.cw3_address);

        let seed = seed_phrase_for_id(&scope.signer)?;
        let daemon = chain.daemon(seed)?;
        let voter_addr = daemon.sender_addr().to_string();

        // TODO: pagination
        let proposal_list: ProposalListResponse = daemon.query(
            &cw3::Cw3QueryMsg::ReverseProposals {
                start_before: None,
                limit: None,
            },
            &contract_addr,
        )?;
        let mut proposals_to_execute = vec![];
        for proposal in proposal_list.proposals {
            match proposal.status {
                cw3::Status::Passed => (),
                // Skip if we can't vote
                _ => continue,
            };
            // Check if we voted already
            let vote: cw3::VoteResponse = daemon.query(
                &cw3::Cw3QueryMsg::Vote {
                    proposal_id: proposal.id,
                    voter: voter_addr.clone(),
                },
                &contract_addr,
            )?;
            proposals_to_execute.push(Cw3ProposalCli {
                proposal,
                vote: vote.vote,
            });
        }
        let proposal = inquire::Select::new("Select proposal", proposals_to_execute).prompt()?;

        let whole_proposal = serde_json::to_string_pretty(&proposal.proposal)?;
        println!("{whole_proposal}");

        let confirmed = inquire::Confirm::new("Do you want to execute this proposal?").prompt()?;

        if confirmed {
            let action = cw3::Cw3ExecuteMsg::<Empty>::Execute {
                proposal_id: proposal.proposal.id,
            };
            let resp = daemon.execute(&action, &[], &contract_addr)?;
            resp.log(chain.chain_info());
        }

        Ok(Cw3ExecuteOutput)
    }
}
