use crate::{daemon::cosmos_modules, DaemonError};
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use tonic::transport::Channel;

use super::DaemonQuerier;

/// Querier for the CosmWasm Staking module
pub struct Staking {
    channel: Channel,
}

impl DaemonQuerier for Staking {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Staking {
    /// Validator queries validator info for given validator address.
    pub async fn validator(
        &self,
        validator_addr: impl Into<String>,
    ) -> Result<cosmos_modules::staking::Validator, DaemonError> {
        let validator: cosmos_modules::staking::QueryValidatorResponse = cosmos_query!(
            self,
            staking,
            validator,
            QueryValidatorRequest {
                validator_addr: validator_addr.into()
            }
        );
        Ok(validator.validator.unwrap())
    }

    /// Validators queries all validators that match the given status.
    pub async fn validators(
        &self,
        status: impl Into<String>,
    ) -> Result<Vec<cosmos_modules::staking::Validator>, DaemonError> {
        let validators: cosmos_modules::staking::QueryValidatorsResponse = cosmos_query!(
            self,
            staking,
            validators,
            QueryValidatorsRequest {
                status: status.into(),
                pagination: None,
            }
        );
        Ok(validators.validators)
    }

    /// ValidatorDelegations queries delegate info for given validator.
    pub async fn validator_delegations(
        &self,
        validator_addr: impl Into<String>,
    ) -> Result<Vec<cosmos_modules::staking::DelegationResponse>, DaemonError> {
        let validator_delegations: cosmos_modules::staking::QueryValidatorDelegationsResponse = cosmos_query!(
            self,
            staking,
            validator_delegations,
            QueryValidatorDelegationsRequest {
                validator_addr: validator_addr.into(),
                pagination: None
            }
        );
        Ok(validator_delegations.delegation_responses)
    }

    /// ValidatorUnbondingDelegations queries unbonding delegations of a validator.
    pub async fn validator_unbonding_delegations(
        &self,
        validator_addr: impl Into<String>,
    ) -> Result<Vec<cosmos_modules::staking::UnbondingDelegation>, DaemonError> {
        let validator_unbonding_delegations: cosmos_modules::staking::QueryValidatorUnbondingDelegationsResponse = cosmos_query!(
            self,
            staking,
            validator_unbonding_delegations,
            QueryValidatorUnbondingDelegationsRequest {
                validator_addr: validator_addr.into(),
                pagination: None
            }
        );
        Ok(validator_unbonding_delegations.unbonding_responses)
    }

    /// Delegation queries delegate info for given validator delegator pair.
    pub async fn delegation(
        &self,
        validator_addr: impl Into<String>,
        delegator_addr: impl Into<String>,
    ) -> Result<cosmos_modules::staking::DelegationResponse, DaemonError> {
        let delegation: cosmos_modules::staking::QueryDelegationResponse = cosmos_query!(
            self,
            staking,
            delegation,
            QueryDelegationRequest {
                validator_addr: validator_addr.into(),
                delegator_addr: delegator_addr.into()
            }
        );
        Ok(delegation.delegation_response.unwrap())
    }

    /// UnbondingDelegation queries unbonding info for given validator delegator pair.
    pub async fn unbonding_delegation(
        &self,
        validator_addr: impl Into<String>,
        delegator_addr: impl Into<String>,
    ) -> Result<cosmos_modules::staking::UnbondingDelegation, DaemonError> {
        let unbonding_delegation: cosmos_modules::staking::QueryUnbondingDelegationResponse = cosmos_query!(
            self,
            staking,
            unbonding_delegation,
            QueryUnbondingDelegationRequest {
                validator_addr: validator_addr.into(),
                delegator_addr: delegator_addr.into()
            }
        );
        Ok(unbonding_delegation.unbond.unwrap())
    }

