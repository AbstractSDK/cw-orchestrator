use crate::{cosmos_modules, error::DaemonError, Daemon};
use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use cosmwasm_std::Addr;
use cw_orch_core::environment::{Querier, QuerierGetter};
use tokio::runtime::Handle;
use tonic::transport::Channel;

/// Queries for Cosmos AuthZ Module
/// All the async function are prefixed with `_`
pub struct Authz {
    pub channel: Channel,
    pub rt_handle: Option<Handle>,
}

impl Authz {
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

impl Querier for Authz {
    type Error = DaemonError;
}

impl QuerierGetter<Authz> for Daemon {
    fn querier(&self) -> Authz {
        Authz::new(self)
    }
}

impl Authz {
    /// Query Authz Grants from grantee to granter
    pub async fn _grants(
        &self,
        granter: &Addr,
        grantee: &Addr,
        msg_type_url: String,
        pagination: Option<PageRequest>,
    ) -> Result<cosmrs::proto::cosmos::authz::v1beta1::QueryGrantsResponse, DaemonError> {
        use cosmos_modules::authz::{query_client::QueryClient, QueryGrantsRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let grants = client
            .grants(QueryGrantsRequest {
                granter: granter.to_string(),
                grantee: grantee.to_string(),
                msg_type_url,
                pagination,
            })
            .await?
            .into_inner();
        Ok(grants)
    }

    /// Query Authz Grants of grantee
    pub async fn _grantee_grants(
        &self,
        grantee: &Addr,
        pagination: Option<PageRequest>,
    ) -> Result<cosmrs::proto::cosmos::authz::v1beta1::QueryGranteeGrantsResponse, DaemonError>
    {
        use cosmos_modules::authz::{query_client::QueryClient, QueryGranteeGrantsRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let grants = client
            .grantee_grants(QueryGranteeGrantsRequest {
                grantee: grantee.to_string(),
                pagination,
            })
            .await?
            .into_inner();
        Ok(grants)
    }

    /// Query Authz Grants for granter
    pub async fn _granter_grants(
        &self,
        granter: &Addr,
        pagination: Option<PageRequest>,
    ) -> Result<cosmrs::proto::cosmos::authz::v1beta1::QueryGranterGrantsResponse, DaemonError>
    {
        use cosmos_modules::authz::{query_client::QueryClient, QueryGranterGrantsRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let grants = client
            .granter_grants(QueryGranterGrantsRequest {
                granter: granter.to_string(),
                pagination,
            })
            .await?
            .into_inner();
        Ok(grants)
    }
}
