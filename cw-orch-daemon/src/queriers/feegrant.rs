use crate::{cosmos_modules, error::DaemonError, Daemon};
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use cw_orch_core::environment::{Querier, QuerierGetter};
use tonic::transport::Channel;

/// Querier for the Cosmos Gov module
/// All the async function are prefixed with `_`
pub struct FeegrantQuerier {
    channel: Channel,
}

impl FeegrantQuerier {
    pub fn new_async(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Querier for FeegrantQuerier {
    type Error = DaemonError;
}

impl QuerierGetter<FeegrantQuerier> for Daemon {
    fn querier(&self) -> FeegrantQuerier {
        FeegrantQuerier::new_async(self.channel())
    }
}

impl FeegrantQuerier {
    /// Query all allowances granted to the grantee address by a granter address
    pub async fn _allowance(
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

    /// Query allowances for grantee address with a given pagination
    ///
    /// see [PageRequest] for pagination
    pub async fn _allowances(
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
