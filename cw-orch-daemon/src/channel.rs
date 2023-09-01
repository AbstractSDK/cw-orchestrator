use cosmrs::proto::cosmos::base::tendermint::v1beta1::{
    service_client::ServiceClient, GetNodeInfoRequest,
};
use ibc_chain_registry::chain::Grpc;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use tonic::transport::{Channel, ClientTlsConfig};

use super::error::DaemonError;

/// A helper for constructing a gRPC channel
pub struct GrpcChannel {}

impl GrpcChannel {
    /// Connect to any of the provided gRPC endpoints
    pub async fn connect(grpc: &[Grpc], chain_id: &ChainId) -> Result<Channel, DaemonError> {
        let mut successful_connections = vec![];

        for Grpc { address, .. } in grpc.iter() {
            log::info!("Trying to connect to endpoint: {}", address);

            // get grpc endpoint
            let endpoint = Channel::builder(address.clone().try_into().unwrap());

            // try to connect to grpc endpoint
            let maybe_client = ServiceClient::connect(endpoint.clone()).await;

            // connection succeeded
            let mut client = if maybe_client.is_ok() {
                maybe_client?
            } else {
                log::warn!(
                    "Cannot connect to gRPC endpoint: {}, {:?}",
                    address,
                    maybe_client.unwrap_err()
                );

                // try HTTPS approach
                // https://github.com/hyperium/tonic/issues/363#issuecomment-638545965
                if !(address.contains("https") || address.contains("443")) {
                    continue;
                };

                log::info!("Attempting to connect with TLS");

                // re attempt to connect
                let endpoint = endpoint.clone().tls_config(ClientTlsConfig::new())?;
                let maybe_client = ServiceClient::connect(endpoint.clone()).await;

                // connection still fails
                if maybe_client.is_err() {
                    log::warn!(
                        "Cannot connect to gRPC endpoint: {}, {:?}",
                        address,
                        maybe_client.unwrap_err()
                    );
                    continue;
                };

                maybe_client?
            };

            // get client information for verification down below
            let node_info = client
                .get_node_info(GetNodeInfoRequest {})
                .await?
                .into_inner();

            // local juno does not return a proper ChainId with epoch format
            if ChainId::is_epoch_format(&node_info.default_node_info.as_ref().unwrap().network) {
                // verify we are connected to the spected network
                if node_info.default_node_info.as_ref().unwrap().network != chain_id.as_str() {
                    log::error!(
                        "Network mismatch: connection:{} != config:{}",
                        node_info.default_node_info.as_ref().unwrap().network,
                        chain_id.as_str()
                    );
                    continue;
                }
            }

            // add endpoint to succesful connections
            successful_connections.push(endpoint.connect().await?)
        }

        // we could not get any succesful connections
        if successful_connections.is_empty() {
            return Err(DaemonError::CannotConnectGRPC);
        }

        Ok(successful_connections.pop().unwrap())
    }
}

/// Indicate that the object has access to a gRPC channel
pub trait ChannelAccess {
    /// Returns the underlying gRPC channel object used to communicate with the remote server
    fn channel(&self) -> Channel;
}

impl ChannelAccess for Channel {
    fn channel(&self) -> tonic::transport::Channel {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    /*
        This test asserts breaking issues around the GRPC connection
    */

    use crate::DaemonAsync;
    use speculoos::prelude::*;

    #[tokio::test]
    async fn no_connection() {
        let mut chain = cw_orch_daemon::networks::LOCAL_JUNO;
        let grpcs = &vec!["https://127.0.0.1:99999"];
        chain.grpc_urls = grpcs;

        let build_res = DaemonAsync::builder()
            .chain(chain)
            .deployment_id("v0.1.0")
            .build()
            .await;

        asserting!("there is no GRPC connection")
            .that(&build_res.err().unwrap().to_string())
            .is_equal_to(String::from(
                "Can not connect to any grpc endpoint that was provided.",
            ))
    }

    #[tokio::test]
    async fn network_grpcs_list_is_empty() {
        let mut chain = cw_orch_daemon::networks::LOCAL_JUNO;
        let grpcs: &Vec<&str> = &vec![];
        chain.grpc_urls = grpcs;

        let build_res = DaemonAsync::builder()
            .chain(chain)
            .deployment_id("v0.1.0")
            .build()
            .await;

        asserting!("GRPC list is empty")
            .that(&build_res.err().unwrap().to_string())
            .is_equal_to(String::from("The list of grpc endpoints is empty"))
    }
}
