use crate::{log::LogOutput, types::keys::seed_phrase_for_id};

use super::Cw3Context;

use cw3::ProposalListResponse;
use cw_orch::prelude::*;

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = Cw3Context)]
#[interactive_clap(output_context = Cw3VoteOutput)]
pub struct VoteOnProposal {
    #[interactive_clap(skip_default_input_arg)]
    signer: String,
}

impl VoteOnProposal {
    fn input_signer(_context: &Cw3Context) -> color_eyre::eyre::Result<Option<String>> {
        crate::common::select_signer()
    }
}

pub struct Cw3VoteOutput;

impl Cw3VoteOutput {
    fn from_previous_context(
        previous_context: Cw3Context,
        scope:&<VoteOnProposal as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        let chain = previous_context.chain;

        let contract_addr = Addr::unchecked(previous_context.cw3_address);

        let seed = seed_phrase_for_id(&scope.signer)?;
        let daemon = chain.daemon(seed)?;
        let voter_addr = daemon.sender_addr().to_string();
        // Check if we can vote
        let voter: cw3::VoterResponse = daemon.query(
            &cw3::Cw3QueryMsg::Voter {
                address: voter_addr.clone(),
            },
            &contract_addr,
        )?;
        if voter.weight.is_none() {
            color_eyre::eyre::bail!("No voting power");
        }

        // TODO: pagination
        let proposal_list: ProposalListResponse = daemon.query(
            &cw3::Cw3QueryMsg::ReverseProposals {
                start_before: None,
                limit: None,
            },
            &contract_addr,
        )?;
        let mut proposals_vote = vec![];
        for proposal in proposal_list.proposals {
            match proposal.status {
                cw3::Status::Pending | cw3::Status::Open => (),
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
            proposals_vote.push(Cw3ProposalCli {
                proposal,
                vote: vote.vote,
            });
        }
        println!("here");
        let proposal = inquire::Select::new("Select proposal", proposals_vote).prompt()?;
        println!("not_here");

        let approved_description = inquire::Confirm::new("Do you agree with description?")
            .with_help_message(&proposal.proposal.description)
            .prompt()?;
        let mut approved = approved_description;
        if approved {
            for action in proposal.proposal.msgs {
                if !inquire::Confirm::new("Do you agree with this action?")
                    .with_help_message(&serde_json::to_string(&action).unwrap())
                    .prompt()?
                {
                    approved = false;
                    break;
                }
            }
        }

        let final_vote = inquire::Select::new(
            "Confirm your vote",
            vec![
                Cw3VoteCli(cw3::Vote::Yes),
                Cw3VoteCli(cw3::Vote::No),
                Cw3VoteCli(cw3::Vote::Abstain),
                Cw3VoteCli(cw3::Vote::Veto),
            ],
        )
        .with_starting_cursor(if approved { 0 } else { 1 })
        .prompt()?;

        let action = cw3::Cw3ExecuteMsg::<Empty>::Vote {
            proposal_id: proposal.proposal.id,
            vote: final_vote.0,
        };
        let resp = daemon.execute(&action, &[], &contract_addr)?;
        resp.log(chain.chain_info());

        Ok(Cw3VoteOutput)
    }
}

pub struct Cw3ProposalCli {
    pub proposal: cw3::ProposalResponse,
    pub vote: Option<cw3::VoteInfo>,
}

impl std::fmt::Display for Cw3ProposalCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self.proposal.status {
            cw3::Status::Pending => "PENDING",
            cw3::Status::Open => "OPEN",
            cw3::Status::Rejected => "REJECTED",
            cw3::Status::Passed => "PASSED",
            cw3::Status::Executed => "EXECUTED",
        };
        let title = self.proposal.title.as_str();
        let vote = match &self.vote {
            Some(v) => match v.vote {
                cw3::Vote::Yes => " [YES]",
                cw3::Vote::No => " [NO]",
                cw3::Vote::Abstain => " [ABSTAIN]",
                cw3::Vote::Veto => " [VETO]",
            },
            None => "",
        };
        write!(f, "[{status}] {title}{vote}")
    }
}

pub struct Cw3VoteCli(cw3::Vote);

impl std::fmt::Display for Cw3VoteCli {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vote = match self.0 {
            cw3::Vote::Yes => "Yes",
            cw3::Vote::No => "No",
            cw3::Vote::Abstain => "Abstain",
            cw3::Vote::Veto => "Veto",
        };
        write!(f, "{vote}")
    }
}
