use crate::{cosmos_modules, error::DaemonError, Daemon};
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use cosmwasm_std::Addr;
use cw_orch_core::environment::{Querier, QuerierGetter};
use tokio::runtime::Handle;
use tonic::transport::Channel;

/// Querier for the Cosmos Gov module
/// All the async function are prefixed with `_`
pub struct FeeGrant {
    pub channel: Channel,
    pub rt_handle: Option<Handle>,
}

impl FeeGrant {
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

impl Querier for FeeGrant {
    type Error = DaemonError;
}

impl QuerierGetter<FeeGrant> for Daemon {
    fn querier(&self) -> FeeGrant {
        FeeGrant::new(self)
    }
}

impl FeeGrant {
    /// Query all allowances granted to the grantee address by a granter address
    pub async fn _allowance(
        &self,
        granter: &Addr,
        grantee: &Addr,
    ) -> Result<cosmos_modules::feegrant::Grant, DaemonError> {
        let allowance: cosmos_modules::feegrant::QueryAllowanceResponse = cosmos_query!(
            self,
            feegrant,
            allowance,
            QueryAllowanceRequest {
                granter: granter.to_string(),
                grantee: grantee.to_string(),
            }
        );
        Ok(allowance.allowance.unwrap())
    }

    /// Query allowances for grantee address with a given pagination
    ///
    /// see [PageRequest] for pagination
    pub async fn _allowances(
        &self,
        grantee: &Addr,
        pagination: Option<PageRequest>,
    ) -> Result<Vec<cosmos_modules::feegrant::Grant>, DaemonError> {
        let allowances: cosmos_modules::feegrant::QueryAllowancesResponse = cosmos_query!(
            self,
            feegrant,
            allowances,
            QueryAllowancesRequest {
                grantee: grantee.to_string(),
                pagination: pagination
            }
        );
        Ok(allowances.allowances)
    }
}