    /// DelegatorDelegations queries all delegations of a given delegator address.
    pub async fn delegator_delegations(
        &self,
        delegator_addr: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::staking::QueryDelegatorDelegationsResponse, DaemonError> {
        let delegator_delegations: cosmos_modules::staking::QueryDelegatorDelegationsResponse = cosmos_query!(
            self,
            staking,
            delegator_delegations,
            QueryDelegatorDelegationsRequest {
                delegator_addr: delegator_addr.into(),
                pagination: pagination
            }
        );
        Ok(delegator_delegations)
    }

    /// Queries all unbonding delegations of a given delegator address.
    pub async fn delegator_unbonding_delegations(
        &self,
        delegator_addr: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::staking::QueryDelegatorUnbondingDelegationsResponse, DaemonError>
    {
        let delegator_unbonding_delegations: cosmos_modules::staking::QueryDelegatorUnbondingDelegationsResponse = cosmos_query!(
            self,
            staking,
            delegator_unbonding_delegations,
            QueryDelegatorUnbondingDelegationsRequest {
                delegator_addr: delegator_addr.into(),
                pagination: pagination
            }
        );
        Ok(delegator_unbonding_delegations)
    }

    /// Redelegations queries redelegations of given address.
    pub async fn redelegations(
        &self,
        delegator_addr: impl Into<String>,
        src_validator_addr: impl Into<String>,
        dst_validator_addr: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::staking::QueryRedelegationsResponse, DaemonError> {
        let redelegations: cosmos_modules::staking::QueryRedelegationsResponse = cosmos_query!(
            self,
            staking,
            redelegations,
            QueryRedelegationsRequest {
                delegator_addr: delegator_addr.into(),
                src_validator_addr: src_validator_addr.into(),
                dst_validator_addr: dst_validator_addr.into(),
                pagination: pagination
            }
        );
        Ok(redelegations)
    }

    /// DelegatorValidators queries all validators info for given delegator address.
    pub async fn delegator_validator(
        &self,
        validator_addr: impl Into<String>,
        delegator_addr: impl Into<String>,
    ) -> Result<cosmos_modules::staking::QueryDelegatorValidatorResponse, DaemonError> {
        let delegator_validator: cosmos_modules::staking::QueryDelegatorValidatorResponse = cosmos_query!(
            self,
            staking,
            delegator_validator,
            QueryDelegatorValidatorRequest {
                validator_addr: validator_addr.into(),
                delegator_addr: delegator_addr.into(),
            }
        );
        Ok(delegator_validator)
    }

    /// DelegatorValidators queries all validators info for given delegator address.
    pub async fn delegator_validators(
        &self,
        delegator_addr: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> Result<cosmos_modules::staking::QueryDelegatorValidatorsResponse, DaemonError> {
        let delegator_validators: cosmos_modules::staking::QueryDelegatorValidatorsResponse = cosmos_query!(
            self,
            staking,
            delegator_validators,
            QueryDelegatorValidatorsRequest {
                delegator_addr: delegator_addr.into(),
                pagination: pagination
            }
        );
        Ok(delegator_validators)
    }

    /// HistoricalInfo queries the historical info for given height.
    pub async fn historical_info(
        &self,
        height: i64,
    ) -> Result<cosmos_modules::staking::QueryHistoricalInfoResponse, DaemonError> {
        let historical_info: cosmos_modules::staking::QueryHistoricalInfoResponse = cosmos_query!(
            self,
            staking,
            historical_info,
            QueryHistoricalInfoRequest { height: height }
        );
        Ok(historical_info)
    }

    /// Pool queries the pool info.
    pub async fn pool(&self) -> Result<cosmos_modules::staking::QueryPoolResponse, DaemonError> {
        let pool: cosmos_modules::staking::QueryPoolResponse =
            cosmos_query!(self, staking, pool, QueryPoolRequest {});
        Ok(pool)
    }

    /// Parameters queries the staking parameters.
    pub async fn params(
        &self,
    ) -> Result<cosmos_modules::staking::QueryParamsResponse, DaemonError> {
        let params: cosmos_modules::staking::QueryParamsResponse =
            cosmos_query!(self, staking, params, QueryParamsRequest {});
        Ok(params)
    }
}
