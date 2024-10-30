use crate::Channel;
use crate::{cosmos_modules, error::DaemonError, Daemon};
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use cosmwasm_std::Addr;
use cw_orch_core::environment::{Querier, QuerierGetter};
use tokio::runtime::Handle;

/// Querier for the Cosmos Gov module
/// All the async function are prefixed with `_`
pub struct Gov {
    pub channel: Channel,
    pub rt_handle: Option<Handle>,
}

impl Gov {
    pub fn new(daemon: &Daemon) -> Self {
        Self {
            channel: daemon.channel(),
            rt_handle: Some(daemon.rt_handle.clone()),
        }
    }

    pub fn new_async(channel: Channel) -> Self {
        Self {
            channel,
            rt_handle: None,
        }
    }
}

impl Querier for Gov {
    type Error = DaemonError;
}

impl QuerierGetter<Gov> for Daemon {
    fn querier(&self) -> Gov {
        Gov::new(self)
    }
}

impl Gov {
    /// Query proposal details by proposal id
    pub async fn _proposal(
        &self,
        proposal_id: u64,
    ) -> Result<cosmos_modules::gov::Proposal, DaemonError> {
        let proposal: cosmos_modules::gov::QueryProposalResponse = cosmos_query!(
            self,
            gov,
            proposal,
            QueryProposalRequest {
                proposal_id: proposal_id,
            }
        );
        Ok(proposal.proposal.unwrap())
    }

    /// Query proposals based on given status
    ///
    /// see [PageRequest] for pagination
    pub async fn _proposals(
        &self,
        proposal_status: GovProposalStatus,
        voter: &Addr,
        depositor: &Addr,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::gov::QueryProposalsResponse, DaemonError> {
        let proposals: cosmos_modules::gov::QueryProposalsResponse = cosmos_query!(
            self,
            gov,
            proposals,
            QueryProposalsRequest {
                proposal_status: proposal_status as i32,
                voter: voter.to_string(),
                depositor: depositor.to_string(),
                pagination: pagination
            }
        );
        Ok(proposals)
    }

    /// Query voted information based on proposal_id for voter address
    pub async fn _vote(
        &self,
        proposal_id: u64,
        voter: &Addr,
    ) -> Result<cosmos_modules::gov::Vote, DaemonError> {
        let vote: cosmos_modules::gov::QueryVoteResponse = cosmos_query!(
            self,
            gov,
            vote,
            QueryVoteRequest {
                proposal_id: proposal_id,
                voter: voter.to_string()
            }
        );
        Ok(vote.vote.unwrap())
    }

    /// Query votes of a given proposal
    ///
    /// see [PageRequest] for pagination
    pub async fn _votes(
        &self,
        proposal_id: impl Into<u64>,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::gov::QueryVotesResponse, DaemonError> {
        let votes: cosmos_modules::gov::QueryVotesResponse = cosmos_query!(
            self,
            gov,
            votes,
            QueryVotesRequest {
                proposal_id: proposal_id.into(),
                pagination: pagination
            }
        );
        Ok(votes)
    }

    /// Query all parameters of the gov module
    pub async fn _params(
        &self,
        params_type: impl Into<String>,
    ) -> Result<cosmos_modules::gov::QueryParamsResponse, DaemonError> {
        let params: cosmos_modules::gov::QueryParamsResponse = cosmos_query!(
            self,
            gov,
            params,
            QueryParamsRequest {
                params_type: params_type.into()
            }
        );
        Ok(params)
    }

    /// Query deposit information using proposal_id and depositor address
    pub async fn _deposit(
        &self,
        proposal_id: u64,
        depositor: impl Into<String>,
    ) -> Result<cosmos_modules::gov::Deposit, DaemonError> {
        let deposit: cosmos_modules::gov::QueryDepositResponse = cosmos_query!(
            self,
            gov,
            deposit,
            QueryDepositRequest {
                proposal_id: proposal_id,
                depositor: depositor.into()
            }
        );
        Ok(deposit.deposit.unwrap())
    }

    /// Query deposits of a proposal
    ///
    /// see [PageRequest] for pagination
    pub async fn _deposits(
        &self,
        proposal_id: u64,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::gov::QueryDepositsResponse, DaemonError> {
        let deposits: cosmos_modules::gov::QueryDepositsResponse = cosmos_query!(
            self,
            gov,
            deposits,
            QueryDepositsRequest {
                proposal_id: proposal_id,
                pagination: pagination
            }
        );
        Ok(deposits)
    }

    /// TallyResult queries the tally of a proposal vote.
    pub async fn _tally_result(
        &mut self,
        proposal_id: u64,
    ) -> Result<cosmos_modules::gov::TallyResult, DaemonError> {
        let tally_result: cosmos_modules::gov::QueryTallyResultResponse = cosmos_query!(
            self,
            gov,
            tally_result,
            QueryTallyResultRequest {
                proposal_id: proposal_id,
            }
        );
        Ok(tally_result.tally.unwrap())
    }
}

/// Proposal status
#[allow(missing_docs)]
pub enum GovProposalStatus {
    Unspecified = 0,
    DepositPeriod = 1,
    VotingPeriod = 2,
    Passed = 3,
    Rejected = 4,
    Failed = 5,
}
