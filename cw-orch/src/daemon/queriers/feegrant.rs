use crate::{daemon::cosmos_modules, DaemonError};
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use tonic::transport::Channel;

use super::DaemonQuerier;

/// Querier for the Cosmos Gov module
pub struct Feegrant {
    channel: Channel,
}

impl DaemonQuerier for Feegrant {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Feegrant {
    /// Query all allowances granted to the grantee address by a granter address
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

    /// Query all allowances for grantee address
    pub async fn allowances(
        &self,
        grantee: impl Into<String>,
    ) -> Result<Vec<cosmos_modules::feegrant::Grant>, DaemonError> {
        self.allowances_with_pagination(grantee, None).await
    }

    /// Query allowances for grantee address with a given pagination
    pub async fn allowances_with_pagination(
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
