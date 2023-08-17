use crate::{cosmos_modules, error::DaemonError};
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use tonic::transport::Channel;

use super::DaemonQuerier;

/// Querier for the Cosmos Staking module
pub struct Staking {
    channel: Channel,
}

impl DaemonQuerier for Staking {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Staking {
    /// Queries validator info for given validator address
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

    /// Queries all validators that match the given status
    ///
    /// see [StakingBondStatus] for available statuses
    pub async fn validators(
        &self,
        status: StakingBondStatus,
    ) -> Result<Vec<cosmos_modules::staking::Validator>, DaemonError> {
        let validators: cosmos_modules::staking::QueryValidatorsResponse = cosmos_query!(
            self,
            staking,
            validators,
            QueryValidatorsRequest {
                status: status.to_string(),
                pagination: None,
            }
        );
        Ok(validators.validators)
    }

    /// Query validator delegations info for given validator
    ///
    /// see [PageRequest] for pagination
    pub async fn delegations(
        &self,
        validator_addr: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> Result<Vec<cosmos_modules::staking::DelegationResponse>, DaemonError> {
        let validator_delegations: cosmos_modules::staking::QueryValidatorDelegationsResponse = cosmos_query!(
            self,
            staking,
            validator_delegations,
            QueryValidatorDelegationsRequest {
                validator_addr: validator_addr.into(),
                pagination: pagination
            }
        );
        Ok(validator_delegations.delegation_responses)
    }

    /// Query validator unbonding delegations of a validator
    pub async fn unbonding_delegations(
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

    /// Query delegation info for given validator for a delegator
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

    /// Query unbonding delegation info for given validator delegator
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

    /// Query all delegator delegations of a given delegator address
    ///
    /// see [PageRequest] for pagination
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
    ///
    /// see [PageRequest] for pagination
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

    /// Query redelegations of a given address
    ///
    /// see [PageRequest] for pagination
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

    /// Query delegator validators info for given delegator address.
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

    /// Query delegator validators info for given delegator address
    ///
    /// see [PageRequest] for pagination
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

    /// Query historical info info for given height
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

    /// Query the pool info
    pub async fn pool(&self) -> Result<cosmos_modules::staking::QueryPoolResponse, DaemonError> {
        let pool: cosmos_modules::staking::QueryPoolResponse =
            cosmos_query!(self, staking, pool, QueryPoolRequest {});
        Ok(pool)
    }

    /// Query staking parameters
    pub async fn params(
        &self,
    ) -> Result<cosmos_modules::staking::QueryParamsResponse, DaemonError> {
        let params: cosmos_modules::staking::QueryParamsResponse =
            cosmos_query!(self, staking, params, QueryParamsRequest {});
        Ok(params)
    }
}

/// Staking bond statuses
pub enum StakingBondStatus {
    /// UNSPECIFIED defines an invalid validator status.
    Unspecified = 0,
    /// UNBONDED defines a validator that is not bonded.
    Unbonded = 1,
    /// UNBONDING defines a validator that is unbonding.
    Unbonding = 2,
    /// BONDED defines a validator that is bonded.
    Bonded = 3,
}

impl ToString for StakingBondStatus {
    /// Convert to string
    fn to_string(&self) -> String {
        match self {
            StakingBondStatus::Unspecified => "BOND_STATUS_UNSPECIFIED".to_string(),
            StakingBondStatus::Unbonded => "BOND_STATUS_UNBONDED".to_string(),
            StakingBondStatus::Unbonding => "BOND_STATUS_UNBONDING".to_string(),
            StakingBondStatus::Bonded => "BOND_STATUS_BONDED".to_string(),
        }
    }
}
