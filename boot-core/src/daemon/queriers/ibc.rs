use crate::{daemon::cosmos_modules, DaemonError};
use cosmrs::proto::ibc::core::{
    client::v1::{IdentifiedClientState, QueryClientStatesResponse},
    connection::v1::IdentifiedConnection,
};
use tonic::transport::Channel;

use super::DaemonQuerier;

/// Queries the node for information
pub struct Ibc {
    channel: Channel,
}

impl DaemonQuerier for Ibc {
    fn new(channel: Channel) -> Self {
        Self { channel }
    }
}

impl Ibc {
    // ### Client queries ###

    pub async fn clients(&self) -> Result<Vec<IdentifiedClientState>, DaemonError> {
        use cosmos_modules::ibc_client::{query_client::QueryClient, QueryClientStatesRequest};
        let mut client = QueryClient::new(self.channel.clone());
        let request = QueryClientStatesRequest {
            ..Default::default()
        };
        let ibc_clients: QueryClientStatesResponse =
            client.client_states(request).await?.into_inner();

        log::debug!("ibc clients states: {:?}", ibc_clients.client_states);
        Ok(ibc_clients.client_states)
    }

    // ### Connection queries ###

    /// Query the IBC connections for a specific chain
    pub async fn connections(&self) -> Result<Vec<IdentifiedConnection>, DaemonError> {
        use cosmos_modules::ibc_connection::{
            query_client::QueryClient, QueryConnectionsRequest, QueryConnectionsResponse,
        };

        
        let mut client = QueryClient::new(self.channel.clone());
        let request = QueryConnectionsRequest {
            ..Default::default()
        };
        let ibc_connections: QueryConnectionsResponse =
            client.connections(request).await?.into_inner();

        log::debug!("ibc connections: {:?}", ibc_connections.connections);
        Ok(ibc_connections.connections)
    }

    /// Get the client for a specific connection
    pub async fn connection_client(
        &self,
        connection_id: impl Into<String>,
    ) -> Result<IdentifiedClientState, DaemonError> {
        use cosmos_modules::ibc_connection::QueryConnectionClientStateResponse;
        let connection_id = connection_id.into();

        let ibc_connection_client: QueryConnectionClientStateResponse = cosmos_query!(
            self,
            ibc_connection,
            connection_client_state,
            QueryConnectionClientStateRequest {
                connection_id: connection_id.clone()
            }
        );

        ibc_connection_client
            .identified_client_state
            .ok_or(DaemonError::ibc_err(format!(
                "error identifying client for connection {}",
                connection_id
            )))
    }
    // ### Channel queries ###
}
