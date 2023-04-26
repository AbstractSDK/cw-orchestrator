use crate::{daemon::cosmos_modules, DaemonError};
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use tonic::transport::Channel;

use super::DaemonQuerier;

/// Querier for the CosmWasm Gov module
pub struct Gov {
    channel: Channel,
}

impl DaemonQuerier for Gov {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Gov {
    /// Query proposal details by proposal id
    pub async fn proposal(
        &self,
        proposal_id: u64,
    ) -> Result<cosmos_modules::gov::QueryProposalResponse, DaemonError> {
        let proposal: cosmos_modules::gov::QueryProposalResponse = cosmos_query!(
            self,
            gov,
            proposal,
            QueryProposalRequest {
                proposal_id: proposal_id,
            }
        );
        Ok(proposal)
    }

    /// Proposals queries all proposals based on given status.
    pub async fn proposals(
        &self,
        proposal_status: i32,
        voter: impl Into<String>,
        depositor: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::gov::QueryProposalsResponse, DaemonError> {
        let proposals: cosmos_modules::gov::QueryProposalsResponse = cosmos_query!(
            self,
            gov,
            proposals,
            QueryProposalsRequest {
                proposal_status: proposal_status,
                voter: voter.into(),
                depositor: depositor.into(),
                pagination: pagination
            }
        );
        Ok(proposals)
    }

    /// Vote queries voted information based on proposalID, voterAddr.
    pub async fn vote(
        &self,
        proposal_id: u64,
        voter: impl Into<String>,
    ) -> Result<cosmos_modules::gov::QueryVoteResponse, DaemonError> {
        let vote: cosmos_modules::gov::QueryVoteResponse = cosmos_query!(
            self,
            gov,
            vote,
            QueryVoteRequest {
                proposal_id: proposal_id,
                voter: voter.into()
            }
        );
        Ok(vote)
    }

    /// Votes queries votes of a given proposal.
    pub async fn votes(
        &self,
        proposal_id: u64,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::gov::QueryVotesResponse, DaemonError> {
        let votes: cosmos_modules::gov::QueryVotesResponse = cosmos_query!(
            self,
            gov,
            votes,
            QueryVotesRequest {
                proposal_id: proposal_id,
                pagination: pagination
            }
        );
        Ok(votes)
    }

    /// Params queries all parameters of the gov module.
    pub async fn params(
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

    /// Deposit queries single deposit information based proposalID, depositAddr.
    pub async fn deposit(
        &self,
        proposal_id: u64,
        depositor: impl Into<String>,
    ) -> Result<cosmos_modules::gov::QueryDepositResponse, DaemonError> {
        let deposit: cosmos_modules::gov::QueryDepositResponse = cosmos_query!(
            self,
            gov,
            deposit,
            QueryDepositRequest {
                proposal_id: proposal_id,
                depositor: depositor.into()
            }
        );
        Ok(deposit)
    }

    /// Deposits queries all deposits of a single proposal.
    pub async fn deposits(
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
    pub async fn tally_result(
        &mut self,
        proposal_id: u64,
    ) -> Result<cosmos_modules::gov::QueryTallyResultResponse, DaemonError> {
        let tally_result: cosmos_modules::gov::QueryTallyResultResponse = cosmos_query!(
            self,
            gov,
            tally_result,
            QueryTallyResultRequest {
                proposal_id: proposal_id,
            }
        );
        Ok(tally_result)
    }
}
