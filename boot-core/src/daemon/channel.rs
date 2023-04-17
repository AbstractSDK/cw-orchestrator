use cosmrs::proto::cosmos::base::tendermint::v1beta1::{
    service_client::ServiceClient, GetNodeInfoRequest,
};
use ibc_chain_registry::chain::Grpc;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use tonic::transport::{Channel, ClientTlsConfig};

use crate::DaemonError;

pub(crate) struct DaemonChannel {}

impl DaemonChannel {
    pub async fn new(grpc: &Vec<Grpc>, chain_id: &ChainId) -> Result<Option<Channel>, DaemonError> {
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

            // verify we are connected to the spected network
            if node_info.default_node_info.as_ref().unwrap().network != chain_id.as_str() {
                log::error!(
                    "Network mismatch: connection:{} != config:{}",
                    node_info.default_node_info.as_ref().unwrap().network,
                    chain_id.as_str()
                );
                continue;
            }

            // add endpoint to succesful connections
            successful_connections.push(endpoint.connect().await?)
        }

        // we could not get any succesful connections
        if successful_connections.is_empty() {
            return Err(DaemonError::CannotConnectGRPC);
        }

        Ok(Some(successful_connections[0].clone()))
    }
}
