use crate::{
    commands::action::CosmosContext,
    types::{CliAddress, CliLockedChain},
    GlobalConfig,
};

use strum::{EnumDiscriminants, EnumIter, EnumMessage};

mod execute;
mod vote;

#[derive(Clone)]
pub struct Cw3Context {
    pub cw3_address: String,
    pub chain: CliLockedChain,
    pub global_config: GlobalConfig,
}

impl Cw3Context {
    fn from_previous_context(
        previous_context: CosmosContext,
        scope:&<Cw3Commands as interactive_clap::ToInteractiveClapContextScope>::InteractiveClapContextScope,
    ) -> color_eyre::eyre::Result<Self> {
        Ok(Cw3Context {
            cw3_address: scope
                .cw3_address
                .clone()
                .account_id(
                    previous_context.chain.chain_info(),
                    &previous_context.global_config,
                )?
                .to_string(),
            chain: previous_context.chain,
            global_config: previous_context.global_config,
        })
    }
}

impl From<Cw3Context> for CosmosContext {
    fn from(value: Cw3Context) -> Self {
        Self {
            chain: value.chain,
            global_config: value.global_config,
        }
    }
}

#[derive(Debug, Clone, interactive_clap::InteractiveClap)]
#[interactive_clap(input_context = CosmosContext)]
#[interactive_clap(output_context = Cw3Context)]
pub struct Cw3Commands {
    /// Cw3 Address or alias from address-book
    cw3_address: CliAddress,
    #[interactive_clap(subcommand)]
    action: Cw3Action,
}

#[derive(Debug, EnumDiscriminants, Clone, interactive_clap::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = Cw3Context)]
/// Select cosmwasm action
pub enum Cw3Action {
    /// Vote on existing proposal
    #[strum_discriminants(strum(message = "🗳️ Vote on the existing proposal"))]
    Vote(vote::VoteOnProposal),
    /// Execute passed proposal
    #[strum_discriminants(strum(message = "⚡ Execute on the passed proposal"))]
    Execute(execute::ExecuteProposal),
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
