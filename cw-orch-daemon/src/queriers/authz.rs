use cosmrs::proto::cosmos::base::query::v1beta1::PageRequest;
use tonic::transport::Channel;

use crate::{cosmos_modules, error::DaemonError};

use super::DaemonQuerier;

/// Queries for Cosmos AuthZ Module
pub struct Authz {
    channel: Channel,
}

impl DaemonQuerier for Authz {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Authz {
    /// Query Authz Grants from grantee to granter
    pub async fn grants(
        &self,
        granter: String,
        grantee: String,
        msg_type_url: String,
        pagination: Option<PageRequest>,
    ) -> Result<cosmrs::proto::cosmos::authz::v1beta1::QueryGrantsResponse, DaemonError> {
        use cosmos_modules::authz::{query_client::QueryClient, QueryGrantsRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let grants = client
            .grants(QueryGrantsRequest {
                granter,
                grantee,
                msg_type_url,
                pagination,
            })
            .await?
            .into_inner();
        Ok(grants)
    }

    /// Query Authz Grants of grantee
    pub async fn grantee_grants(
        &self,
        grantee: String,
        pagination: Option<PageRequest>,
    ) -> Result<cosmrs::proto::cosmos::authz::v1beta1::QueryGranteeGrantsResponse, DaemonError>
    {
        use cosmos_modules::authz::{query_client::QueryClient, QueryGranteeGrantsRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let grants = client
            .grantee_grants(QueryGranteeGrantsRequest {
                grantee,
                pagination,
            })
            .await?
            .into_inner();
        Ok(grants)
    }

    /// Query Authz Grants for granter
    pub async fn granter_grants(
        &self,
        granter: String,
        pagination: Option<PageRequest>,
    ) -> Result<cosmrs::proto::cosmos::authz::v1beta1::QueryGranterGrantsResponse, DaemonError>
    {
        use cosmos_modules::authz::{query_client::QueryClient, QueryGranterGrantsRequest};
        let mut client: QueryClient<Channel> = QueryClient::new(self.channel.clone());
        let grants = client
            .granter_grants(QueryGranterGrantsRequest {
                granter,
                pagination,
            })
            .await?
            .into_inner();
        Ok(grants)
    }
}
