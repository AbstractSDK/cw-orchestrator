use crate::{daemon::cosmos_modules, DaemonError};
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use tonic::transport::Channel;

use super::DaemonQuerier;

/// Querier for the CosmWasm Gov module
pub struct Feegrant {
    channel: Channel,
}

impl DaemonQuerier for Feegrant {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Feegrant {
    /// Allowance returns fee granted to the grantee by the granter.
    pub async fn allowance(
        &self,
        granter: impl Into<String>,
        grantee: impl Into<String>,
    ) -> Result<cosmos_modules::feegrant::Grant, DaemonError> {
        let allowance: cosmos_modules::feegrant::QueryAllowanceResponse = cosmos_query!(
            self,
            feegrant,
            allowance,
            QueryAllowanceRequest {
                granter: granter.into(),
                grantee: grantee.into(),
            }
        );
        Ok(allowance.allowance.unwrap())
    }

    /// Allowances returns all the grants for address.
    pub async fn allowances(
        &self,
        grantee: impl Into<String>,
        pagination: Option<PageRequest>,
    ) -> Result<Vec<cosmos_modules::feegrant::Grant>, DaemonError> {
        let allowances: cosmos_modules::feegrant::QueryAllowancesResponse = cosmos_query!(
            self,
            feegrant,
            allowances,
            QueryAllowancesRequest {
                grantee: grantee.into(),
                pagination: pagination
            }
        );
        Ok(allowances.allowances)
    }
}
