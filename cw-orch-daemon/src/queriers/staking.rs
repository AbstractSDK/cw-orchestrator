use std::fmt::Display;

use crate::Channel;
use crate::{cosmos_modules, error::DaemonError, Daemon};
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use cosmwasm_std::{Addr, StdError};
use cw_orch_core::environment::{Querier, QuerierGetter};
use tokio::runtime::Handle;

use super::bank::cosmrs_to_cosmwasm_coin;

/// Querier for the Cosmos Staking module
/// All the async function are prefixed with `_`
pub struct Staking {
    pub channel: Channel,
    pub rt_handle: Option<Handle>,
}

impl Staking {
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

impl Querier for Staking {
    type Error = DaemonError;
}

impl QuerierGetter<Staking> for Daemon {
    fn querier(&self) -> Staking {
        Staking::new(self)
    }
}
impl Staking {
    /// Queries validator info for given validator address
    pub async fn _validator(
        &self,
        validator_addr: &Addr,
    ) -> Result<cosmwasm_std::Validator, DaemonError> {
        let validator_response: cosmos_modules::staking::QueryValidatorResponse = cosmos_query!(
            self,
            staking,
            validator,
            QueryValidatorRequest {
                validator_addr: validator_addr.into()
            }
        );

        Ok(cosmrs_to_cosmwasm_validator(
            validator_response.validator.unwrap(),
        )?)
    }

    /// Queries all validators that match the given status
    ///
    /// see [StakingBondStatus] for available statuses
    pub async fn _validators(
        &self,
        status: StakingBondStatus,
    ) -> Result<Vec<cosmwasm_std::Validator>, DaemonError> {
        let validators: cosmos_modules::staking::QueryValidatorsResponse = cosmos_query!(
            self,
            staking,
            validators,
            QueryValidatorsRequest {
                status: status.to_string(),
                pagination: None,
            }
        );

        Ok(validators
            .validators
            .into_iter()
            .map(cosmrs_to_cosmwasm_validator)
            .collect::<Result<_, _>>()?)
    }

    /// Query validator delegations info for given validator
    ///
    /// see [PageRequest] for pagination
    pub async fn _delegations(
        &self,
        validator_addr: &Addr,
        pagination: Option<PageRequest>,
    ) -> Result<Vec<cosmwasm_std::Delegation>, DaemonError> {
        let validator_delegations: cosmos_modules::staking::QueryValidatorDelegationsResponse = cosmos_query!(
            self,
            staking,
            validator_delegations,
            QueryValidatorDelegationsRequest {
                validator_addr: validator_addr.into(),
                pagination: pagination
            }
        );
        Ok(validator_delegations
            .delegation_responses
            .into_iter()
            .map(cosmrs_to_cosmwasm_delegation)
            .collect::<Result<_, _>>()?)
    }

    /// Query validator unbonding delegations of a validator
    pub async fn _unbonding_delegations(
        &self,
        validator_addr: &Addr,
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
    pub async fn _delegation(
        &self,
        validator_addr: &Addr,
        delegator_addr: &Addr,
    ) -> Result<cosmwasm_std::Delegation, DaemonError> {
        let delegation: cosmos_modules::staking::QueryDelegationResponse = cosmos_query!(
            self,
            staking,
            delegation,
            QueryDelegationRequest {
                validator_addr: validator_addr.into(),
                delegator_addr: delegator_addr.into()
            }
        );
        Ok(cosmrs_to_cosmwasm_delegation(
            delegation.delegation_response.unwrap(),
        )?)
    }

    /// Query unbonding delegation info for given validator delegator
    pub async fn _unbonding_delegation(
        &self,
        validator_addr: &Addr,
        delegator_addr: &Addr,
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
    pub async fn _delegator_delegations(
        &self,
        delegator_addr: &Addr,
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
    pub async fn _delegator_unbonding_delegations(
        &self,
        delegator_addr: &Addr,
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
    pub async fn _redelegations(
        &self,
        delegator_addr: &Addr,
        src_validator_addr: &Addr,
        dst_validator_addr: &Addr,
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
    pub async fn _delegator_validator(
        &self,
        validator_addr: &Addr,
        delegator_addr: &Addr,
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
    pub async fn _delegator_validators(
        &self,
        delegator_addr: &Addr,
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
    pub async fn _historical_info(
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
    pub async fn _pool(&self) -> Result<cosmos_modules::staking::QueryPoolResponse, DaemonError> {
        let pool: cosmos_modules::staking::QueryPoolResponse =
            cosmos_query!(self, staking, pool, QueryPoolRequest {});
        Ok(pool)
    }

    /// Query staking parameters
    pub async fn _params(
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

impl Display for StakingBondStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            StakingBondStatus::Unspecified => "BOND_STATUS_UNSPECIFIED",
            StakingBondStatus::Unbonded => "BOND_STATUS_UNBONDED",
            StakingBondStatus::Unbonding => "BOND_STATUS_UNBONDING",
            StakingBondStatus::Bonded => "BOND_STATUS_BONDED",
        };
        write!(f, "{}", str)
    }
}

pub fn cosmrs_to_cosmwasm_validator(
    validator: cosmrs::proto::cosmos::staking::v1beta1::Validator,
) -> Result<cosmwasm_std::Validator, StdError> {
    let comission = validator.commission.unwrap().commission_rates.unwrap();
    Ok(cosmwasm_std::Validator::new(
        validator.operator_address,
        comission.rate.parse()?,
        comission.max_rate.parse()?,
        comission.max_change_rate.parse()?,
    ))
}

pub fn cosmrs_to_cosmwasm_delegation(
    delegation_response: cosmrs::proto::cosmos::staking::v1beta1::DelegationResponse,
) -> Result<cosmwasm_std::Delegation, StdError> {
    let delegation = delegation_response.delegation.unwrap();
    Ok(cosmwasm_std::Delegation::new(
        Addr::unchecked(delegation.delegator_address),
        delegation.validator_address,
        cosmrs_to_cosmwasm_coin(delegation_response.balance.unwrap())?,
    ))
}
